//! Centralized Composition Root and Bootstrap Builder for PlazaVM.
//!
//! Enforces Dependency Injection across the entire application graph.
//! Nothing outside the bootstrap module constructs application services directly.

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
use std::path::PathBuf;
use std::sync::Arc;

/// Container holding all instantiated application services.
#[derive(Clone)]
pub struct Container {
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

/// Fluent builder for assembling the application dependency graph.
///
/// Supports overriding services for unit/integration testing.
pub struct BootstrapBuilder {
    db_path: Option<PathBuf>,
    in_memory_db: bool,
    event_bus: Option<Arc<EventBus>>,
    platform: Option<Arc<PlatformDetector>>,
    plugin_host: Option<Arc<PluginHost>>,
}

impl BootstrapBuilder {
    pub fn new() -> Self {
        Self {
            db_path: None,
            in_memory_db: false,
            event_bus: None,
            platform: None,
            plugin_host: None,
        }
    }

    /// Use a specific database path.
    pub fn with_db_path(mut self, path: PathBuf) -> Self {
        self.db_path = Some(path);
        self
    }

    /// Use an in-memory SQLite database (ideal for tests).
    pub fn with_in_memory_db(mut self) -> Self {
        self.in_memory_db = true;
        self
    }

    /// Override the event bus instance.
    pub fn with_event_bus(mut self, bus: Arc<EventBus>) -> Self {
        self.event_bus = Some(bus);
        self
    }

    /// Override the platform detector instance.
    pub fn with_platform(mut self, platform: Arc<PlatformDetector>) -> Self {
        self.platform = Some(platform);
        self
    }

    /// Override the plugin host instance.
    pub fn with_plugin_host(mut self, plugin_host: Arc<PluginHost>) -> Self {
        self.plugin_host = Some(plugin_host);
        self
    }

    /// Build and wire up the complete application dependency container.
    pub async fn build(self) -> plaza_core::PlazaResult<Container> {
        plaza_core::paths::ensure_directories()?;

        // 1. Storage / Repository
        let repo = if self.in_memory_db {
            SqliteWorkspaceRepository::open_in_memory()?
        } else {
            let db_path = self
                .db_path
                .unwrap_or_else(plaza_core::paths::database_path);
            SqliteWorkspaceRepository::open(db_path)?
        };

        // 2. Foundation Services
        let event_bus = self.event_bus.unwrap_or_else(|| Arc::new(EventBus::new()));
        let platform = self
            .platform
            .unwrap_or_else(|| Arc::new(PlatformDetector::new()));

        // Perform initial platform scan if not already cached
        if platform.capabilities().await.is_err() {
            platform.scan().await?;
        }

        // 3. Domain & Infrastructure Services
        let workspace_service = Arc::new(WorkspaceService::new(repo.clone(), event_bus.clone()));
        let resource_manager = Arc::new(ResourceManager::new(platform.clone(), event_bus.clone()));

        let plugin_dir = plaza_core::paths::plugin_dir();
        let plugin_host = self
            .plugin_host
            .unwrap_or_else(|| Arc::new(PluginHost::new(event_bus.clone(), plugin_dir)));

        // 4. Decision & Controller Services
        let decision_engine = Arc::new(DecisionEngine::new(platform.clone(), plugin_host.clone()));
        let controller = Arc::new(WorkspaceController::new(
            workspace_service.clone(),
            decision_engine.clone(),
            resource_manager.clone(),
            plugin_host.clone(),
            event_bus.clone(),
        ));

        // 5. System Monitor & Registries
        let monitor = Arc::new(SystemMonitor::new(event_bus.clone()));
        let image_registry = Arc::new(RuntimeImageRegistry::new());
        let template_registry = Arc::new(WorkspaceTemplateRegistry::new());

        Ok(Container {
            event_bus,
            platform,
            repo,
            workspace_service,
            resource_manager,
            plugin_host,
            decision_engine,
            controller,
            monitor,
            image_registry,
            template_registry,
        })
    }
}

impl Default for BootstrapBuilder {
    fn default() -> Self {
        Self::new()
    }
}
