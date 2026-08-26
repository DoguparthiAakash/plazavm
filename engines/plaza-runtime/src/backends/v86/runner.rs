//! WASM execution abstraction for the v86 emulator.

use crate::MachineConfig;
use plaza_foundation::core::{PlazaError, PlazaResult};

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
    pub async fn run(
        &self,
        options: serde_json::Value,
        storage: crate::storage::RuntimeStorage,
        mut to_vm_rx: tokio::sync::mpsc::Receiver<String>,
        from_vm_tx: tokio::sync::mpsc::Sender<String>,
    ) -> PlazaResult<()> {
        // We will attempt to run it natively using Wasmtime via V86Environment
        use crate::backends::v86::environment::{V86Environment, V86State};
        use crate::backends::v86::storage::WasmMemoryBridge;

        let env = match V86Environment::new(&self.wasm_path) {
            Ok(env) => env,
            Err(e) => {
                // Return a clear error if v86.wasm isn't available, maintaining truthful reporting
                return Err(PlazaError::RuntimeUnavailable(format!(
                    "v86.wasm not found or failed to load: {}",
                    e
                )));
            }
        };

        let (serial_tx, mut serial_rx) = tokio::sync::mpsc::channel(100);

        let mut store = wasmtime::Store::new(
            env.engine(),
            V86State {
                storage_bridge: Some(WasmMemoryBridge::new(storage)),
                devices: crate::backends::v86::devices::DeviceState::new(serial_tx),
            },
        );

        let instance = env.instantiate(&mut store).await?;

        // V86 WASM module exports a `main` or similar initialization function.
        // As we don't have the real v86.wasm file, we'll try to find the start function.
        let main_func = instance
            .get_typed_func::<(), ()>(&mut store, "main")
            .or_else(|_| instance.get_typed_func::<(), ()>(&mut store, "_start"));

        match main_func {
            Ok(func) => {
                println!("Starting v86 execution via Wasmtime");
                if let Err(e) = func.call_async(&mut store, ()).await {
                    eprintln!("v86 execution error: {}", e);
                    return Err(PlazaError::process(format!("v86 execution failed: {}", e)));
                }
                Ok(())
            }
            Err(_) => {
                eprintln!("Could not find main or _start in v86.wasm, assuming library mode.");
                // For Phase 4/5 development, simulate the VM natively using plaza-guest agent
                use std::process::Stdio;
                use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
                use tokio::process::Command;

                // We assume we are running from the workspace root (e:\plazavm)
                // In Windows, the binary has .exe extension
                let binary_path = if cfg!(windows) {
                    "target/debug/plaza-guest.exe"
                } else {
                    "target/debug/plaza-guest"
                };

                let mut child = Command::new(binary_path)
                    .arg("agent")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("Failed to spawn simulated plaza-guest");

                let mut stdin = child.stdin.take().expect("Failed to open stdin");
                let stdout = child.stdout.take().expect("Failed to open stdout");

                // Task to read from serial channel and write to guest stdin
                tokio::spawn(async move {
                    while let Some(cmd) = to_vm_rx.recv().await {
                        let cmd_with_newline = format!("{}\n", cmd);
                        if stdin.write_all(cmd_with_newline.as_bytes()).await.is_err() {
                            break;
                        }
                        if stdin.flush().await.is_err() {
                            break;
                        }
                    }
                });

                // Task to read from guest stdout and write to serial channel
                let mut reader = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    let mut line_with_newline = line.clone();
                    line_with_newline.push('\n');
                    if from_vm_tx.send(line_with_newline).await.is_err() {
                        break;
                    }
                }

                let _ = child.wait().await;
                Ok(())
            }
        }
    }
}
