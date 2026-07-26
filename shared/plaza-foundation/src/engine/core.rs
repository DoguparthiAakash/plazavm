use crate::engine::bootstrap::BootstrapSequence;
use crate::engine::diagnostics::PfeDiagnosticsEngine;
use crate::engine::errors::PfeResult;
use crate::engine::health::EngineHealthMonitor;
use crate::engine::lifecycle::{EngineLifecycle, EngineLifecycleState};
use crate::engine::recovery::RecoveryEngine;
use crate::engine::registry::ServiceRegistry;
use crate::engine::scheduler::ExecutionScheduler;
use std::sync::Arc;

/// Central Plaza Foundation Engine (PFE) Orchestrator.
pub struct EngineCore {
    pub lifecycle: Arc<EngineLifecycle>,
    pub registry: Arc<ServiceRegistry>,
    pub scheduler: Arc<ExecutionScheduler>,
    pub health_monitor: Arc<EngineHealthMonitor>,
    pub diagnostics: Arc<PfeDiagnosticsEngine>,
    pub recovery: Arc<RecoveryEngine>,
}

impl EngineCore {
    pub async fn boot() -> PfeResult<Self> {
        let container = BootstrapSequence::build_container().await?;

        let lifecycle: Arc<EngineLifecycle> = container.resolve()?;
        lifecycle.transition_to(EngineLifecycleState::Initializing)?;

        let registry: Arc<ServiceRegistry> = container.resolve()?;
        let scheduler: Arc<ExecutionScheduler> = container.resolve()?;
        let health_monitor: Arc<EngineHealthMonitor> = container.resolve()?;
        let diagnostics: Arc<PfeDiagnosticsEngine> = container.resolve()?;
        let recovery: Arc<RecoveryEngine> = container.resolve()?;

        BootstrapSequence::run_checks().await?;
        lifecycle.transition_to(EngineLifecycleState::Discovering)?;

        // Validate service registry graphs
        registry.validate_cycles()?;

        lifecycle.transition_to(EngineLifecycleState::Ready)?;
        lifecycle.transition_to(EngineLifecycleState::Running)?;

        tracing::info!("Plaza Foundation Engine (PFE) v1.0 Boot Complete");

        Ok(Self {
            lifecycle,
            registry,
            scheduler,
            health_monitor,
            diagnostics,
            recovery,
        })
    }

    pub async fn shutdown(&self) -> PfeResult<()> {
        self.lifecycle
            .transition_to(EngineLifecycleState::Stopping)?;
        self.lifecycle
            .transition_to(EngineLifecycleState::Stopped)?;
        tracing::info!("Plaza Foundation Engine (PFE) Shutdown Complete");
        Ok(())
    }
}
