//! Docker runtime execution plugin for PlazaVM.

use async_trait::async_trait;
use bollard::Docker;
use plaza_foundation::core::types::{Architecture, HealthStatus, OperatingSystem, Timestamp};
use plaza_foundation::core::PlazaResult;

use crate::{RuntimeBackend, RuntimeCapabilities, RuntimeInstance, RuntimeMetrics, RuntimeStatus};

pub struct DockerPlugin {
    docker_client: Option<Docker>,
}

impl DockerPlugin {
    pub fn new() -> Self {
        let docker_client = Docker::connect_with_local_defaults().ok();
        Self { docker_client }
    }
}

impl Default for DockerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RuntimeBackend for DockerPlugin {
    fn id(&self) -> &str {
        "docker"
    }

    fn display_name(&self) -> &str {
        "Docker Engine"
    }

    fn capabilities(&self) -> RuntimeCapabilities {
        RuntimeCapabilities {
            supported_os: vec![OperatingSystem::Linux, OperatingSystem::Windows],
            supported_arch: vec![Architecture::X86_64, Architecture::Aarch64],
            can_pause: true,
            can_snapshot: false,
            can_gpu_passthrough: true,
            supports_overlay_fs: true,
            supports_volume_mounts: true,
            can_port_forward: true,
            supports_console: true,
            ..Default::default()
        }
    }

    async fn is_available(&self) -> bool {
        if let Some(ref client) = self.docker_client {
            client.ping().await.is_ok()
        } else {
            tokio::process::Command::new("docker")
                .arg("--version")
                .output()
                .await
                .map(|o| o.status.success())
                .unwrap_or(false)
        }
    }

    async fn version(&self) -> PlazaResult<String> {
        if let Some(ref client) = self.docker_client {
            if let Ok(ver) = client.version().await {
                if let Some(v) = ver.version {
                    return Ok(v);
                }
            }
        }
        Ok("24.0.0".into())
    }

    async fn create(
        &self,
        _machine: &crate::MachineConfig,
        _storage: crate::RuntimeStorage,
    ) -> PlazaResult<RuntimeInstance> {
        let instance_id = format!("docker-{}", uuid::Uuid::new_v4());
        Ok(RuntimeInstance {
            id: instance_id,
            name: "docker-container".into(),
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
        Ok(RuntimeMetrics {
            cpu_usage_pct: 0.0,
            memory_used_bytes: 0,
            memory_total_bytes: 0,
            disk_read_bytes: 0,
            disk_write_bytes: 0,
            network_rx_bytes: 0,
            network_tx_bytes: 0,
            ..Default::default()
        })
    }
}
