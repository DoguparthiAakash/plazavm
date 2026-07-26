//! The core runtime backend trait.
//!
//! This is the **only** interface the PlazaVM core system uses to interact
//! with execution backends. Every backend (Docker, QEMU, VirtualBox, etc.)
//! implements this trait via a plugin crate.

use async_trait::async_trait;
use plaza_foundation::core::PlazaResult;

use crate::capabilities::RuntimeCapabilities;
use crate::instance::{
    ConsoleStream, RuntimeInstance, RuntimeMetrics, RuntimeStatus, SnapshotInfo,
};

/// The contract every execution backend must implement.
///
/// # Design
///
/// - **Mandatory methods**: `create`, `start`, `stop`, `force_stop`, `destroy`,
///   `status`, `metrics`, `is_available`, `version`.
/// - **Optional methods**: Have default implementations that return
///   [`CapabilityNotSupported`](plaza_foundation::core::PlazaError::CapabilityNotSupported).
///   Override only what the backend actually supports.
/// - **Capability discovery**: Call [`capabilities()`](Self::capabilities) to
///   check what a backend supports before invoking optional methods.
#[async_trait]
pub trait RuntimeBackend: Send + Sync {
    /// Unique identifier for this backend type (e.g., `"docker"`, `"qemu"`).
    fn id(&self) -> &str;

    /// Human-readable display name (e.g., `"Docker Engine"`).
    fn display_name(&self) -> &str;

    /// Declare what this backend can do.
    fn capabilities(&self) -> RuntimeCapabilities;

    /// Check if this backend is available on the current host.
    async fn is_available(&self) -> bool;

    /// Get the installed version string.
    async fn version(&self) -> PlazaResult<String>;

    // ── Mandatory lifecycle ─────────────────────────────────────────────────

    /// Provision a new runtime instance from a workspace spec.
    ///
    /// The `spec_json` parameter is a JSON-serialized `WorkspaceSpec`.
    /// We use JSON here to avoid making `plaza-runtime` depend on
    /// `plaza-workspace`, keeping the dependency graph acyclic.
    async fn create(&self, spec_json: &str) -> PlazaResult<RuntimeInstance>;

    /// Start a previously created instance.
    async fn start(&self, instance_id: &str) -> PlazaResult<()>;

    /// Gracefully stop a running instance.
    async fn stop(&self, instance_id: &str) -> PlazaResult<()>;

    /// Forcefully terminate a running instance.
    async fn force_stop(&self, instance_id: &str) -> PlazaResult<()>;

    /// Destroy an instance and release all associated resources.
    async fn destroy(&self, instance_id: &str) -> PlazaResult<()>;

    // ── Mandatory inspection ────────────────────────────────────────────────

    /// Get the current status of an instance.
    async fn status(&self, instance_id: &str) -> PlazaResult<RuntimeStatus>;

    /// Get resource usage metrics for an instance.
    async fn metrics(&self, instance_id: &str) -> PlazaResult<RuntimeMetrics>;

    // ── Optional lifecycle (default = unsupported) ──────────────────────────

    /// Pause a running instance.
    async fn pause(&self, _instance_id: &str) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::CapabilityNotSupported {
            capability: "pause".into(),
            backend: self.id().into(),
        })
    }

    /// Resume a paused instance.
    async fn resume(&self, _instance_id: &str) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::CapabilityNotSupported {
            capability: "resume".into(),
            backend: self.id().into(),
        })
    }

    // ── Optional snapshots ──────────────────────────────────────────────────

    /// Create a named snapshot.
    async fn snapshot_create(&self, _instance_id: &str, _tag: &str) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::CapabilityNotSupported {
            capability: "snapshots".into(),
            backend: self.id().into(),
        })
    }

    /// Restore a named snapshot.
    async fn snapshot_restore(&self, _instance_id: &str, _tag: &str) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::CapabilityNotSupported {
            capability: "snapshots".into(),
            backend: self.id().into(),
        })
    }

    /// Delete a named snapshot.
    async fn snapshot_delete(&self, _instance_id: &str, _tag: &str) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::CapabilityNotSupported {
            capability: "snapshots".into(),
            backend: self.id().into(),
        })
    }

    /// List all snapshots for an instance.
    async fn snapshot_list(&self, _instance_id: &str) -> PlazaResult<Vec<SnapshotInfo>> {
        Ok(vec![])
    }

    // ── Optional console ────────────────────────────────────────────────────

    /// Attach an interactive console to a running instance.
    async fn attach_console(&self, _instance_id: &str) -> PlazaResult<ConsoleStream> {
        Err(plaza_foundation::core::PlazaError::CapabilityNotSupported {
            capability: "console".into(),
            backend: self.id().into(),
        })
    }
}

