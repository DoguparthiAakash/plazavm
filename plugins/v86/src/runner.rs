//! WASM execution abstraction for the v86 emulator.

use plaza_foundation::core::{PlazaError, PlazaResult};
use plaza_runtime::MachineConfig;

/// WASM Runner abstraction.
///
/// In a real implementation, this would spin up a headless JavaScript engine 
/// (like V8 via Deno/Node) or a native WebAssembly runtime (like Wasmtime/Wasmer) 
/// to execute `v86.wasm`.
pub struct V86Runner {
    wasm_path: std::path::PathBuf,
    bios_path: std::path::PathBuf,
}

impl V86Runner {
    /// Create a new v86 runner.
    pub fn new(wasm_path: std::path::PathBuf, bios_path: std::path::PathBuf) -> Self {
        Self {
            wasm_path,
            bios_path,
        }
    }

    /// Map `MachineConfig` to v86 initialization parameters.
    pub fn map_config(&self, config: &MachineConfig) -> PlazaResult<serde_json::Value> {
        // Architecture Check
        if config.machine.architecture != plaza_foundation::core::types::Architecture::X86_32 {
            return Err(PlazaError::config(format!(
                "v86 emulator only supports 32-bit x86 architecture. Requested: {}",
                config.machine.architecture
            )));
        }

        let mem_mb = plaza_foundation::core::types::ByteSize::parse(&config.machine.memory.size)
            .unwrap_or_else(|_| plaza_foundation::core::types::ByteSize::from_mb(128))
            .as_mb();

        // Build v86 startup options structure
        let options = serde_json::json!({
            "wasm_path": self.wasm_path.to_string_lossy(),
            "memory_size": mem_mb * 1024 * 1024,
            "vga_memory_size": 8 * 1024 * 1024, // 8MB VGA
            "bios": {
                "url": self.bios_path.to_string_lossy(),
            },
            // hda handles the virtual block device from the Composer
            "hda": {
                "url": config.boot_device.to_string_lossy(),
                "async": true
            },
            "network_relay_url": if config.capabilities.network.enabled {
                "wss://relay.plazavm.local/"
            } else {
                ""
            },
            "autostart": true,
        });

        Ok(options)
    }

    /// Spawn the WASM execution engine asynchronously.
    pub async fn run(&self, _options: serde_json::Value) -> PlazaResult<()> {
        // Implementation stub for starting the WASM runner.
        Ok(())
    }
}
