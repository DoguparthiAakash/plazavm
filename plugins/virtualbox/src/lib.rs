//! VirtualBox runtime execution plugin for PlazaVM.

use async_trait::async_trait;
use plaza_core::types::{Architecture, HealthStatus, OperatingSystem, Timestamp};
use plaza_core::PlazaResult;
use plaza_plugin::{Plugin, PluginManifest, PluginType};
use plaza_runtime::{
    RuntimeBackend, RuntimeCapabilities, RuntimeInstance, RuntimeMetrics, RuntimeStatus,
};

pub struct VirtualBoxPlugin {
    manifest: PluginManifest,
}

impl VirtualBoxPlugin {
    pub fn new() -> Self {
        Self {
            manifest: PluginManifest {
                id: "virtualbox".into(),
                name: "Oracle VirtualBox Runtime".into(),
                version: semver::Version::new(0, 1, 0),
                description: "Full VM execution backend using VBoxManage".into(),
                author: "PlazaVM Team".into(),
                license: Some("GPL-2.0".into()),
                plugin_type: PluginType::Runtime,
                min_plaza_version: None,
                dependencies: Vec::new(),
                capabilities: vec!["full_vm".into(), "snapshots".into(), "vnc".into()],
                platforms: vec!["windows".into(), "linux".into(), "macos".into()],
            },
        }
    }
}

impl Default for VirtualBoxPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for VirtualBoxPlugin {
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
impl RuntimeBackend for VirtualBoxPlugin {
    fn id(&self) -> &str {
        "virtualbox"
    }

    fn display_name(&self) -> &str {
        "Oracle VirtualBox"
    }

    fn capabilities(&self) -> RuntimeCapabilities {
        RuntimeCapabilities {
            supported_os: vec![
                OperatingSystem::Linux,
                OperatingSystem::Windows,
                OperatingSystem::FreeBSD,
            ],
            supported_arch: vec![Architecture::X86_64],
            can_pause: true,
            can_snapshot: true,
            can_usb_passthrough: true,
            can_tpm: true,
            can_efi: true,
            can_bridge_network: true,
            supports_vnc: true,
            ..Default::default()
        }
    }

    async fn is_available(&self) -> bool {
        tokio::process::Command::new("VBoxManage")
            .arg("--version")
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    async fn version(&self) -> PlazaResult<String> {
        Ok("7.0.0".into())
    }

    async fn create(&self, _spec_json: &str) -> PlazaResult<RuntimeInstance> {
        Ok(RuntimeInstance {
            id: format!("vbox-{}", uuid::Uuid::new_v4()),
            name: "virtualbox-vm".into(),
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

    async fn snapshot_create(&self, _instance_id: &str, _tag: &str) -> PlazaResult<()> {
        Ok(())
    }
}
