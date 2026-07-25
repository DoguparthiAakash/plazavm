use thiserror::Error;

/// Canonical Plaza Foundation Engine (PFE) Error Types.
#[derive(Error, Debug)]
pub enum PfeError {
    #[error("[PZE-1001] Engine Initialization Failed: {0}")]
    InitializationFailed(String),

    #[error("[PZE-1002] Invalid Lifecycle Transition from {from} to {to}")]
    InvalidLifecycleTransition { from: String, to: String },

    #[error("[PZE-2001] Cyclic Service Dependency Detected: {0}")]
    CyclicDependency(String),

    #[error("[PZE-2002] Service Not Found: {0}")]
    ServiceNotFound(String),

    #[error("[PZE-3001] Execution Scheduler Error: {0}")]
    SchedulerError(String),

    #[error("[PZE-4001] Coordinator Delegation Error ({coordinator}): {message}")]
    CoordinatorError { coordinator: String, message: String },

    #[error("[PZE-5001] Diagnostics Engine Failed: {0}")]
    DiagnosticsFailed(String),

    #[error("[PZE-6001] Recovery Engine Transaction Failed: {0}")]
    RecoveryFailed(String),
}

pub type PfeResult<T> = Result<T, PfeError>;
