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
    /// PID of the execution engine.
    pub pid: Option<u32>,
    /// Uptime in seconds.
    pub uptime_secs: Option<u64>,
    /// Execution mode (e.g. TCG, KVM, WASM).
    pub execution_mode: Option<String>,
    /// Storage backend details.
    pub storage_backend: Option<String>,
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
/// Provides bidirectional communication between the host and a guest VM
/// via two tokio mpsc channels — one for sending data *to* the guest,
/// and one for receiving data *from* the guest.
pub struct ConsoleStream {
    /// Send data (commands) to the guest.
    pub tx: tokio::sync::mpsc::Sender<String>,
    /// Receive data (output) from the guest.
    pub rx: tokio::sync::Mutex<tokio::sync::mpsc::Receiver<String>>,
}

impl ConsoleStream {
    /// Create a new console stream from pre-existing channel halves.
    pub fn new(
        tx: tokio::sync::mpsc::Sender<String>,
        rx: tokio::sync::mpsc::Receiver<String>,
    ) -> Self {
        Self {
            tx,
            rx: tokio::sync::Mutex::new(rx),
        }
    }

    /// Create a placeholder console stream (returns None for both channels).
    /// Kept for backward compatibility with backends that don't support console.
    pub fn placeholder() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        Self {
            tx,
            rx: tokio::sync::Mutex::new(rx),
        }
    }

    /// Send a command string to the guest.
    pub async fn send(
        &self,
        data: String,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<String>> {
        self.tx.send(data).await
    }

    /// Receive the next line of output from the guest.
    /// Returns `None` if the guest has disconnected.
    pub async fn recv(&self) -> Option<String> {
        self.rx.lock().await.recv().await
    }
}
