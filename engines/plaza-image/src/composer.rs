//! Composes multiple immutable layers and one writable COW layer into a single VirtualBlockDevice.
//!
//! Read resolution order:
//!   1. Writable layer (if block is dirty)
//!   2. Immutable layers in reverse order (highest-priority last-added wins)
//!   3. Zero-fill (unallocated block)
//!
//! Write routing:
//!   All writes go to the writable layer. If the target block hasn't been COW'd yet,
//!   the data from the highest immutable layer is first read, then the partial write
//!   is applied on top (read-modify-write for sub-block writes).

use crate::block::{VirtualBlockDevice, ImmutableLayer, CowWritableLayer, BLOCK_SIZE};
use plaza_foundation::core::{PlazaError, PlazaResult};
use std::sync::Arc;

pub struct LayeredBlockDevice {
    /// Immutable layers ordered from base (index 0) to top (last index).
    immutable_layers: Vec<Arc<dyn ImmutableLayer>>,
    /// The single writable COW overlay layer.
    writable_layer: CowWritableLayer,
    /// The logical size of the composed virtual disk.
    logical_size: u64,
}

impl LayeredBlockDevice {
    pub fn new(
        immutable_layers: Vec<Arc<dyn ImmutableLayer>>,
        writable_layer: CowWritableLayer,
    ) -> Self {
        // Logical size = max of all layer sizes
        let mut logical_size = writable_layer.size();
        for layer in &immutable_layers {
            let s = layer.size();
            if s > logical_size {
                logical_size = s;
            }
        }

        Self {
            immutable_layers,
            writable_layer,
            logical_size,
        }
    }

    /// Read a block from the highest-priority immutable layer that contains it.
    async fn read_from_immutable(&self, offset: u64, buffer: &mut [u8]) -> PlazaResult<usize> {
        // Try layers in reverse (highest priority first)
        for layer in self.immutable_layers.iter().rev() {
            if offset < layer.size() {
                let n = layer.read_at(offset, buffer).await?;
                if n > 0 {
                    return Ok(n);
                }
            }
        }
        // No layer had this data — zero fill
        buffer.fill(0);
        Ok(buffer.len())
    }
}

#[async_trait::async_trait]
impl VirtualBlockDevice for LayeredBlockDevice {
    async fn read_at(&self, offset: u64, buffer: &mut [u8]) -> PlazaResult<usize> {
        if offset >= self.logical_size {
            return Err(PlazaError::OutOfBounds {
                offset,
                length: buffer.len() as u64,
                size: self.logical_size,
            });
        }

        let length = std::cmp::min(buffer.len() as u64, self.logical_size - offset) as usize;
        let buf = &mut buffer[..length];

        // 1. Try writable layer first (returns 0 if the block isn't dirty)
        let n = self.writable_layer.read_at(offset, buf).await?;
        if n == length {
            return Ok(n);
        }

        // 2. Fall through to immutable layers
        self.read_from_immutable(offset, buf).await
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

        // Determine block boundaries for COW read-modify-write
        let start_block = offset / BLOCK_SIZE;
        let end_block = (offset + max_write as u64).saturating_sub(1) / BLOCK_SIZE;

        // For each block that hasn't been materialized yet, read-modify-write
        for block_idx in start_block..=end_block {
            if !self.writable_layer.has_block(block_idx) {
                // Read the full block from immutable layers
                let block_offset = block_idx * BLOCK_SIZE;
                let mut block_buf = vec![0u8; BLOCK_SIZE as usize];
                let readable_len = std::cmp::min(
                    BLOCK_SIZE,
                    self.logical_size.saturating_sub(block_offset),
                ) as usize;
                self.read_from_immutable(block_offset, &mut block_buf[..readable_len])
                    .await?;
                // Write the full block to the writable layer to materialize it
                self.writable_layer
                    .write_at(block_offset, &block_buf[..readable_len])
                    .await?;
            }
        }

        // Now write the actual data
        self.writable_layer.write_at(offset, &buffer[..max_write]).await
    }

    async fn flush(&mut self) -> PlazaResult<()> {
        self.writable_layer.flush().await
    }

    fn size(&self) -> u64 {
        self.logical_size
    }
}
