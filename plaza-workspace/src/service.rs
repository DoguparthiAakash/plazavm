//! Workspace domain service for managing workspace operations.

use crate::model::*;
use plaza_core::id::WorkspaceId;
use plaza_core::{PlazaError, PlazaResult};
use plaza_events::{EventBus, PlazaEvent};
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
        self.save_workspace(&workspace).await?;

        self.event_bus
            .publish(PlazaEvent::WorkspaceCreated {
                id: workspace.id.clone(),
                name: workspace.name.clone(),
            })
            .await;

        info!(id = %workspace.id, name = %workspace.name, "workspace created");
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

    /// Delete a workspace.
    pub async fn delete_workspace(&self, id: &WorkspaceId) -> PlazaResult<()> {
        self.repo.delete(id)?;
        self.event_bus
            .publish(PlazaEvent::WorkspaceDeleted { id: id.clone() })
            .await;
        info!(id = %id, "workspace deleted");
        Ok(())
    }
}
