//! v86 runtime execution plugin for PlazaVM.
//!
//! Provides a WebAssembly-based x86 virtualization backend using the `v86` project.
//! Unlike QEMU, this backend executes wholly within software emulation or WASM engines,
//! without KVM/hardware assistance.

use async_trait::async_trait;
use plaza_foundation::core::types::{Architecture, HealthStatus, OperatingSystem, Timestamp};
use plaza_foundation::core::PlazaResult;
use plaza_plugin::{Plugin, PluginManifest, PluginType};
use plaza_runtime::{
    RuntimeBackend, RuntimeCapabilities, RuntimeInstance, RuntimeMetrics, RuntimeStatus,
};

pub mod environment;
pub mod runner;
pub mod storage;
#[cfg(test)]
pub mod imports_test;

use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use plaza_runtime::storage::RuntimeStorage;
use crate::runner::V86Runner;

pub struct V86InstanceState {
    pub instance: RuntimeInstance,
    pub storage: RuntimeStorage,
    pub config: plaza_runtime::MachineConfig,
}

pub struct V86Plugin {
    manifest: PluginManifest,
    instances: Arc<Mutex<HashMap<String, V86InstanceState>>>,
}

impl V86Plugin {
    pub fn new() -> Self {
        Self {
            manifest: PluginManifest {
                id: "v86".into(),
                name: "v86 WebAssembly Emulator".into(),
                version: semver::Version::new(0, 1, 0),
                description: "x86 architecture emulation using v86 and WASM".into(),
                author: "PlazaVM Team".into(),
                license: Some("MIT".into()),
                plugin_type: PluginType::Runtime,
                min_plaza_version: None,
                dependencies: Vec::new(),
                capabilities: vec![
                    "x86_only".into(),
                    "snapshots".into(),
                ],
                platforms: vec!["linux".into(), "windows".into(), "macos".into()],
            },
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
impl Plugin for V86Plugin {
    fn manifest(&self) -> &PluginManifest {
        &self.manifest
    }

    async fn init(&mut self) -> PlazaResult<()> {
        Ok(())
    }

    async fn shutdown(&mut self) -> PlazaResult<()> {
        Ok(())
    }

    fn health(&self) -> HealthStatus {
        HealthStatus::Healthy
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
            supported_os: vec![
                OperatingSystem::Linux,
                OperatingSystem::FreeBSD,
            ],
            // v86 ONLY supports 32-bit x86 architecture.
            supported_arch: vec![
                Architecture::X86_32,
            ],
            can_pause: true,
            can_snapshot: true,
            can_live_migrate: false,
            can_nested_virtualization: false,
            can_efi: false, // v86 typically uses seabios
            supports_vnc: false,
            supports_spice: false,
            can_bridge_network: true, // Requires WebSocket/WebRTC proxying
            custom: [("persistent_storage".into(), serde_json::Value::Bool(false))].into_iter().collect(),
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

    async fn create(&self, machine: &plaza_runtime::MachineConfig, storage: plaza_runtime::RuntimeStorage) -> PlazaResult<RuntimeInstance> {
        let instance = RuntimeInstance {
            id: format!("v86-{}", uuid::Uuid::new_v4()),
            name: "v86-vm".into(),
            status: RuntimeStatus::Stopped,
            created_at: Timestamp::now(),
        };
        
        let mut instances = self.instances.lock().await;
        instances.insert(instance.id.clone(), V86InstanceState {
            instance: instance.clone(),
            storage,
            config: machine.clone(),
        });

        Ok(instance)
    }

    async fn start(&self, instance_id: &str) -> PlazaResult<()> {
        let mut instances = self.instances.lock().await;
        if let Some(state) = instances.get_mut(instance_id) {
            state.instance.status = RuntimeStatus::Starting;
            
            // Assume v86.wasm is at some path or fallback
            let wasm_path = std::env::var("V86_WASM_PATH").unwrap_or_else(|_| "/opt/plaza/v86.wasm".into());
            let bios_path = std::env::var("V86_BIOS_PATH").unwrap_or_else(|_| "/opt/plaza/seabios.bin".into());
            
            let runner = V86Runner::new(wasm_path.into(), bios_path.into());
            let options = runner.map_config(&state.config)?;
            
            let storage = state.storage.clone();
            
            // For now, run() will likely fail if v86.wasm doesn't exist.
            // But we simulate advancing towards execution by attempting it.
            let run_result = runner.run(options, storage).await;
            
            match run_result {
                Ok(_) => {
                    state.instance.status = RuntimeStatus::Running;
                    Ok(())
                }
                Err(e) => {
                    state.instance.status = RuntimeStatus::Error;
                    Err(e)
                }
            }
        } else {
            Err(plaza_foundation::core::PlazaError::process(format!("Instance not found: {}", instance_id)))
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
}
