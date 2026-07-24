//! Shared AppState holding handles to all subsystem services.

use crate::bootstrap::BootstrapBuilder;
use plaza_controller::WorkspaceController;
use plaza_decision::DecisionEngine;
use plaza_events::EventBus;
use plaza_monitor::SystemMonitor;
use plaza_platform::PlatformDetector;
use plaza_plugin::PluginHost;
use plaza_registry::{RuntimeImageRegistry, WorkspaceTemplateRegistry};
use plaza_resource::ResourceManager;
use plaza_storage::SqliteWorkspaceRepository;
use plaza_workspace::WorkspaceService;
use std::sync::Arc;

/// Central application state initialized at daemon/desktop startup.
#[derive(Clone)]
pub struct AppState {
    pub container: Arc<crate::bootstrap::Container>,
    pub event_bus: Arc<EventBus>,
    pub platform: Arc<PlatformDetector>,
    pub repo: SqliteWorkspaceRepository,
    pub workspace_service: Arc<WorkspaceService>,
    pub resource_manager: Arc<ResourceManager>,
    pub plugin_host: Arc<PluginHost>,
    pub decision_engine: Arc<DecisionEngine>,
    pub controller: Arc<WorkspaceController>,
    pub monitor: Arc<SystemMonitor>,
    pub image_registry: Arc<RuntimeImageRegistry>,
    pub template_registry: Arc<WorkspaceTemplateRegistry>,
}

impl AppState {
    /// Initialize the complete PlazaVM subsystem graph via composition root.
    pub async fn initialize() -> plaza_core::PlazaResult<Self> {
        let container = Arc::new(BootstrapBuilder::new().build().await?);
        Ok(Self {
            container: container.clone(),
            event_bus: container.event_bus.clone(),
            platform: container.platform.clone(),
            repo: container.repo.clone(),
            workspace_service: container.workspace_service.clone(),
            resource_manager: container.resource_manager.clone(),
            plugin_host: container.plugin_host.clone(),
            decision_engine: container.decision_engine.clone(),
            controller: container.controller.clone(),
            monitor: container.monitor.clone(),
            image_registry: container.image_registry.clone(),
            template_registry: container.template_registry.clone(),
        })
    }
}
