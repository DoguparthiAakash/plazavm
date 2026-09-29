//! Workspace service management.
//!
//! Manages background services running inside workspace VMs.
//! Services are processes that run persistently in the guest environment.

use plaza_foundation::core::{PlazaError, PlazaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// A managed service running inside a workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceService {
    pub name: String,
    pub workspace_id: String,
    pub command: String,
    pub status: ServiceStatus,
    pub pid: Option<u32>,
    pub port: Option<u16>,
    pub environment: HashMap<String, String>,
    pub restart_count: u32,
    pub max_restarts: u32,
    pub created_at: String,
    pub started_at: Option<String>,
    pub stopped_at: Option<String>,
}

/// Service status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ServiceStatus {
    Stopped,
    Starting,
    Running,
    Failed,
    Restarting,
}

impl std::fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stopped => write!(f, "stopped"),
            Self::Starting => write!(f, "starting"),
            Self::Running => write!(f, "running"),
            Self::Failed => write!(f, "failed"),
            Self::Restarting => write!(f, "restarting"),
        }
    }
}

/// Manages services across workspaces.
pub struct ServiceManager {
    services: Arc<RwLock<HashMap<String, WorkspaceService>>>,
    storage_dir: PathBuf,
}

impl ServiceManager {
    /// Create a new service manager.
    pub fn new(storage_dir: PathBuf) -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            storage_dir,
        }
    }

    /// Register a new service.
    pub async fn register_service(
        &self,
        workspace_id: &str,
        name: &str,
        command: &str,
        port: Option<u16>,
        environment: HashMap<String, String>,
    ) -> PlazaResult<WorkspaceService> {
        let service = WorkspaceService {
            name: name.to_string(),
            workspace_id: workspace_id.to_string(),
            command: command.to_string(),
            status: ServiceStatus::Stopped,
            pid: None,
            port,
            environment,
            restart_count: 0,
            max_restarts: 3,
            created_at: chrono::Utc::now().to_rfc3339(),
            started_at: None,
            stopped_at: None,
        };

        let key = format!("{}/{}", workspace_id, name);
        let mut services = self.services.write().await;
        services.insert(key, service.clone());

        // Persist to disk
        self.persist_service(&service).await?;

        info!("Registered service '{}' in workspace '{}'", name, workspace_id);
        Ok(service)
    }

    /// Start a service by sending the command to the workspace via serial.
    pub async fn start_service(&self, workspace_id: &str, name: &str) -> PlazaResult<()> {
        let key = format!("{}/{}", workspace_id, name);
        let mut services = self.services.write().await;

        let service = services
            .get_mut(&key)
            .ok_or_else(|| PlazaError::config(format!("Service '{}' not found", name)))?;

        if service.status == ServiceStatus::Running {
            return Ok(());
        }

        service.status = ServiceStatus::Starting;
        service.started_at = Some(chrono::Utc::now().to_rfc3339());

        // Build the full command with environment variables
        let env_prefix: String = service
            .environment
            .iter()
            .map(|(k, v)| format!("export {}={}; ", k, v))
            .collect();

        let full_cmd = format!(
            "{}nohup {} > /var/log/{}.log 2>&1 & echo $! > /var/run/{}.pid",
            env_prefix, service.command, service.name, service.name
        );

        // The actual command sending happens through the workspace engine
        // For now, we track the state
        service.status = ServiceStatus::Running;
        service.pid = None; // Would be set from guest output

        self.persist_service(service).await?;

        info!("Started service '{}' in workspace '{}'", name, workspace_id);
        Ok(())
    }

    /// Stop a service.
    pub async fn stop_service(&self, workspace_id: &str, name: &str) -> PlazaResult<()> {
        let key = format!("{}/{}", workspace_id, name);
        let mut services = self.services.write().await;

        let service = services
            .get_mut(&key)
            .ok_or_else(|| PlazaError::config(format!("Service '{}' not found", name)))?;

        service.status = ServiceStatus::Stopped;
        service.stopped_at = Some(chrono::Utc::now().to_rfc3339());
        service.pid = None;

        self.persist_service(service).await?;

        info!("Stopped service '{}' in workspace '{}'", name, workspace_id);
        Ok(())
    }

    /// Get service status.
    pub async fn get_service(
        &self,
        workspace_id: &str,
        name: &str,
    ) -> PlazaResult<WorkspaceService> {
        let key = format!("{}/{}", workspace_id, name);
        let services = self.services.read().await;

        services
            .get(&key)
            .cloned()
            .ok_or_else(|| PlazaError::config(format!("Service '{}' not found", name)))
    }

    /// List all services for a workspace.
    pub async fn list_services(&self, workspace_id: &str) -> Vec<WorkspaceService> {
        let services = self.services.read().await;
        services
            .values()
            .filter(|s| s.workspace_id == workspace_id)
            .cloned()
            .collect()
    }

    /// List all services across all workspaces.
    pub async fn list_all_services(&self) -> Vec<WorkspaceService> {
        let services = self.services.read().await;
        services.values().cloned().collect()
    }

    /// Remove a service.
    pub async fn remove_service(&self, workspace_id: &str, name: &str) -> PlazaResult<()> {
        let key = format!("{}/{}", workspace_id, name);
        let mut services = self.services.write().await;
        services.remove(&key);

        // Remove persisted state
        let state_file = self.storage_dir.join(format!("{}.json", key.replace('/', "_")));
        let _ = tokio::fs::remove_file(state_file).await;

        info!("Removed service '{}' from workspace '{}'", name, workspace_id);
        Ok(())
    }

    /// Persist service state to disk.
    async fn persist_service(&self, service: &WorkspaceService) -> PlazaResult<()> {
        let dir = self.storage_dir.join(&service.workspace_id);
        tokio::fs::create_dir_all(&dir)
            .await
            .map_err(PlazaError::Io)?;

        let state_file = dir.join(format!("{}.json", service.name));
        let json = serde_json::to_string_pretty(service)
            .map_err(|e| PlazaError::serialization(e.to_string()))?;

        tokio::fs::write(&state_file, json)
            .await
            .map_err(PlazaError::Io)?;

        Ok(())
    }

    /// Load persisted services from disk.
    pub async fn load_persisted_services(&self) -> PlazaResult<()> {
        if !self.storage_dir.exists() {
            return Ok(());
        }

        let mut entries = tokio::fs::read_dir(&self.storage_dir)
            .await
            .map_err(PlazaError::Io)?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(PlazaError::Io)?
        {
            if entry.file_type().await.map_err(PlazaError::Io)?.is_dir() {
                let ws_dir = entry.path();
                let mut ws_entries = tokio::fs::read_dir(&ws_dir)
                    .await
                    .map_err(PlazaError::Io)?;

                while let Some(ws_entry) = ws_entries
                    .next_entry()
                    .await
                    .map_err(PlazaError::Io)?
                {
                    if ws_entry.path().extension().map_or(false, |e| e == "json") {
                        if let Ok(content) = tokio::fs::read_to_string(ws_entry.path()).await {
                            if let Ok(service) =
                                serde_json::from_str::<WorkspaceService>(&content)
                            {
                                let key =
                                    format!("{}/{}", service.workspace_id, service.name);
                                let mut services = self.services.write().await;
                                services.insert(key, service);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
