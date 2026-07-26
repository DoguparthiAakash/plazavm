//! Runtime instance types — returned by backends after creation.

use plaza_foundation::core::types::Timestamp;
use serde::{Deserialize, Serialize};

/// A running (or stopped) runtime instance managed by a backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeInstance {
    /// Backend-specific instance identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Current status.
    pub status: RuntimeStatus,
    /// When the instance was created.
    pub created_at: Timestamp,
}

/// Runtime instance lifecycle status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeStatus {
    Creating,
    Starting,
    Running,
    Paused,
    Stopping,
    Stopped,
    Error,
    Unknown,
}

impl std::fmt::Display for RuntimeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Creating => write!(f, "creating"),
            Self::Starting => write!(f, "starting"),
            Self::Running => write!(f, "running"),
            Self::Paused => write!(f, "paused"),
            Self::Stopping => write!(f, "stopping"),
            Self::Stopped => write!(f, "stopped"),
            Self::Error => write!(f, "error"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// Resource usage metrics for a runtime instance.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuntimeMetrics {
    /// CPU usage as a percentage (0.0–100.0).
    pub cpu_usage_pct: f64,
    /// Memory used in bytes.
    pub memory_used_bytes: u64,
    /// Memory allocated in bytes.
    pub memory_total_bytes: u64,
    /// Disk read bytes since start.
    pub disk_read_bytes: u64,
    /// Disk write bytes since start.
    pub disk_write_bytes: u64,
    /// Network received bytes since start.
    pub network_rx_bytes: u64,
    /// Network transmitted bytes since start.
    pub network_tx_bytes: u64,
}

/// Information about a stored snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    /// Snapshot identifier.
    pub id: String,
    /// Human-readable tag.
    pub tag: String,
    /// When the snapshot was created.
    pub created_at: Timestamp,
    /// Size in bytes (if known).
    pub size_bytes: Option<u64>,
}

/// A handle to an interactive console stream.
///
/// Phase 1 stub — will be backed by tokio channels in Phase 2.
pub struct ConsoleStream {
    _private: (),
}

impl ConsoleStream {
    /// Create a placeholder console stream.
    pub fn placeholder() -> Self {
        Self { _private: () }
    }
}

