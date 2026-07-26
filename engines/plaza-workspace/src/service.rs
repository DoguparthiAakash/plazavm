//! Workspace domain service for managing workspace operations.

use crate::model::*;
use plaza_foundation::core::id::WorkspaceId;
use plaza_foundation::core::{PlazaError, PlazaResult};
use plaza_foundation::events::{EventBus, PlazaEvent};
use plaza_storage::SqliteWorkspaceRepository;
use std::sync::Arc;
use tracing::info;

/// Workspace application service managing persistence and lifecycle events.
pub struct WorkspaceService {
    repo: SqliteWorkspaceRepository,
    event_bus: Arc<EventBus>,
}

impl WorkspaceService {
    pub fn new(repo: SqliteWorkspaceRepository, event_bus: Arc<EventBus>) -> Self {
        Self { repo, event_bus }
    }

    /// Create a new workspace from a specification.
    pub async fn create_workspace(
        &self,
        name: &str,
        spec: WorkspaceSpec,
    ) -> PlazaResult<Workspace> {
        let workspace = Workspace::new(name, spec);
        let _id_str = workspace.id.to_string();
        let ws_dir = plaza_foundation::core::paths::workspaces_dir().join(&name);

        // Provision Directory Tree
        let dirs_to_create = [
            ws_dir.join(".plaza"),
            ws_dir.join("project"),
            ws_dir.join("runtime"),
            ws_dir.join("registry"),
            ws_dir.join("packages"),
            ws_dir.join("plugins"),
            ws_dir.join("storage"),
            ws_dir.join("snapshots"),
            ws_dir.join("events"),
            ws_dir.join("cache"),
            ws_dir.join("downloads"),
            ws_dir.join("logs"),
            ws_dir.join("artifacts"),
            ws_dir.join("templates"),
            ws_dir.join("metadata"),
            ws_dir.join("config"),
            ws_dir.join("locks"),
        ];

        for dir in &dirs_to_create {
            std::fs::create_dir_all(dir).map_err(|e| {
                PlazaError::storage(format!("Failed to create workspace directory {}: {}", dir.display(), e))
            })?;
        }

        // Initialize SQLite Databases
        let plaza_dir = ws_dir.join(".plaza");
        let workspace_db_path = plaza_dir.join("workspace.db");
        let metrics_db_path = plaza_dir.join("metrics.db");
        let events_db_path = plaza_dir.join("events.db");

        // Use SqliteWorkspaceRepository to init and run migrations on workspace.db
        let _workspace_repo = SqliteWorkspaceRepository::open(workspace_db_path.clone())?;
        
        // Touch metrics and events DB files
        std::fs::File::create(&metrics_db_path).map_err(|e| PlazaError::storage(e.to_string()))?;
        std::fs::File::create(&events_db_path).map_err(|e| PlazaError::storage(e.to_string()))?;

        // Generate Manifest
        let manifest_content = toml::to_string_pretty(&workspace.spec)
            .map_err(|e| PlazaError::serialization(e.to_string()))?;
        std::fs::write(plaza_dir.join("manifest.toml"), manifest_content)
            .map_err(|e| PlazaError::storage(e.to_string()))?;

        // Generate Metadata JSON
        let metadata_content = serde_json::to_string_pretty(&workspace.metadata)
            .map_err(|e| PlazaError::serialization(e.to_string()))?;
        std::fs::write(ws_dir.join("metadata").join("initial.json"), metadata_content)
            .map_err(|e| PlazaError::storage(e.to_string()))?;

        // State files
        std::fs::write(plaza_dir.join("workspace.version"), "1.0.0")
            .map_err(|e| PlazaError::storage(e.to_string()))?;
        std::fs::write(plaza_dir.join("workspace.state"), "Stopped")
            .map_err(|e| PlazaError::storage(e.to_string()))?;

        // Config placeholders
        for config_file in &["engine.toml", "runtime.toml", "registry.toml", "packages.toml", "plugins.toml", "security.toml"] {
            std::fs::write(plaza_dir.join(config_file), "")
                .map_err(|e| PlazaError::storage(e.to_string()))?;
        }
        
        // Save to central repository
        self.save_workspace(&workspace).await?;

        // Emit lifecycle event
        self.event_bus
            .publish(PlazaEvent::WorkspaceCreated {
                id: workspace.id.clone(),
                name: workspace.name.clone(),
            })
            .await;

        info!(id = %workspace.id, name = %workspace.name, "workspace created with full directory structure");
        Ok(workspace)
    }

    /// Save a workspace to the repository.
    pub async fn save_workspace(&self, workspace: &Workspace) -> PlazaResult<()> {
        let spec_json = serde_json::to_string(&workspace.spec)
            .map_err(|e| PlazaError::serialization(e.to_string()))?;
        let status_json = serde_json::to_string(&workspace.status)
            .map_err(|e| PlazaError::serialization(e.to_string()))?;
        let meta_json = serde_json::to_string(&workspace.metadata)
            .map_err(|e| PlazaError::serialization(e.to_string()))?;

        self.repo.save_raw(
            &workspace.id,
            &workspace.name,
            workspace.description.as_deref(),
            &spec_json,
            &status_json,
            &meta_json,
        )?;

        Ok(())
    }

    /// Get a workspace by ID.
    pub async fn get_workspace(&self, id: &WorkspaceId) -> PlazaResult<Option<Workspace>> {
        if let Some((name, description, spec_json, status_json, meta_json)) =
            self.repo.get_raw(id)?
        {
            let spec: WorkspaceSpec = serde_json::from_str(&spec_json)
                .map_err(|e| PlazaError::serialization(e.to_string()))?;
            let status: WorkspaceStatus = serde_json::from_str(&status_json)
                .map_err(|e| PlazaError::serialization(e.to_string()))?;
            let metadata: WorkspaceMetadata = serde_json::from_str(&meta_json)
                .map_err(|e| PlazaError::serialization(e.to_string()))?;

            let graph = super::graph::WorkspaceGraph::single_node(
                "main",
                spec.runtime.clone(),
                spec.resources.clone(),
            );

            Ok(Some(Workspace {
                id: id.clone(),
                name,
                description,
                spec,
                status,
                metadata,
                graph,
            }))
        } else {
            Ok(None)
        }
    }

    /// List all workspaces.
    pub async fn list_workspaces(&self) -> PlazaResult<Vec<Workspace>> {
        let raw_list = self.repo.list_raw()?;
        let mut workspaces = Vec::new();

        for (id_str, name, description, spec_json, status_json, meta_json) in raw_list {
            if let Ok(id) = WorkspaceId::parse(&id_str) {
                if let (Ok(spec), Ok(status), Ok(metadata)) = (
                    serde_json::from_str::<WorkspaceSpec>(&spec_json),
                    serde_json::from_str::<WorkspaceStatus>(&status_json),
                    serde_json::from_str::<WorkspaceMetadata>(&meta_json),
                ) {
                    let graph = super::graph::WorkspaceGraph::single_node(
                        "main",
                        spec.runtime.clone(),
                        spec.resources.clone(),
                    );
                    workspaces.push(Workspace {
                        id,
                        name,
                        description,
                        spec,
                        status,
                        metadata,
                        graph,
                    });
                }
            }
        }

        Ok(workspaces)
    }

    /// Update the desired state of a workspace.
    pub async fn set_desired_state(
        &self,
        id: &WorkspaceId,
        state: DesiredState,
    ) -> PlazaResult<()> {
        if let Some(mut ws) = self.get_workspace(id).await? {
            ws.spec.desired_state = state;
            self.save_workspace(&ws).await?;

            self.event_bus
                .publish(PlazaEvent::WorkspaceDesiredStateChanged {
                    id: id.clone(),
                    desired: format!("{state:?}"),
                })
                .await;
        } else {
            return Err(PlazaError::WorkspaceNotFound(id.clone()));
        }
        Ok(())
    }

    /// Delete a workspace and its directory.
    pub async fn delete_workspace(&self, id: &WorkspaceId) -> PlazaResult<()> {
        if let Some(ws) = self.get_workspace(id).await? {
            // Delete from repository
            self.repo.delete(id)?;
            
            // Delete from filesystem
            let ws_dir = plaza_foundation::core::paths::workspaces_dir().join(&ws.name);
            if ws_dir.exists() {
                std::fs::remove_dir_all(&ws_dir)
                    .map_err(|e| PlazaError::storage(format!("Failed to delete workspace directory {}: {}", ws_dir.display(), e)))?;
            }

            self.event_bus
                .publish(PlazaEvent::WorkspaceDeleted { id: id.clone() })
                .await;
            info!(id = %id, name = %ws.name, "workspace deleted");
            Ok(())
        } else {
            Err(PlazaError::WorkspaceNotFound(id.clone()))
        }
    }
}

