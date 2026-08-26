//! Virtual block storage abstraction.
//!
//! Exposes layered, content-addressed block mappings as a contiguous, byte-addressable block device.
//! The guest sees one coherent disk — it must not know PlazaVM uses fragmented layers internally.

use plaza_foundation::core::{PlazaError, PlazaResult};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

/// The PlazaVM internal block/chunk granularity (4 KiB).
pub const BLOCK_SIZE: u64 = 4096;

/// Trait representing a synthesized virtual block device.
#[async_trait::async_trait]
pub trait VirtualBlockDevice: Send + Sync {
    /// Read raw bytes from the block device at the given offset.
    async fn read_at(&self, offset: u64, buffer: &mut [u8]) -> PlazaResult<usize>;

    /// Write raw bytes to the block device at the given offset.
    async fn write_at(&mut self, offset: u64, buffer: &[u8]) -> PlazaResult<usize>;

    /// Sync the block device to persistent storage.
    async fn flush(&mut self) -> PlazaResult<()>;

    /// Get the total size of the block device in bytes.
    fn size(&self) -> u64;
}

#[async_trait::async_trait]
impl VirtualBlockDevice for Box<dyn VirtualBlockDevice> {
    async fn read_at(&self, offset: u64, buffer: &mut [u8]) -> PlazaResult<usize> {
        (**self).read_at(offset, buffer).await
    }

    async fn write_at(&mut self, offset: u64, buffer: &[u8]) -> PlazaResult<usize> {
        (**self).write_at(offset, buffer).await
    }

    async fn flush(&mut self) -> PlazaResult<()> {
        (**self).flush().await
    }

    fn size(&self) -> u64 {
        (**self).size()
    }
}

/// A block device layer that is strictly read-only (backed by an immutable blob file).
#[async_trait::async_trait]
pub trait ImmutableLayer: Send + Sync {
    /// Read raw bytes from the layer at the given offset.
    /// Returns 0 if the offset is beyond this layer's size.
    async fn read_at(&self, offset: u64, buffer: &mut [u8]) -> PlazaResult<usize>;

    /// The logical byte-size of this layer's backing file.
    fn size(&self) -> u64;
}

/// A concrete ImmutableLayer backed by a file on the host filesystem.
pub struct FileBackedImmutableLayer {
    path: PathBuf,
    file_size: u64,
}

impl FileBackedImmutableLayer {
    pub async fn open(path: PathBuf) -> PlazaResult<Self> {
        let meta = tokio::fs::metadata(&path).await.map_err(PlazaError::Io)?;
        Ok(Self {
            path,
            file_size: meta.len(),
        })
    }
}

#[async_trait::async_trait]
impl ImmutableLayer for FileBackedImmutableLayer {
    async fn read_at(&self, offset: u64, buffer: &mut [u8]) -> PlazaResult<usize> {
        if offset >= self.file_size {
            return Ok(0);
        }
        let mut file = tokio::fs::File::open(&self.path)
            .await
            .map_err(PlazaError::Io)?;
        file.seek(std::io::SeekFrom::Start(offset))
            .await
            .map_err(PlazaError::Io)?;
        let max_read = std::cmp::min(buffer.len() as u64, self.file_size - offset) as usize;
        let n = file
            .read(&mut buffer[..max_read])
            .await
            .map_err(PlazaError::Io)?;
        Ok(n)
    }

    fn size(&self) -> u64 {
        self.file_size
    }
}

/// A VirtualBlockDevice that wraps an ImmutableLayer and rejects all writes.
pub struct ReadOnlyBlockDevice {
    layer: std::sync::Arc<dyn ImmutableLayer>,
}

impl ReadOnlyBlockDevice {
    pub fn new(layer: std::sync::Arc<dyn ImmutableLayer>) -> Self {
        Self { layer }
    }
}

#[async_trait::async_trait]
impl VirtualBlockDevice for ReadOnlyBlockDevice {
    async fn read_at(&self, offset: u64, buffer: &mut [u8]) -> PlazaResult<usize> {
        self.layer.read_at(offset, buffer).await
    }

    async fn write_at(&mut self, _offset: u64, _buffer: &[u8]) -> PlazaResult<usize> {
        Err(PlazaError::process(
            "Attempted to write to a read-only block device (SquashFS base image)",
        ))
    }

    async fn flush(&mut self) -> PlazaResult<()> {
        Ok(())
    }

    fn size(&self) -> u64 {
        self.layer.size()
    }
}

/// A copy-on-write writable layer backed by a sparse file.
///
/// Tracks which blocks have been materialized (written) using a block bitmap.
/// Reads return `Ok(0)` for blocks that haven't been COW'd yet, signalling to the
/// `LayeredBlockDevice` that it should fall through to lower layers.
pub struct CowWritableLayer {
    path: PathBuf,
    logical_size: u64,
    /// Tracks which block indices have been materialized in this writable layer.
    dirty_blocks: HashMap<u64, bool>,
    meta_path: PathBuf,
}

impl CowWritableLayer {
    pub async fn create(path: PathBuf, logical_size: u64) -> PlazaResult<Self> {
        let meta_path = path.with_extension("meta");
        let mut dirty_blocks = HashMap::new();

        // Create the file if it doesn't exist, or open it
        if !path.exists() {
            let file = tokio::fs::File::create(&path)
                .await
                .map_err(PlazaError::Io)?;
            // Set the file to the logical size (sparse file)
            file.set_len(logical_size).await.map_err(PlazaError::Io)?;
        } else if meta_path.exists() {
            // Load dirty blocks from meta file
            if let Ok(meta_bytes) = tokio::fs::read(&meta_path).await {
                if let Ok(loaded_blocks) = serde_json::from_slice::<Vec<u64>>(&meta_bytes) {
                    for block in loaded_blocks {
                        dirty_blocks.insert(block, true);
                    }
                }
            }
        }

        Ok(Self {
            path,
            logical_size,
            dirty_blocks,
            meta_path,
        })
    }

    /// Returns true if the given block index has been written to in this layer.
    pub fn has_block(&self, block_index: u64) -> bool {
        self.dirty_blocks
            .get(&block_index)
            .copied()
            .unwrap_or(false)
    }

    /// Returns the set of dirty block indices (for testing / inspection).
    pub fn dirty_block_indices(&self) -> Vec<u64> {
        self.dirty_blocks
            .iter()
            .filter_map(|(k, v)| if *v { Some(*k) } else { None })
            .collect()
    }
}

#[async_trait::async_trait]
impl VirtualBlockDevice for CowWritableLayer {
    async fn read_at(&self, offset: u64, buffer: &mut [u8]) -> PlazaResult<usize> {
        if offset >= self.logical_size {
            return Ok(0);
        }

        // Determine which block this read starts in
        let start_block = offset / BLOCK_SIZE;

        // If this block hasn't been COW'd, return 0 to signal "not here"
        if !self.has_block(start_block) {
            return Ok(0);
        }

        let mut file = tokio::fs::File::open(&self.path)
            .await
            .map_err(PlazaError::Io)?;
        file.seek(std::io::SeekFrom::Start(offset))
            .await
            .map_err(PlazaError::Io)?;
        let max_read = std::cmp::min(buffer.len() as u64, self.logical_size - offset) as usize;
        let n = file
            .read(&mut buffer[..max_read])
            .await
            .map_err(PlazaError::Io)?;
        Ok(n)
    }

    async fn write_at(&mut self, offset: u64, buffer: &[u8]) -> PlazaResult<usize> {
        if offset >= self.logical_size {
            return Err(PlazaError::OutOfBounds {
                offset,
                length: buffer.len() as u64,
                size: self.logical_size,
            });
        }

        let max_write = std::cmp::min(buffer.len() as u64, self.logical_size - offset) as usize;

        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .open(&self.path)
            .await
            .map_err(PlazaError::Io)?;
        file.seek(std::io::SeekFrom::Start(offset))
            .await
            .map_err(PlazaError::Io)?;
        file.write_all(&buffer[..max_write])
            .await
            .map_err(PlazaError::Io)?;

        // Mark all affected blocks as dirty
        let start_block = offset / BLOCK_SIZE;
        let end_block = (offset + max_write as u64).saturating_sub(1) / BLOCK_SIZE;
        for block_idx in start_block..=end_block {
            self.dirty_blocks.insert(block_idx, true);
        }

        Ok(max_write)
    }

    async fn flush(&mut self) -> PlazaResult<()> {
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .open(&self.path)
            .await
            .map_err(PlazaError::Io)?;
        file.flush().await.map_err(PlazaError::Io)?;
        file.sync_all().await.map_err(PlazaError::Io)?;

        // Write dirty blocks metadata atomically
        let dirty_list = self.dirty_block_indices();
        let meta_json = serde_json::to_string(&dirty_list)
            .map_err(|e| PlazaError::process(format!("Failed to serialize cow metadata: {}", e)))?;

        let temp_meta_path = self.meta_path.with_extension("meta.tmp");
        tokio::fs::write(&temp_meta_path, meta_json)
            .await
            .map_err(PlazaError::Io)?;
        tokio::fs::rename(&temp_meta_path, &self.meta_path)
            .await
            .map_err(PlazaError::Io)?;

        Ok(())
    }

    fn size(&self) -> u64 {
        self.logical_size
    }
}

/// A provider that creates a writable filesystem (e.g. ext4) image to be exposed via VirtualBlockDevice.
#[async_trait::async_trait]
pub trait WritableFilesystemProvider: Send + Sync {
    /// Create a writable filesystem image of the specified size.
    async fn create(&self, size: u64) -> Result<Box<dyn VirtualBlockDevice>, PlazaError>;
}

/// A placeholder implementation that always fails, used as a capability boundary.
pub struct UnavailableWritableFilesystemProvider;

#[async_trait::async_trait]
impl WritableFilesystemProvider for UnavailableWritableFilesystemProvider {
    async fn create(&self, _size: u64) -> Result<Box<dyn VirtualBlockDevice>, PlazaError> {
        Err(PlazaError::GuestWritableFilesystemUnavailable(
            "No pure-Rust filesystem creation library is available. PlazaVM requires a host-independent implementation.".to_string(),
        ))
    }
}

pub struct Ext4WritableFilesystemProvider {
    path: PathBuf,
    block_size: u64,
}

impl Ext4WritableFilesystemProvider {
    pub fn new(path: PathBuf, block_size: u64) -> Self {
        Self { path, block_size }
    }
}

#[async_trait::async_trait]
impl WritableFilesystemProvider for Ext4WritableFilesystemProvider {
    async fn create(&self, size: u64) -> Result<Box<dyn VirtualBlockDevice>, PlazaError> {
        let base_path = self.path.with_extension("ext4.base");

        // 1. Format the baseline cleanly if it doesn't exist
        if !base_path.exists() {
            // Using standard arcbox_ext4 API.
            let mut fmt = arcbox_ext4::Formatter::new(&base_path, self.block_size as u32, size)
                .map_err(|e| PlazaError::process(format!("Formatter err: {:?}", e)))?;
            fmt.create(
                "/workspace",
                0o755 | 0x4000,
                None,
                None,
                None,
                None,
                None,
                None,
            )
            .map_err(|e| PlazaError::process(format!("Workspace create err: {:?}", e)))?;
            fmt.create("/upper", 0o755 | 0x4000, None, None, None, None, None, None)
                .map_err(|e| PlazaError::process(format!("Upper create err: {:?}", e)))?;
            fmt.create("/work", 0o755 | 0x4000, None, None, None, None, None, None)
                .map_err(|e| PlazaError::process(format!("Work create err: {:?}", e)))?;
            fmt.close()
                .map_err(|e| PlazaError::process(format!("Close err: {:?}", e)))?;
        }

        let actual_size = std::fs::metadata(&base_path).map_err(PlazaError::Io)?.len();

        // 2. Wrap it as a strictly read-only ImmutableLayer
        let base_layer = std::sync::Arc::new(FileBackedImmutableLayer::open(base_path).await?)
            as std::sync::Arc<dyn ImmutableLayer>;

        // 3. Create the CowWritableLayer (starts with completely clean metadata)
        let cow_layer = CowWritableLayer::create(self.path.clone(), actual_size).await?;

        // 4. Return the composition
        Ok(Box::new(crate::composer::LayeredBlockDevice::new(
            vec![base_layer],
            cow_layer,
        )))
    }
}
