use async_trait::async_trait;
use plaza_foundation::engine::errors::PfeResult;
use plaza_foundation::engine::manager::Engine;
use std::collections::HashMap;

use std::sync::Arc;
use crate::service::WorkspaceService;
use plaza_runtime::RuntimeManager;
use plaza_image::ImageManager;
use crate::model::DesiredState;
use tokio::time::{sleep, Duration};

pub struct WorkspaceEngine {
    workspace_service: Arc<WorkspaceService>,
    runtime_manager: Arc<RuntimeManager>,
    image_manager: Arc<ImageManager>,
}

impl WorkspaceEngine {
    pub fn new(
        workspace_service: Arc<WorkspaceService>,
        runtime_manager: Arc<RuntimeManager>,
        image_manager: Arc<ImageManager>,
    ) -> Self {
        Self {
            workspace_service,
            runtime_manager,
            image_manager,
        }
    }
}

#[async_trait]
impl Engine for WorkspaceEngine {
    fn name(&self) -> &'static str {
        "workspace_engine"
    }

    fn dependencies(&self) -> Vec<&'static str> {
        vec!["storage_engine"] // Depends on storage being up first
    }

    async fn initialize(&self) -> PfeResult<()> {
        Ok(())
    }

    async fn start(&self) -> PfeResult<()> {
        let ws_svc = self.workspace_service.clone();
        let rt_mgr = self.runtime_manager.clone();
        let img_mgr = self.image_manager.clone();
        
        tokio::spawn(async move {
            loop {
                if let Ok(workspaces) = ws_svc.list_workspaces().await {
                    for ws in workspaces {
                        let id_str = ws.id.to_string();
                        let backend_id = match &ws.spec.runtime.backend {
                            crate::model::RuntimeBackendPreference::Preferred(id) => id.clone(),
                            crate::model::RuntimeBackendPreference::Pinned(id) => id.clone(),
                            crate::model::RuntimeBackendPreference::Auto => "v86".to_string(),
                        };
                        
                        // Get runtime status and metrics
                        let mut is_running = false;
                        let instance_id_to_check = ws.status.runtime_instance_id.clone().unwrap_or_else(|| id_str.clone());
                        if let Ok(backend) = rt_mgr.get_backend(&backend_id) {
                            if let Ok(plaza_runtime::RuntimeStatus::Running) = backend.status(&instance_id_to_check).await {
                                is_running = true;
                                if let Ok(metrics) = backend.metrics(&instance_id_to_check).await {
                                    let _ = ws_svc.update_status(&ws.id, |status| {
                                        status.state = crate::model::WorkspaceState::Running;
                                        status.pid = metrics.pid;
                                        status.uptime_secs = metrics.uptime_secs;
                                        status.execution_mode = metrics.execution_mode.clone();
                                        status.storage_backend = metrics.storage_backend.clone();
                                        status.runtime_backend = Some(backend_id.clone());
                                        status.runtime_instance_id = Some(instance_id_to_check.clone());
                                    }).await;
                                }
                            } else {
                                let _ = ws_svc.update_status(&ws.id, |status| {
                                    status.state = crate::model::WorkspaceState::Stopped;
                                    status.pid = None;
                                    status.uptime_secs = None;
                                    status.execution_mode = None;
                                    status.storage_backend = None;
                                }).await;
                            }
                        }
                        
                        match (ws.spec.desired_state, is_running) {
                            (DesiredState::Running, false) => {
                                // Start the instance
                                if let Ok(backend) = rt_mgr.get_backend(&backend_id) {
                                    tracing::info!("Preparing to start runtime for workspace {} [{}]", ws.name, id_str);
                                    
                                    // Resolve the base image
                                    let ws_dir = plaza_foundation::core::paths::workspaces_dir().join(&ws.name);
                                    let plaza_yaml_path = ws_dir.join("..").join("plaza.yaml"); // Assuming ws is inside .space
                                    let plaza_yaml_path = if plaza_yaml_path.exists() {
                                        plaza_yaml_path
                                    } else {
                                        // Fallback to project dir if initialized directly
                                        std::env::current_dir().unwrap_or_default().join("plaza.yaml")
                                    };
                                    
                                    let mut base_path = None;
                                    if plaza_yaml_path.exists() {
                                        if let Ok(content) = tokio::fs::read_to_string(&plaza_yaml_path).await {
                                            if let Ok(yaml) = plaza_foundation::config::PlazaYaml::parse_yaml(&content) {
                                                if let Ok(image_id) = crate::pipeline::TransactionalPipelineBuilder::provision_image(&yaml, img_mgr.clone()).await {
                                                    if let Ok(manifest) = img_mgr.inspect_image(&image_id).await {
                                                        if let Some(layer) = manifest.layers.first() {
                                                            base_path = img_mgr.get_blob_path(&layer.digest).ok();
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    
                                    if base_path.is_none() {
                                        tracing::error!("Failed to resolve immutable base image for workspace {}", id_str);
                                        continue;
                                    }
                                    let base_path = base_path.unwrap();
                                    
                                    // Setup workspace storage
                                    let storage_dir = ws_dir.join(".plaza").join("storage");
                                    let _ = tokio::fs::create_dir_all(&storage_dir).await;
                                    let cow_path = storage_dir.join(format!("{}.img", id_str));
                                    
                                    use plaza_image::block::{Ext4WritableFilesystemProvider, WritableFilesystemProvider, FileBackedImmutableLayer};
                                    
                                    let provider = Ext4WritableFilesystemProvider::new(cow_path.clone(), 4096);
                                    let ws_dev = match provider.create(1024 * 1024 * 1024).await { // 1GB limit for now
                                        Ok(dev) => dev,
                                        Err(e) => {
                                            tracing::error!("Failed to create Ext4 workspace storage for {}: {}", id_str, e);
                                            continue;
                                        }
                                    };
                                    
                                    let immutable_layer = match FileBackedImmutableLayer::open(base_path.to_path_buf()).await {
                                        Ok(layer) => layer,
                                        Err(e) => {
                                            tracing::error!("Failed to open base immutable layer for {}: {}", id_str, e);
                                            continue;
                                        }
                                    };
                                    
                                    let read_only_device = plaza_image::block::ReadOnlyBlockDevice::new(std::sync::Arc::new(immutable_layer));
                                    let mut storage = plaza_runtime::RuntimeStorage::new(read_only_device);
                                    let ws_dev_arc: std::sync::Arc<tokio::sync::Mutex<dyn plaza_image::block::VirtualBlockDevice>> = std::sync::Arc::new(tokio::sync::Mutex::new(ws_dev));
                                    storage.workspace_device = Some(ws_dev_arc);
                                    
                                    let acq = crate::image::acquisition::AlpineAcquisitionSource::new().unwrap();
                                    let (kernel_path, initrd_path, modloop_path) = acq.fetch_kernel_and_initrd("alpine:3.19.1").await.unwrap();
                                    
                                    let mut machine_section = plaza_foundation::config::machine_section::MachineSection::default();
                                    if backend_id == "v86" {
                                        machine_section.architecture = plaza_foundation::core::types::Architecture::X86_32;
                                    }

                                    let machine = plaza_runtime::MachineConfig {
                                        workspace_id: id_str.clone(),
                                        instance_id: id_str.clone(),
                                        machine: machine_section,
                                        capabilities: plaza_foundation::core::CapabilityPolicy::default(),
                                        boot_device: std::path::PathBuf::from("dummy"),
                                        kernel_path: Some(kernel_path),
                                        initrd_path: Some(initrd_path),
                                        kernel_args: Some("console=ttyS0 root=/dev/vda rw init=/plaza-init".into()),
                                        modloop_path,
                                        os_target: plaza_runtime::OperatingSystemTarget::Linux,
                                        volume_mounts: std::collections::HashMap::new(),
                                        port_forwards: std::collections::HashMap::new(),
                                        env_vars: std::collections::HashMap::new(),
                                    };
                                    
                                    match backend.create(&machine, storage).await {
                                        Ok(rt_instance) => {
                                            tracing::info!("Created runtime instance: {}", rt_instance.id);
                                            if let Err(e) = backend.start(&rt_instance.id).await {
                                                tracing::error!("Failed to start runtime for {}: {}", rt_instance.id, e);
                                            } else {
                                                tracing::info!("Started runtime for {}", rt_instance.id);
                                                let _ = ws_svc.update_status(&ws.id, |status| {
                                                    status.runtime_instance_id = Some(rt_instance.id.clone());
                                                }).await;
                                            }
                                        }
                                        Err(e) => {
                                            tracing::error!("Failed to create runtime instance {}: {}", id_str, e);
                                        }
                                    }
                                }
                            }
                            (DesiredState::Stopped, true) => {
                                // Stop the instance
                                if let Ok(backend) = rt_mgr.get_backend(&backend_id) {
                                    if let Err(e) = backend.stop(&instance_id_to_check).await {
                                        tracing::error!("Failed to stop runtime for {}: {}", instance_id_to_check, e);
                                    } else {
                                        tracing::info!("Stopped runtime for {}", instance_id_to_check);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                sleep(Duration::from_secs(2)).await;
            }
        });
        Ok(())
    }

    async fn stop(&self) -> PfeResult<()> {
        Ok(())
    }

    async fn restart(&self) -> PfeResult<()> {
        Ok(())
    }

    async fn reload(&self) -> PfeResult<()> {
        Ok(())
    }

    async fn recover(&self) -> PfeResult<()> {
        Ok(())
    }

    async fn shutdown(&self) -> PfeResult<()> {
        Ok(())
    }

    async fn health(&self) -> PfeResult<String> {
        Ok("HEALTHY".to_string())
    }

    async fn metrics(&self) -> PfeResult<HashMap<String, String>> {
        Ok(HashMap::new())
    }

    async fn diagnostics(&self) -> PfeResult<Vec<String>> {
        Ok(vec![])
    }

    async fn status(&self) -> PfeResult<String> {
        Ok("RUNNING".to_string())
    }


}

