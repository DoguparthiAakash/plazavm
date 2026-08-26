//! v86 runtime execution plugin for PlazaVM.
//!
//! Provides a WebAssembly-based x86 virtualization backend using the `v86` project.
//! Unlike QEMU, this backend executes wholly within software emulation or WASM engines,
//! without KVM/hardware assistance.

use async_trait::async_trait;
use plaza_foundation::core::types::{Architecture, HealthStatus, OperatingSystem, Timestamp};
use plaza_foundation::core::PlazaResult;

use crate::{RuntimeBackend, RuntimeCapabilities, RuntimeInstance, RuntimeMetrics, RuntimeStatus};

pub mod devices;
pub mod environment;
#[cfg(test)]
pub mod imports_test;
pub mod runner;
pub mod serial;
pub mod storage;

use crate::backends::v86::runner::V86Runner;
use crate::storage::RuntimeStorage;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct V86InstanceState {
    pub instance: RuntimeInstance,
    pub storage: RuntimeStorage,
    pub config: crate::MachineConfig,
    pub serial_tx: Option<tokio::sync::mpsc::Sender<String>>,
    pub serial_rx: Option<Arc<Mutex<tokio::sync::mpsc::Receiver<String>>>>,
}

pub struct V86Plugin {
    instances: Arc<Mutex<HashMap<String, V86InstanceState>>>,
}

impl V86Plugin {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for V86Plugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RuntimeBackend for V86Plugin {
    fn id(&self) -> &str {
        "v86"
    }

    fn display_name(&self) -> &str {
        "v86 Emulator (x86)"
    }

    fn capabilities(&self) -> RuntimeCapabilities {
        RuntimeCapabilities {
            supported_os: vec![OperatingSystem::Linux, OperatingSystem::FreeBSD],
            // v86 ONLY supports 32-bit x86 architecture.
            supported_arch: vec![Architecture::X86_32],
            can_pause: true,
            can_snapshot: true,
            can_live_migrate: false,
            can_nested_virtualization: false,
            can_efi: false, // v86 typically uses seabios
            supports_vnc: false,
            supports_spice: false,
            can_bridge_network: true, // Requires WebSocket/WebRTC proxying
            custom: [("persistent_storage".into(), serde_json::Value::Bool(false))]
                .into_iter()
                .collect(),
            ..Default::default()
        }
    }

    async fn is_available(&self) -> bool {
        // Checking availability might involve looking for Node.js or a standalone WASM runner.
        // For PlazaVM, we will eventually bundle a WASMTIME or similar runner.
        // For Phase 6 stub, we assume true if WASM runner logic compiles.
        true
    }

    async fn version(&self) -> PlazaResult<String> {
        Ok("v86-latest".into())
    }

    async fn create(
        &self,
        machine: &crate::MachineConfig,
        storage: crate::RuntimeStorage,
    ) -> PlazaResult<RuntimeInstance> {
        let instance = RuntimeInstance {
            id: format!("v86-{}", uuid::Uuid::new_v4()),
            name: "v86-vm".into(),
            status: RuntimeStatus::Stopped,
            created_at: Timestamp::now(),
        };

        let mut instances = self.instances.lock().await;
        instances.insert(
            instance.id.clone(),
            V86InstanceState {
                instance: instance.clone(),
                storage,
                config: machine.clone(),
                serial_tx: None,
                serial_rx: None,
            },
        );

        Ok(instance)
    }

    async fn start(&self, instance_id: &str) -> PlazaResult<()> {
        let mut instances = self.instances.lock().await;
        if let Some(state) = instances.get_mut(instance_id) {
            state.instance.status = RuntimeStatus::Starting;

            // Assume v86.wasm is at some path or fallback
            let current_dir = std::env::current_dir().unwrap_or_default();
            let wasm_path = std::env::var("V86_WASM_PATH").unwrap_or_else(|_| {
                current_dir
                    .join("v86_investigation/v86.wasm")
                    .to_string_lossy()
                    .into_owned()
            });
            let bios_path = std::env::var("V86_BIOS_PATH").unwrap_or_else(|_| {
                current_dir
                    .join("v86_investigation/seabios.bin")
                    .to_string_lossy()
                    .into_owned()
            });

            let runner = V86Runner::new(wasm_path.into(), bios_path.into());
            let options = runner.map_config(&state.config)?;

            let (to_vm_tx, to_vm_rx) = tokio::sync::mpsc::channel::<String>(100);
            let (from_vm_tx, from_vm_rx) = tokio::sync::mpsc::channel::<String>(100);

            state.serial_tx = Some(to_vm_tx);
            state.serial_rx = Some(Arc::new(Mutex::new(from_vm_rx)));

            let storage = state.storage.clone();

            state.instance.status = RuntimeStatus::Running;

            tokio::spawn(async move {
                let run_result = runner.run(options, storage, to_vm_rx, from_vm_tx).await;

                match run_result {
                    Ok(_) => {
                        println!("v86 runner finished successfully");
                    }
                    Err(e) => {
                        eprintln!("v86 runner failed: {}", e);
                    }
                }
            });

            Ok(())
        } else {
            Err(plaza_foundation::core::PlazaError::process(format!(
                "Instance not found: {}",
                instance_id
            )))
        }
    }

    async fn stop(&self, instance_id: &str) -> PlazaResult<()> {
        let mut instances = self.instances.lock().await;
        if let Some(state) = instances.get_mut(instance_id) {
            state.instance.status = RuntimeStatus::Stopped;
        }
        Ok(())
    }

    async fn force_stop(&self, instance_id: &str) -> PlazaResult<()> {
        let mut instances = self.instances.lock().await;
        if let Some(state) = instances.get_mut(instance_id) {
            state.instance.status = RuntimeStatus::Stopped;
        }
        Ok(())
    }

    async fn destroy(&self, instance_id: &str) -> PlazaResult<()> {
        let mut instances = self.instances.lock().await;
        instances.remove(instance_id);
        Ok(())
    }

    async fn status(&self, instance_id: &str) -> PlazaResult<RuntimeStatus> {
        let instances = self.instances.lock().await;
        if let Some(state) = instances.get(instance_id) {
            Ok(state.instance.status.clone())
        } else {
            Ok(RuntimeStatus::Stopped)
        }
    }

    async fn metrics(&self, _instance_id: &str) -> PlazaResult<RuntimeMetrics> {
        Ok(RuntimeMetrics {
            execution_mode: Some("wasmtime".into()),
            storage_backend: Some("wasm_memory_bridge".into()),
            ..Default::default()
        })
    }

    async fn exec(&self, instance_id: &str, cmd: &str) -> PlazaResult<()> {
        let (tx, rx) = {
            let instances = self.instances.lock().await;
            if let Some(state) = instances.get(instance_id) {
                if let (Some(tx), Some(rx)) = (&state.serial_tx, &state.serial_rx) {
                    (tx.clone(), rx.clone())
                } else {
                    return Err(plaza_foundation::core::PlazaError::process(
                        "Instance is not running or has no serial console".to_string(),
                    ));
                }
            } else {
                return Err(plaza_foundation::core::PlazaError::process(format!(
                    "Instance not found: {}",
                    instance_id
                )));
            }
        };

        // Send the command
        if let Err(_) = tx.send(cmd.to_string()).await {
            return Err(plaza_foundation::core::PlazaError::process(
                "Failed to send command to guest serial port".to_string(),
            ));
        }

        // Wait for output
        let mut rx = rx.lock().await;
        loop {
            match rx.recv().await {
                Some(line) => {
                    if line.starts_with("EXIT_CODE:") {
                        let code = line.trim_start_matches("EXIT_CODE:").trim();
                        if code == "0" {
                            return Ok(());
                        } else {
                            return Err(plaza_foundation::core::PlazaError::process(format!(
                                "Command failed with exit code: {}",
                                code
                            )));
                        }
                    } else if line.starts_with("SUCCESS_PLAZA_GUEST_READY") {
                        // ignore ready marker
                    } else {
                        // Print the output of the command
                        print!("{}", line);
                    }
                }
                None => {
                    return Err(plaza_foundation::core::PlazaError::process(
                        "Guest serial port disconnected unexpectedly".to_string(),
                    ));
                }
            }
        }
    }
}
