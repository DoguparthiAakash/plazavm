//! Unified error types for the entire PlazaVM platform.

use crate::core::id::WorkspaceId;

/// The unified error type used across all PlazaVM crates.
#[derive(Debug, thiserror::Error)]
pub enum PlazaError {
    // ── Workspace errors ────────────────────────────────────────────────────
    #[error("workspace not found: {0}")]
    WorkspaceNotFound(WorkspaceId),

    #[error("workspace already exists: {0}")]
    WorkspaceAlreadyExists(String),

    #[error("invalid workspace state transition: {from} → {to}")]
    InvalidStateTransition { from: String, to: String },

    // ── Runtime errors ──────────────────────────────────────────────────────
    #[error("runtime unavailable: {0}")]
    RuntimeUnavailable(String),

    #[error("no suitable runtime for workspace: {reason}")]
    NoSuitableRuntime { reason: String },

    #[error("capability not supported: {capability} by backend {backend}")]
    CapabilityNotSupported { capability: String, backend: String },

    // ── Resource errors ─────────────────────────────────────────────────────
    #[error("resource exhausted: {resource}")]
    ResourceExhausted { resource: String },

    // ── Decision errors ─────────────────────────────────────────────────────
    #[error("decision rejected: {0}")]
    DecisionRejected(String),

    // ── Plugin errors ───────────────────────────────────────────────────────
    #[error("plugin error [{plugin}]: {message}")]
    Plugin { plugin: String, message: String },

    #[error("plugin not found: {0}")]
    PluginNotFound(String),

    // ── Platform errors ─────────────────────────────────────────────────────
    #[error("platform detection failed: {0}")]
    PlatformDetection(String),

    // ── Configuration errors ────────────────────────────────────────────────
    #[error("configuration error: {0}")]
    Config(String),

    // ── Storage errors ──────────────────────────────────────────────────────
    #[error("storage error: {0}")]
    Storage(String),

    // ── AI errors ───────────────────────────────────────────────────────────
    #[error("AI error: {0}")]
    Ai(String),

    // ── Infrastructure errors ───────────────────────────────────────────────
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serialization(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Convenience result type alias used throughout PlazaVM.
pub type PlazaResult<T> = Result<T, PlazaError>;

impl PlazaError {
    /// Create a config error from any displayable value.
    pub fn config(msg: impl std::fmt::Display) -> Self {
        Self::Config(msg.to_string())
    }

    /// Create a storage error from any displayable value.
    pub fn storage(msg: impl std::fmt::Display) -> Self {
        Self::Storage(msg.to_string())
    }

    /// Create a serialization error from any displayable value.
    pub fn serialization(msg: impl std::fmt::Display) -> Self {
        Self::Serialization(msg.to_string())
    }

    /// Returns the canonical error details (code, category, severity, resolution).
    pub fn canonical(&self) -> CanonicalError {
        match self {
            Self::WorkspaceNotFound(id) => CanonicalError {
                code: "PZE-3001".into(),
                category: "Workspace".into(),
                message: format!("Workspace not found: {}", id),
                severity: ErrorSeverity::Error,
                recoverable: false,
                resolution: "Verify workspace ID via 'plaza workspace list'".into(),
                correlation_id: uuid::Uuid::new_v4().to_string(),
            },
            Self::Config(msg) => CanonicalError {
                code: "PZE-2001".into(),
                category: "Config".into(),
                message: msg.clone(),
                severity: ErrorSeverity::Error,
                recoverable: true,
                resolution: "Check plaza.yaml syntax and configuration parameters".into(),
                correlation_id: uuid::Uuid::new_v4().to_string(),
            },
            _ => CanonicalError {
                code: "PZE-1001".into(),
                category: "Core".into(),
                message: self.to_string(),
                severity: ErrorSeverity::Error,
                recoverable: false,
                resolution: "Check application logs in ~/.plazavm/logs/plazavm.log".into(),
                correlation_id: uuid::Uuid::new_v4().to_string(),
            },
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ErrorSeverity {
    Fatal,
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CanonicalError {
    pub code: String,
    pub category: String,
    pub message: String,
    pub severity: ErrorSeverity,
    pub recoverable: bool,
    pub resolution: String,
    pub correlation_id: String,
}


