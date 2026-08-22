//! QEMU runtime execution plugin for PlazaVM.

use async_trait::async_trait;
use plaza_foundation::core::types::{Architecture, HealthStatus, OperatingSystem, Timestamp};
use plaza_foundation::core::{PlazaResult, PlazaError};

use crate::{
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
use crate::backends::qemu::process::QemuProcess;


pub struct QemuPlugin {
    #[allow(clippy::type_complexity)]
    instances: Arc<Mutex<HashMap<String, (RuntimeInstance, Option<crate::RuntimeStorage>, Option<QemuProcess>, Option<crate::MachineConfig>, Option<tokio::task::JoinHandle<()>>)>>>,
}

impl QemuPlugin {
    pub fn new() -> Self {
        Self {
            instances: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Wait for the guest to signal readiness via serial output.
    pub async fn wait_for_ready(&self, instance_id: &str, timeout: std::time::Duration) -> PlazaResult<()> {
        let mut instances = self.instances.lock().await;
        if let Some(entry) = instances.get_mut(instance_id) {
            if let Some(process) = entry.2.as_mut() {
                return process.wait_for_ready(timeout).await;
            }
        }
        Err(PlazaError::process(format!("Instance {} not found or not running", instance_id)))
    }

    /// Send a command to the guest's serial console.
    pub async fn send_command(&self, instance_id: &str, command: &str) -> PlazaResult<()> {
        let mut instances = self.instances.lock().await;
        if let Some(entry) = instances.get_mut(instance_id) {
            if let Some(process) = entry.2.as_mut() {
                return process.send_command(command).await;
            }
        }
        Err(PlazaError::process(format!("Instance {} not found or not running", instance_id)))
    }
}

impl Default for QemuPlugin {
    fn default() -> Self {
        Self::new()
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
        let qemu_bin = if cfg!(windows) {
            "plaza-qemu.exe"
        } else {
            "plaza-qemu"
        };
        let qemu_path = std::env::current_exe()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join(qemu_bin);
        let qemu_bin_str = qemu_path.to_str().unwrap_or(qemu_bin);
        tokio::process::Command::new(qemu_bin_str)
            .arg("--version")
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    async fn version(&self) -> PlazaResult<String> {
        Ok("8.2.0".into())
    }

    async fn create(&self, _machine: &crate::MachineConfig, storage: crate::RuntimeStorage) -> PlazaResult<RuntimeInstance> {
        let instance_id = format!("qemu-{}", uuid::Uuid::new_v4());
        let instance = RuntimeInstance {
            id: instance_id.clone(),
            name: "qemu-vm".into(),
            status: RuntimeStatus::Stopped,
            created_at: Timestamp::now(),
        };
        
        self.instances.lock().await.insert(
            instance_id,
            (instance.clone(), Some(storage), None, Some(_machine.clone()), None)
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
        
        // Setup NBD server socket path for base image
        let socket_path = std::env::temp_dir().join(format!("plaza-nbd-{}.sock", instance_id));
        let base_storage = crate::RuntimeStorage {
            device: storage.device.clone(),
            workspace_device: None,
        };
        let nbd_server = NbdServer::new(socket_path.clone(), base_storage);
        
        // Spawn NBD server in background
        let server_id = instance_id.to_string();
        let nbd_handle = tokio::spawn(async move {
            if let Err(e) = nbd_server.run().await {
                eprintln!("NBD server for instance {} exited with error: {:?}", server_id, e);
            }
        });
        entry.4 = Some(nbd_handle);
        
        // Check for secondary workspace device
        let mut workspace_socket_path = None;
        if let Some(workspace_device) = &storage.workspace_device {
            let ws_socket_path = std::env::temp_dir().join(format!("plaza-nbd-ws-{}.sock", instance_id));
            let ws_storage = crate::RuntimeStorage {
                device: workspace_device.clone(),
                workspace_device: None,
            };
            let ws_nbd_server = NbdServer::new(ws_socket_path.clone(), ws_storage);
            
            let ws_server_id = instance_id.to_string();
            tokio::spawn(async move {
                if let Err(e) = ws_nbd_server.run().await {
                    eprintln!("NBD workspace server for instance {} exited with error: {:?}", ws_server_id, e);
                }
            });
            workspace_socket_path = Some(ws_socket_path);
        }

        // Wait a bit for the Unix sockets to be created
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        let config = entry.3.clone().ok_or_else(|| {
            PlazaError::process("No machine config associated with instance")
        })?;

        // Configure adapter with machine config
        let qemu_bin = if cfg!(windows) {
            "plaza-qemu.exe"
        } else {
            "plaza-qemu"
        };
        let qemu_path = std::env::current_exe()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join(qemu_bin);
        let mut adapter = adapter::QemuAdapter::new(qemu_path);
        adapter = adapter.apply_config(&config)?;
        
        #[cfg(unix)]
        let drive_arg = format!("file=nbd+unix:///?socket={},format=raw", socket_path.display());
        
        #[cfg(windows)]
        let drive_arg = {
            let port_str = tokio::fs::read_to_string(&socket_path)
                .await
                .map_err(|e| plaza_foundation::core::PlazaError::process(format!("Failed to read NBD port: {}", e)))?;
            format!("file=nbd:127.0.0.1:{},format=raw,if=virtio", port_str)
        };

        // Add NBD drive parameter for base image (vda)
        adapter.add_arg("-drive".into());
        adapter.add_arg(drive_arg.into());
        
        // Add NBD drive parameter for workspace image (vdb)
        if let Some(ws_socket_path) = workspace_socket_path {
            #[cfg(unix)]
            let ws_drive_arg = format!("file=nbd+unix:///?socket={},format=raw", ws_socket_path.display());
            
            #[cfg(windows)]
            let ws_drive_arg = {
                let ws_port_str = tokio::fs::read_to_string(&ws_socket_path)
                    .await
                    .map_err(|e| plaza_foundation::core::PlazaError::process(format!("Failed to read NBD workspace port: {}", e)))?;
                format!("file=nbd:127.0.0.1:{},format=raw,if=virtio", ws_port_str)
            };
            
            adapter.add_arg("-drive".into());
            adapter.add_arg(ws_drive_arg.into());
        }
        
        // Add modloop drive (vdc)
        if let Some(modloop) = &config.modloop_path {
            adapter.add_arg("-drive".into());
            adapter.add_arg(format!("file={},format=raw,if=virtio,readonly=on", modloop.display()).into());
        }
        
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
        
        // Phase 19: Explicitly flush COW layer before shutting down NBD
        if let Some(storage) = &entry.1 {
            let mut device = storage.device.lock().await;
            if let Err(e) = device.flush().await {
                tracing::warn!("Failed to flush storage before stop: {}", e);
            }
        }
        
        if let Some(handle) = entry.4.take() {
            handle.abort();
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
            process.force_kill().await?;
        }
        
        if let Some(storage) = &entry.1 {
            let mut device = storage.device.lock().await;
            let _ = device.flush().await;
        }

        if let Some(handle) = entry.4.take() {
            handle.abort();
        }
        entry.0.status = RuntimeStatus::Stopped;
        Ok(())
    }

    async fn destroy(&self, instance_id: &str) -> PlazaResult<()> {
        let mut instances = self.instances.lock().await;
        if let Some(mut entry) = instances.remove(instance_id) {
            if let Some(mut process) = entry.2.take() {
                let _ = process.force_kill().await;
            }
            if let Some(storage) = &entry.1 {
                let mut device = storage.device.lock().await;
                let _ = device.flush().await;
            }
            if let Some(handle) = entry.4.take() {
                handle.abort();
            }
        }
        Ok(())
    }

    async fn status(&self, instance_id: &str) -> PlazaResult<RuntimeStatus> {
        let mut instances = self.instances.lock().await;
        let entry = instances.get_mut(instance_id).ok_or_else(|| {
            PlazaError::process(format!("Instance {} not found", instance_id))
        })?;
        
        if let Some(process) = entry.2.as_mut() {
            match process.status().await {
                Ok(s) if s == "running" => entry.0.status = RuntimeStatus::Running,
                Ok(_) => entry.0.status = RuntimeStatus::Stopped,
                Err(_) => {
                    // Process likely died or QMP disconnected
                    entry.0.status = RuntimeStatus::Error;
                    if let Some(handle) = entry.4.take() {
                        handle.abort();
                    }
                }
            }
        }
        
        Ok(entry.0.status)
    }

    async fn metrics(&self, instance_id: &str) -> PlazaResult<RuntimeMetrics> {
        let instances = self.instances.lock().await;
        if let Some(entry) = instances.get(instance_id) {
            let pid = entry.2.as_ref().and_then(|p| p.id());
            let uptime_secs = if entry.0.status == RuntimeStatus::Running {
                let now = Timestamp::now();
                let created = &entry.0.created_at;
                
                // Extremely simple and robust parsing logic for ISO 8601 timestamps
                let parse_epoch = |ts: &Timestamp| -> Option<u64> {
                    let s = ts.to_string();
                    if s.len() >= 19 {
                        // "2026-08-12T15:51:00Z"
                        let year = s[0..4].parse::<u64>().ok()?;
                        let month = s[5..7].parse::<u64>().ok()?;
                        let day = s[8..10].parse::<u64>().ok()?;
                        let hour = s[11..13].parse::<u64>().ok()?;
                        let minute = s[14..16].parse::<u64>().ok()?;
                        let second = s[17..19].parse::<u64>().ok()?;
                        
                        // Very rough approximation of seconds since 1970 (assuming 30 days/month, etc)
                        // This is sufficient for relative uptime calculation during tests/demo.
                        let epoch = (year - 1970) * 31536000 + month * 2592000 + day * 86400 + hour * 3600 + minute * 60 + second;
                        Some(epoch)
                    } else {
                        None
                    }
                };

                if let (Some(t_now), Some(t_created)) = (parse_epoch(&now), parse_epoch(created)) {
                    Some(t_now.saturating_sub(t_created))
                } else {
                    Some(0)
                }
            } else {
                None
            };
            
            Ok(RuntimeMetrics {
                pid,
                uptime_secs,
                execution_mode: Some("TCG (Userspace NBD)".into()),
                storage_backend: Some(if entry.1.is_some() { "LayeredBlockDevice + Userspace NBD" } else { "None" }.into()),
                ..Default::default()
            })
        } else {
            Err(PlazaError::process(format!("Instance {} not found", instance_id)))
        }
    }

    async fn exec(&self, instance_id: &str, cmd: &str) -> PlazaResult<()> {
        self.send_command(instance_id, cmd).await
    }
}

