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
        let n = file.read(&mut buffer[..max_read]).await.map_err(PlazaError::Io)?;
        Ok(n)
    }

    fn size(&self) -> u64 {
        self.file_size
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
}

impl CowWritableLayer {
    /// Create or open a COW writable layer file with the given logical disk size.
    pub async fn create(path: PathBuf, logical_size: u64) -> PlazaResult<Self> {
        // Create the file if it doesn't exist, or open it
        if !path.exists() {
            let file = tokio::fs::File::create(&path)
                .await
                .map_err(PlazaError::Io)?;
            // Set the file to the logical size (sparse file)
            file.set_len(logical_size).await.map_err(PlazaError::Io)?;
        }
        Ok(Self {
            path,
            logical_size,
            dirty_blocks: HashMap::new(),
        })
    }

    /// Returns true if the given block index has been written to in this layer.
    pub fn has_block(&self, block_index: u64) -> bool {
        self.dirty_blocks.get(&block_index).copied().unwrap_or(false)
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
        let n = file.read(&mut buffer[..max_read]).await.map_err(PlazaError::Io)?;
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
        Ok(())
    }

    fn size(&self) -> u64 {
        self.logical_size
    }
}
