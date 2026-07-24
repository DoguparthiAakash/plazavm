//! Hyper-V runtime execution plugin for PlazaVM.

use async_trait::async_trait;
use plaza_core::types::{Architecture, HealthStatus, OperatingSystem, Timestamp};
use plaza_core::PlazaResult;
use plaza_plugin::{Plugin, PluginManifest, PluginType};
use plaza_runtime::{
    RuntimeBackend, RuntimeCapabilities, RuntimeInstance, RuntimeMetrics, RuntimeStatus,
};

pub struct HyperVPlugin {
    manifest: PluginManifest,
}

impl HyperVPlugin {
    pub fn new() -> Self {
        Self {
            manifest: PluginManifest {
                id: "hyperv".into(),
                name: "Windows Hyper-V Runtime".into(),
                version: semver::Version::new(0, 1, 0),
                description: "Native Windows hypervisor backend using PowerShell cmdlets".into(),
                author: "PlazaVM Team".into(),
                license: Some("MIT".into()),
                plugin_type: PluginType::Runtime,
                min_plaza_version: None,
                dependencies: Vec::new(),
                capabilities: vec!["hyperv".into(), "snapshots".into(), "vhdx".into()],
                platforms: vec!["windows".into()],
            },
        }
    }
}

impl Default for HyperVPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for HyperVPlugin {
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
impl RuntimeBackend for HyperVPlugin {
    fn id(&self) -> &str {
        "hyperv"
    }

    fn display_name(&self) -> &str {
        "Windows Hyper-V"
    }

    fn capabilities(&self) -> RuntimeCapabilities {
        RuntimeCapabilities {
            supported_os: vec![OperatingSystem::Windows, OperatingSystem::Linux],
            supported_arch: vec![Architecture::X86_64],
            can_pause: true,
            can_snapshot: true,
            can_nested_virtualization: true,
            can_efi: true,
            can_bridge_network: true,
            ..Default::default()
        }
    }

    async fn is_available(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            tokio::process::Command::new("powershell")
                .args(["-Command", "Get-VM"])
                .output()
                .await
                .map(|o| o.status.success())
                .unwrap_or(false)
        }
        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    }

    async fn version(&self) -> PlazaResult<String> {
        Ok("10.0".into())
    }

    async fn create(&self, _spec_json: &str) -> PlazaResult<RuntimeInstance> {
        Ok(RuntimeInstance {
            id: format!("hyperv-{}", uuid::Uuid::new_v4()),
            name: "hyperv-vm".into(),
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
