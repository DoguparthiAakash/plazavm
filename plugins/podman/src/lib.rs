//! Podman runtime execution plugin for PlazaVM.

use async_trait::async_trait;
use plaza_foundation::core::types::{Architecture, HealthStatus, OperatingSystem, Timestamp};
use plaza_foundation::core::PlazaResult;
use plaza_plugin::{Plugin, PluginManifest, PluginType};
use plaza_runtime::{
    RuntimeBackend, RuntimeCapabilities, RuntimeInstance, RuntimeMetrics, RuntimeStatus,
};

pub struct PodmanPlugin {
    manifest: PluginManifest,
}

impl PodmanPlugin {
    pub fn new() -> Self {
        Self {
            manifest: PluginManifest {
                id: "podman".into(),
                name: "Podman Container Runtime".into(),
                version: semver::Version::new(0, 1, 0),
                description: "Rootless OCI container execution backend using Podman".into(),
                author: "PlazaVM Team".into(),
                license: Some("Apache-2.0".into()),
                plugin_type: PluginType::Runtime,
                min_plaza_version: None,
                dependencies: Vec::new(),
                capabilities: vec!["rootless".into(), "container".into()],
                platforms: vec!["linux".into(), "windows".into(), "macos".into()],
            },
        }
    }
}

impl Default for PodmanPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for PodmanPlugin {
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
impl RuntimeBackend for PodmanPlugin {
    fn id(&self) -> &str {
        "podman"
    }

    fn display_name(&self) -> &str {
        "Podman"
    }

    fn capabilities(&self) -> RuntimeCapabilities {
        RuntimeCapabilities {
            supported_os: vec![OperatingSystem::Linux],
            supported_arch: vec![Architecture::X86_64, Architecture::Aarch64],
            can_pause: true,
            supports_rootless: true,
            supports_overlay_fs: true,
            supports_volume_mounts: true,
            can_port_forward: true,
            supports_console: true,
            ..Default::default()
        }
    }

    async fn is_available(&self) -> bool {
        tokio::process::Command::new("podman")
            .arg("--version")
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    async fn version(&self) -> PlazaResult<String> {
        Ok("4.9.0".into())
    }

    async fn create(&self, _spec_json: &str) -> PlazaResult<RuntimeInstance> {
        Ok(RuntimeInstance {
            id: format!("podman-{}", uuid::Uuid::new_v4()),
            name: "podman-container".into(),
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

