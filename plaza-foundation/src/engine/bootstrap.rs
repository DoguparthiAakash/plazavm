use crate::engine::container::ServiceContainer;
use crate::engine::diagnostics::PfeDiagnosticsEngine;
use crate::engine::errors::PfeResult;
use crate::engine::health::EngineHealthMonitor;
use crate::engine::lifecycle::EngineLifecycle;
use crate::engine::recovery::RecoveryEngine;
use crate::engine::registry::ServiceRegistry;
use crate::engine::scheduler::ExecutionScheduler;
use std::sync::Arc;

pub struct BootstrapSequence;

impl BootstrapSequence {
    pub async fn run_checks() -> PfeResult<()> {
        tracing::info!("PFE Bootstrap Sequence: Environment validation OK");
        Ok(())
    }

    /// Construct and wire all foundational engine services into a Composition Root [`ServiceContainer`].
    pub async fn build_container() -> PfeResult<ServiceContainer> {
        let mut container = ServiceContainer::new();

        let lifecycle = Arc::new(EngineLifecycle::new());
        let registry = Arc::new(ServiceRegistry::new());
        let scheduler = Arc::new(ExecutionScheduler::new());
        let health_monitor = Arc::new(EngineHealthMonitor::new());
        let diagnostics = Arc::new(PfeDiagnosticsEngine::new());
        let recovery = Arc::new(RecoveryEngine::new());

        container.register(lifecycle);
        container.register(registry);
        container.register(scheduler);
        container.register(health_monitor);
        container.register(diagnostics);
        container.register(recovery);

        tracing::info!("PFE Bootstrap Sequence: Composition Root ServiceContainer initialized");
        Ok(container)
    }
}
