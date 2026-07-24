//! WorkspaceController reconciliation loop.

use plaza_core::PlazaResult;
use plaza_decision::DecisionEngine;
use plaza_events::{EventBus, PlazaEvent};
use plaza_plugin::PluginHost;
use plaza_resource::ResourceManager;
use plaza_workspace::model::{DesiredState, Workspace, WorkspaceState};
use plaza_workspace::WorkspaceService;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

const RECONCILE_INTERVAL_SECS: u64 = 5;

/// The primary controller continuous reconciliation loop.
pub struct WorkspaceController {
    workspace_service: Arc<WorkspaceService>,
    decision_engine: Arc<DecisionEngine>,
    resource_manager: Arc<ResourceManager>,
    plugin_host: Arc<PluginHost>,
    event_bus: Arc<EventBus>,
}

impl WorkspaceController {
    pub fn new(
        workspace_service: Arc<WorkspaceService>,
        decision_engine: Arc<DecisionEngine>,
        resource_manager: Arc<ResourceManager>,
        plugin_host: Arc<PluginHost>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self {
            workspace_service,
            decision_engine,
            resource_manager,
            plugin_host,
            event_bus,
        }
    }

    /// Run the reconciliation background loop until cancellation is requested.
    pub async fn run(&self, cancellation_token: CancellationToken) {
        info!("workspace controller reconciliation loop started");
        let mut event_rx = self.event_bus.subscribe();

        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    info!("workspace controller received cancellation signal");
                    break;
                }

                Ok(event) = event_rx.recv() => {
                    if let PlazaEvent::WorkspaceDesiredStateChanged { id, .. } = event {
                        if let Ok(Some(ws)) = self.workspace_service.get_workspace(&id).await {
                            let _ = self.reconcile_workspace(&ws).await;
                        }
                    }
                }

                _ = tokio::time::sleep(std::time::Duration::from_secs(RECONCILE_INTERVAL_SECS)) => {
                    if let Ok(workspaces) = self.workspace_service.list_workspaces().await {
                        for ws in workspaces {
                            let _ = self.reconcile_workspace(&ws).await;
                        }
                    }
                }
            }
        }
    }

    /// Reconcile a single workspace (desired vs actual state).
    pub async fn reconcile_workspace(&self, workspace: &Workspace) -> PlazaResult<()> {
        let desired = workspace.spec.desired_state;
        let actual = workspace.status.state;

        self.event_bus
            .publish(PlazaEvent::ReconciliationStarted {
                workspace_id: workspace.id.clone(),
            })
            .await;

        let result = match (desired, actual) {
            // Desired: Running, Actual: Stopped => Schedule & Start
            (DesiredState::Running, WorkspaceState::Stopped) => {
                self.start_workspace(workspace).await
            }

            // Desired: Stopped, Actual: Running => Stop & Release
            (DesiredState::Stopped, WorkspaceState::Running) => {
                self.stop_workspace(workspace).await
            }

            // Desired: Destroyed, Actual: Stopped => Delete
            (DesiredState::Destroyed, WorkspaceState::Stopped) => {
                self.workspace_service.delete_workspace(&workspace.id).await
            }

            // Already in desired state or in-flight state
            _ => Ok(()),
        };

        match result {
            Ok(_) => {
                self.event_bus
                    .publish(PlazaEvent::ReconciliationCompleted {
                        workspace_id: workspace.id.clone(),
                        action: format!("{desired:?} -> {actual:?}"),
                    })
                    .await;
            }
            Err(ref e) => {
                self.event_bus
                    .publish(PlazaEvent::ReconciliationFailed {
                        workspace_id: workspace.id.clone(),
                        error: e.to_string(),
                    })
                    .await;
                warn!(id = %workspace.id, error = %e, "reconciliation failed");
            }
        }

        result
    }

    async fn start_workspace(&self, workspace: &Workspace) -> PlazaResult<()> {
        info!(id = %workspace.id, name = %workspace.name, "reconciler starting workspace");

        let mut updated = workspace.clone();
        updated.status.state = WorkspaceState::Scheduling;
        self.workspace_service.save_workspace(&updated).await?;

        // 1. Decision engine selects backend & plans resources
        let decision = match self.decision_engine.decide(workspace.spec.clone()).await {
            Ok(d) => d,
            Err(e) => {
                updated.status.state = WorkspaceState::Error;
                self.workspace_service.save_workspace(&updated).await?;
                return Err(e);
            }
        };

        self.event_bus
            .publish(PlazaEvent::RuntimeSelected {
                workspace_id: workspace.id.clone(),
                backend: decision.selected_backend.backend_id.clone(),
                reason: decision.selected_backend.reason.clone(),
            })
            .await;

        // 2. Resource manager allocates resources
        let mut plan = decision.resource_plan;
        plan.workspace_id = workspace.id.clone();
        if let Err(e) = self.resource_manager.allocate(&plan).await {
            updated.status.state = WorkspaceState::Error;
            self.workspace_service.save_workspace(&updated).await?;
            return Err(e);
        }

        // 3. Execution via plugin
        let backend_id = decision.selected_backend.backend_id.clone();
        updated.status.state = WorkspaceState::Starting;
        updated.status.runtime_backend = Some(backend_id.clone());
        self.workspace_service.save_workspace(&updated).await?;

        self.event_bus
            .publish(PlazaEvent::WorkspaceStarting {
                id: workspace.id.clone(),
            })
            .await;

        if let Some(plugin) = self.plugin_host.get_runtime_plugin(&backend_id).await {
            let spec_json = serde_json::to_string(&workspace.spec).unwrap_or_default();
            match plugin.create(&spec_json).await {
                Ok(instance) => {
                    if let Err(e) = plugin.start(&instance.id).await {
                        warn!(id = %workspace.id, error = %e, "plugin start failed, updating state");
                        updated.status.state = WorkspaceState::Error;
                        self.workspace_service.save_workspace(&updated).await?;
                        return Err(e);
                    }
                    updated.status.runtime_instance_id = Some(instance.id);
                    updated.status.state = WorkspaceState::Running;
                    updated.status.health = plaza_core::types::HealthStatus::Healthy;
                    self.workspace_service.save_workspace(&updated).await?;

                    self.event_bus
                        .publish(PlazaEvent::WorkspaceStarted {
                            id: workspace.id.clone(),
                            runtime_backend: backend_id,
                        })
                        .await;

                    info!(id = %workspace.id, "workspace started successfully");
                }
                Err(e) => {
                    updated.status.state = WorkspaceState::Error;
                    self.workspace_service.save_workspace(&updated).await?;
                    return Err(e);
                }
            }
        } else {
            // Plugin not loaded (stub behavior for Phase 1 testing)
            info!(id = %workspace.id, backend = %backend_id, "plugin stub active for Phase 1");
            updated.status.state = WorkspaceState::Running;
            updated.status.health = plaza_core::types::HealthStatus::Healthy;
            self.workspace_service.save_workspace(&updated).await?;

            self.event_bus
                .publish(PlazaEvent::WorkspaceStarted {
                    id: workspace.id.clone(),
                    runtime_backend: backend_id,
                })
                .await;
        }

        Ok(())
    }

    async fn stop_workspace(&self, workspace: &Workspace) -> PlazaResult<()> {
        info!(id = %workspace.id, name = %workspace.name, "reconciler stopping workspace");

        let mut updated = workspace.clone();
        updated.status.state = WorkspaceState::Stopping;
        self.workspace_service.save_workspace(&updated).await?;

        self.event_bus
            .publish(PlazaEvent::WorkspaceStopping {
                id: workspace.id.clone(),
            })
            .await;

        if let (Some(backend_id), Some(instance_id)) = (
            &workspace.status.runtime_backend,
            &workspace.status.runtime_instance_id,
        ) {
            if let Some(plugin) = self.plugin_host.get_runtime_plugin(backend_id).await {
                let _ = plugin.stop(instance_id).await;
            }
        }

        self.resource_manager.release(&workspace.id).await?;

        updated.status.state = WorkspaceState::Stopped;
        updated.status.health = plaza_core::types::HealthStatus::Unknown;
        updated.status.runtime_instance_id = None;
        self.workspace_service.save_workspace(&updated).await?;

        self.event_bus
            .publish(PlazaEvent::WorkspaceStopped {
                id: workspace.id.clone(),
            })
            .await;

        info!(id = %workspace.id, "workspace stopped successfully");
        Ok(())
    }
}
