//! Hyper-V runtime execution plugin for PlazaVM.

use async_trait::async_trait;
use plaza_foundation::core::types::{Architecture, HealthStatus, OperatingSystem, Timestamp};
use plaza_foundation::core::PlazaResult;

use crate::{
    RuntimeBackend, RuntimeCapabilities, RuntimeInstance, RuntimeMetrics, RuntimeStatus,
};

pub struct HyperVPlugin {
}

impl HyperVPlugin {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Default for HyperVPlugin {
    fn default() -> Self {
        Self::new()
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

    async fn create(&self, _machine: &crate::MachineConfig, _storage: crate::RuntimeStorage) -> PlazaResult<RuntimeInstance> {
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

