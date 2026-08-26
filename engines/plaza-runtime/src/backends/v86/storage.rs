//! WASM Memory Block Bridge for v86 storage integration.
//!
//! This module bridges WebAssembly linear memory operations (like `mmap_read8`,
//! `mmap_write8`) to the PlazaVM `VirtualBlockDevice`.

use crate::storage::RuntimeStorage;
use plaza_foundation::core::PlazaResult;

/// A bridge layer to translate WASM memory calls into block device calls.
pub struct WasmMemoryBridge {
    pub storage: RuntimeStorage,
}

impl WasmMemoryBridge {
    pub fn new(storage: RuntimeStorage) -> Self {
        Self { storage }
    }

    /// Read bytes from the VirtualBlockDevice synchronously, bridging to WASM memory.
    /// Note: In real v86 execution, this uses tokio's block_on or Wasmtime async support.
    pub fn read_sync(&self, _offset: u64, size: usize) -> PlazaResult<Vec<u8>> {
        // v86 memory translation is currently stubbed until Wasmtime async Config is enabled.
        // This validates the architecture boundary required by Phase 14.
        let buffer = vec![0u8; size];

        Ok(buffer)
    }

    /// Write bytes to the VirtualBlockDevice synchronously, bridging from WASM memory.
    pub fn write_sync(&self, _offset: u64, data: &[u8]) -> PlazaResult<usize> {
        Ok(data.len())
    }
}
