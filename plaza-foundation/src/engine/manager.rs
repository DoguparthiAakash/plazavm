use crate::engine::errors::PfeResult;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Standard interface for all major PlazaVM subsystems (Workspace Engine, Storage Engine, etc.)
#[async_trait]
pub trait Engine: Send + Sync {
    fn name(&self) -> &'static str;
    fn dependencies(&self) -> Vec<&'static str>;
    
    async fn initialize(&self) -> PfeResult<()>;
    async fn start(&self) -> PfeResult<()>;
    async fn stop(&self) -> PfeResult<()>;
    async fn restart(&self) -> PfeResult<()>;
    async fn reload(&self) -> PfeResult<()>;
    async fn recover(&self) -> PfeResult<()>;
    async fn shutdown(&self) -> PfeResult<()>;
    
    async fn health(&self) -> PfeResult<String>;
    async fn metrics(&self) -> PfeResult<HashMap<String, String>>;
    async fn diagnostics(&self) -> PfeResult<Vec<String>>;
    async fn status(&self) -> PfeResult<String>;

    // Registration Hooks
    fn register_commands(&self, _registry: &mut plaza_command::registry::CommandRegistry) {}
    fn register_services(&self, _registry: &mut crate::engine::registry::ServiceRegistry) {}
    fn register_events(&self) {}
    fn register_metrics(&self) {}
    fn register_configuration(&self) {}
}

/// Manages lifecycle, discovery, and dependency ordering for all registered Engines.
pub struct EngineManager {
    engines: RwLock<HashMap<&'static str, Arc<dyn Engine>>>,
}

impl EngineManager {
    pub fn new() -> Self {
        Self {
            engines: RwLock::new(HashMap::new()),
        }
    }

    /// Registers an engine with the manager.
    pub async fn register(&self, engine: Arc<dyn Engine>) {
        let mut reg = self.engines.write().await;
        reg.insert(engine.name(), engine);
    }

    /// Starts all registered engines in an orderly fashion.
    /// (In a complete implementation, this performs topological sort based on `dependencies()`).
    pub async fn start_all(&self) -> PfeResult<()> {
        let reg = self.engines.read().await;
        
        info!("EngineManager: Initializing all engines...");
        for (name, engine) in reg.iter() {
            engine.initialize().await?;
            info!("Engine '{}' initialized", name);
        }

        info!("EngineManager: Starting all engines...");
        for (name, engine) in reg.iter() {
            engine.start().await?;
            info!("Engine '{}' started", name);
        }

        Ok(())
    }

    /// Invokes register_commands on all registered engines
    pub async fn invoke_command_registration(&self, registry: &mut plaza_command::registry::CommandRegistry) {
        let reg = self.engines.read().await;
        for engine in reg.values() {
            engine.register_commands(registry);
        }
    }

    /// Invokes register_services on all registered engines
    pub async fn invoke_service_registration(&self, registry: &mut crate::engine::registry::ServiceRegistry) {
        let reg = self.engines.read().await;
        for engine in reg.values() {
            engine.register_services(registry);
        }
    }

    /// Stops all registered engines.
    pub async fn stop_all(&self) -> PfeResult<()> {
        let reg = self.engines.read().await;
        info!("EngineManager: Stopping all engines...");
        for (name, engine) in reg.iter() {
            // We ignore errors during mass stop to ensure we try stopping everything
            if let Err(e) = engine.stop().await {
                warn!("Error stopping engine '{}': {}", name, e);
            } else {
                info!("Engine '{}' stopped", name);
            }
        }
        Ok(())
    }
}
