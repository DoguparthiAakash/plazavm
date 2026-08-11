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
#[cfg(test)]
pub mod imports_test;

pub struct V86Plugin {
    manifest: PluginManifest,
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

    async fn create(&self, _machine: &plaza_runtime::MachineConfig, _storage: plaza_runtime::RuntimeStorage) -> PlazaResult<RuntimeInstance> {
        Ok(RuntimeInstance {
            id: format!("v86-{}", uuid::Uuid::new_v4()),
            name: "v86-vm".into(),
            status: RuntimeStatus::Stopped,
            created_at: Timestamp::now(),
        })
    }

    async fn start(&self, _instance_id: &str) -> PlazaResult<()> {
        Ok(())
    }

    async fn stop(&self, _instance_id: &str) -> PlazaResult<()> {
        Ok(())
    }

    async fn force_stop(&self, _instance_id: &str) -> PlazaResult<()> {
        Ok(())
    }

    async fn destroy(&self, _instance_id: &str) -> PlazaResult<()> {
        Ok(())
    }

    async fn status(&self, _instance_id: &str) -> PlazaResult<RuntimeStatus> {
        Ok(RuntimeStatus::Running)
    }

    async fn metrics(&self, _instance_id: &str) -> PlazaResult<RuntimeMetrics> {
        Ok(RuntimeMetrics::default())
    }
}
