use async_trait::async_trait;
use plaza_foundation::engine::errors::PfeResult;
use plaza_foundation::engine::manager::Engine;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use crate::service::WorkspaceService;
use plaza_runtime::RuntimeManager;
use plaza_image::ImageManager;
use crate::model::DesiredState;
use tokio::time::{sleep, Duration};
use plaza_runtime::runtime::GuestRuntimeParams;

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
                let ws_count;
                if let Ok(workspaces) = ws_svc.list_workspaces().await {
                    ws_count = workspaces.len();
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
                                // Start the instance using GuestRuntime abstraction
                                if let Err(e) = Self::start_workspace(
                                    &ws,
                                    &rt_mgr,
                                    &img_mgr,
                                    &ws_svc,
                                    &backend_id,
                                ).await {
                                    tracing::error!("Failed to start workspace {}: {}", id_str, e);
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
                } else {
                    ws_count = 0;
                }
                // Backoff: poll every 10s normally, 30s when no workspaces exist
                let poll_interval = if ws_count == 0 {
                    Duration::from_secs(30)
                } else {
                    Duration::from_secs(10)
                };
                sleep(poll_interval).await;
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

impl WorkspaceEngine {
    /// Start a workspace using the GuestRuntime abstraction.
    ///
    /// This is the primary entry point for workspace startup. It:
    /// 1. Resolves the appropriate GuestRuntime from the workspace spec
    /// 2. Provisions the immutable base image
    /// 3. Creates the workspace writable storage
    /// 4. Builds the MachineConfig via the GuestRuntime
    /// 5. Creates and starts the runtime instance via the backend
    pub async fn start_workspace(
        ws: &crate::model::Workspace,
        rt_mgr: &Arc<RuntimeManager>,
        img_mgr: &Arc<ImageManager>,
        ws_svc: &Arc<WorkspaceService>,
        backend_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let id_str = ws.id.to_string();
        tracing::info!("Preparing to start runtime for workspace {} [{}]", ws.name, id_str);

        // Resolve the GuestRuntime for this workspace
        let guest_runtime = crate::runtime::create_guest_runtime(ws.spec.guest_runtime.clone());

        tracing::info!(
            "Using {} guest runtime for workspace '{}'",
            guest_runtime.display_name(),
            ws.name
        );

        // Build runtime parameters
        let ws_dir = plaza_foundation::core::paths::workspaces_dir().join(&ws.name);
        let project_path = ws.metadata.project_path.as_ref().map(|p| PathBuf::from(p));
        let params = GuestRuntimeParams {
            workspace_id: id_str.clone(),
            workspace_name: ws.name.clone(),
            workspace_dir: ws_dir,
            image_reference: ws.spec.runtime.image.clone(),
            project_path,
        };

        // Step 1: Resolve artifacts (kernel, initrd, etc.)
        let artifacts = guest_runtime.resolve_artifacts(&params).await
            .map_err(|e| format!("Failed to resolve artifacts: {}", e))?;

        // Create a minimal base image for /dev/vda if one doesn't exist.
        // For initramfs-based boots, this is just a minimal block device.
        let ws_dir = plaza_foundation::core::paths::workspaces_dir().join(&ws.name);
        let storage_dir = ws_dir.join(".plaza").join("storage");
        tokio::fs::create_dir_all(&storage_dir).await.map_err(|e| e.to_string())?;
        let base_img = storage_dir.join("base.img");
        if !base_img.exists() {
            // Create a minimal 1MB base image
            let file = std::fs::File::create(&base_img).map_err(|e| e.to_string())?;
            file.set_len(1024 * 1024).map_err(|e| e.to_string())?;
        }
        let base_path = base_img;

        // Step 3: Create workspace writable storage device
        let ws_dev = guest_runtime.create_workspace_device(&params).await
            .map_err(|e| format!("Failed to create workspace device: {}", e))?;

        // Step 4: Set up RuntimeStorage with immutable base + writable workspace
        let immutable_layer = plaza_image::block::FileBackedImmutableLayer::open(base_path.clone()).await
            .map_err(|e| format!("Failed to open immutable layer: {}", e))?;
        let read_only_device = plaza_image::block::ReadOnlyBlockDevice::new(Arc::new(immutable_layer));
        let mut storage = plaza_runtime::RuntimeStorage::new(read_only_device);
        let ws_dev_arc: Arc<tokio::sync::Mutex<dyn plaza_image::block::VirtualBlockDevice>> =
            Arc::new(tokio::sync::Mutex::new(ws_dev));
        storage.workspace_device = Some(ws_dev_arc);

        // Step 5: Build MachineConfig via GuestRuntime
        let machine = guest_runtime.build_machine_config(&params, &base_path, &storage).await
            .map_err(|e| format!("Failed to build machine config: {}", e))?;

        // Step 6: Resolve backend and create/start runtime instance
        let backend = rt_mgr.get_backend(backend_id)
            .map_err(|e| format!("Backend '{}' not found: {}", backend_id, e))?;

        let rt_instance = backend.create(&machine, storage).await
            .map_err(|e| format!("Failed to create runtime instance: {}", e))?;

        tracing::info!("Created runtime instance: {}", rt_instance.id);

        backend.start(&rt_instance.id).await
            .map_err(|e| format!("Failed to start runtime: {}", e))?;

        tracing::info!("Started runtime for {}", rt_instance.id);

        let _ = ws_svc.update_status(&ws.id, |status| {
            status.runtime_instance_id = Some(rt_instance.id.clone());
        }).await;

        Ok(())
    }
}
