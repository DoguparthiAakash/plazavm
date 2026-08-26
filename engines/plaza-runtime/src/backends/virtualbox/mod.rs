//! VirtualBox runtime execution plugin for PlazaVM.

use async_trait::async_trait;
use plaza_foundation::core::types::{Architecture, HealthStatus, OperatingSystem, Timestamp};
use plaza_foundation::core::PlazaResult;

use crate::{RuntimeBackend, RuntimeCapabilities, RuntimeInstance, RuntimeMetrics, RuntimeStatus};

pub struct VirtualBoxPlugin {}

impl VirtualBoxPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for VirtualBoxPlugin {
    fn default() -> Self {
        Self::new()
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

    async fn create(
        &self,
        _machine: &crate::MachineConfig,
        _storage: crate::RuntimeStorage,
    ) -> PlazaResult<RuntimeInstance> {
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
