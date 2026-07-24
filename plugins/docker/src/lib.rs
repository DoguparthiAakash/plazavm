//! Docker runtime execution plugin for PlazaVM.

use async_trait::async_trait;
use plaza_core::types::{Architecture, HealthStatus, OperatingSystem, Timestamp};
use plaza_core::PlazaResult;
use plaza_plugin::{Plugin, PluginManifest, PluginType};
use plaza_runtime::{
    RuntimeBackend, RuntimeCapabilities, RuntimeInstance, RuntimeMetrics, RuntimeStatus,
};

pub struct DockerPlugin {
    manifest: PluginManifest,
}

impl DockerPlugin {
    pub fn new() -> Self {
        Self {
            manifest: PluginManifest {
                id: "docker".into(),
                name: "Docker Engine Runtime".into(),
                version: semver::Version::new(0, 1, 0),
                description: "OCI container execution backend using Docker Engine".into(),
                author: "PlazaVM Team".into(),
                license: Some("MIT".into()),
                plugin_type: PluginType::Runtime,
                min_plaza_version: None,
                dependencies: Vec::new(),
                capabilities: vec!["container".into(), "gpu".into(), "overlay_fs".into()],
                platforms: vec!["linux".into(), "windows".into(), "macos".into()],
            },
        }
    }
}

impl Default for DockerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for DockerPlugin {
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
        tokio::process::Command::new("docker")
            .arg("--version")
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    async fn version(&self) -> PlazaResult<String> {
        Ok("24.0.0".into())
    }

    async fn create(&self, _spec_json: &str) -> PlazaResult<RuntimeInstance> {
        Ok(RuntimeInstance {
            id: format!("docker-{}", uuid::Uuid::new_v4()),
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
        Ok(RuntimeMetrics::default())
    }
}
