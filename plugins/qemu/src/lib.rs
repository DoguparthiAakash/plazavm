//! QEMU runtime execution plugin for PlazaVM.

use async_trait::async_trait;
use plaza_foundation::core::types::{Architecture, HealthStatus, OperatingSystem, Timestamp};
use plaza_foundation::core::{PlazaResult, PlazaError};
use plaza_plugin::{Plugin, PluginManifest, PluginType};
use plaza_runtime::{
    RuntimeBackend, RuntimeCapabilities, RuntimeInstance, RuntimeMetrics, RuntimeStatus,
};

pub mod adapter;
pub mod discovery;
pub mod process;
pub mod qmp;
pub mod storage;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use storage::nbd::NbdServer;
use crate::process::QemuProcess;


pub struct QemuPlugin {
    manifest: PluginManifest,
    instances: Arc<Mutex<HashMap<String, (RuntimeInstance, Option<plaza_runtime::RuntimeStorage>, Option<QemuProcess>)>>>,
}

impl QemuPlugin {
    pub fn new() -> Self {
        Self {
            manifest: PluginManifest {
                id: "qemu".into(),
                name: "QEMU Hypervisor Runtime".into(),
                version: semver::Version::new(0, 1, 0),
                description: "Full hardware emulation and virtualization backend using QEMU/QMP"
                    .into(),
                author: "PlazaVM Team".into(),
                license: Some("GPL-2.0".into()),
                plugin_type: PluginType::Runtime,
                min_plaza_version: None,
                dependencies: Vec::new(),
                capabilities: vec![
                    "multi_arch".into(),
                    "qmp".into(),
                    "snapshots".into(),
                    "vnc".into(),
                ],
                platforms: vec!["linux".into(), "windows".into(), "macos".into()],
            },
            instances: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for QemuPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for QemuPlugin {
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
impl RuntimeBackend for QemuPlugin {
    fn id(&self) -> &str {
        "qemu"
    }

    fn display_name(&self) -> &str {
        "QEMU Hypervisor"
    }

    fn capabilities(&self) -> RuntimeCapabilities {
        RuntimeCapabilities {
            supported_os: vec![
                OperatingSystem::Linux,
                OperatingSystem::Windows,
                OperatingSystem::FreeBSD,
                OperatingSystem::MacOS,
            ],
            supported_arch: vec![
                Architecture::X86_64,
                Architecture::Aarch64,
                Architecture::Riscv64,
                Architecture::Arm32,
            ],
            can_pause: true,
            can_snapshot: true,
            can_live_migrate: true,
            can_nested_virtualization: true,
            can_efi: true,
            supports_vnc: true,
            supports_spice: true,
            can_bridge_network: true,
            ..Default::default()
        }
    }

    async fn is_available(&self) -> bool {
        tokio::process::Command::new("qemu-system-x86_64")
            .arg("--version")
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    async fn version(&self) -> PlazaResult<String> {
        Ok("8.2.0".into())
    }

    async fn create(&self, _machine: &plaza_runtime::MachineConfig, storage: plaza_runtime::RuntimeStorage) -> PlazaResult<RuntimeInstance> {
        let instance_id = format!("qemu-{}", uuid::Uuid::new_v4());
        let instance = RuntimeInstance {
            id: instance_id.clone(),
            name: "qemu-vm".into(),
            status: RuntimeStatus::Stopped,
            created_at: Timestamp::now(),
        };
        
        self.instances.lock().await.insert(
            instance_id,
            (instance.clone(), Some(storage), None)
        );
        
        Ok(instance)
    }

    async fn start(&self, instance_id: &str) -> PlazaResult<()> {
        let mut instances = self.instances.lock().await;
        let entry = instances.get_mut(instance_id).ok_or_else(|| {
            PlazaError::process(format!("Instance {} not found", instance_id))
        })?;
        
        if entry.2.is_some() {
            return Ok(()); // Already running
        }
        
        let storage = entry.1.clone().ok_or_else(|| {
            PlazaError::process("No storage associated with instance")
        })?;
        
        // Setup NBD server socket path
        let socket_path = std::env::temp_dir().join(format!("plaza-nbd-{}.sock", instance_id));
        let nbd_server = NbdServer::new(socket_path.clone(), storage);
        
        // Spawn NBD server in background
        let server_id = instance_id.to_string();
        tokio::spawn(async move {
            if let Err(e) = nbd_server.run().await {
                eprintln!("NBD server for instance {} exited with error: {:?}", server_id, e);
            }
        });
        
        // Wait a bit for the Unix socket to be created
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Configure adapter
        let mut adapter = adapter::QemuAdapter::new(std::path::PathBuf::from("qemu-system-x86_64"));
        
        #[cfg(unix)]
        let drive_arg = format!("file=nbd+unix:///?socket={},format=raw,if=virtio", socket_path.display());
        
        #[cfg(windows)]
        let drive_arg = {
            let port_str = tokio::fs::read_to_string(&socket_path)
                .await
                .map_err(|e| plaza_foundation::core::PlazaError::process(format!("Failed to read NBD port: {}", e)))?;
            format!("file=nbd:127.0.0.1:{},format=raw,if=virtio", port_str)
        };

        // Add NBD drive parameter
        adapter.add_arg("-drive".into());
        adapter.add_arg(drive_arg.into());
        
        let process = QemuProcess::spawn(adapter, instance_id).await?;
        entry.2 = Some(process);
        entry.0.status = RuntimeStatus::Running;
        
        Ok(())
    }

    async fn stop(&self, instance_id: &str) -> PlazaResult<()> {
        let mut instances = self.instances.lock().await;
        let entry = instances.get_mut(instance_id).ok_or_else(|| {
            PlazaError::process(format!("Instance {} not found", instance_id))
        })?;
        
        if let Some(mut process) = entry.2.take() {
            process.stop().await?;
        }
        entry.0.status = RuntimeStatus::Stopped;
        
        Ok(())
    }

    async fn force_stop(&self, instance_id: &str) -> PlazaResult<()> {
        let mut instances = self.instances.lock().await;
        let entry = instances.get_mut(instance_id).ok_or_else(|| {
            PlazaError::process(format!("Instance {} not found", instance_id))
        })?;
        
        if let Some(mut process) = entry.2.take() {
            process.force_kill()?;
        }
        entry.0.status = RuntimeStatus::Stopped;
        Ok(())
    }

    async fn destroy(&self, instance_id: &str) -> PlazaResult<()> {
        let mut instances = self.instances.lock().await;
        if let Some(mut entry) = instances.remove(instance_id) {
            if let Some(mut process) = entry.2.take() {
                let _ = process.force_kill();
            }
        }
        Ok(())
    }

    async fn status(&self, instance_id: &str) -> PlazaResult<RuntimeStatus> {
        let instances = self.instances.lock().await;
        let entry = instances.get(instance_id).ok_or_else(|| {
            PlazaError::process(format!("Instance {} not found", instance_id))
        })?;
        Ok(entry.0.status)
    }

    async fn metrics(&self, _instance_id: &str) -> PlazaResult<RuntimeMetrics> {
        Ok(RuntimeMetrics::default())
    }
}

