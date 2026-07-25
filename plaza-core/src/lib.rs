//! # plaza-core
//!
//! Foundation crate for the PlazaVM workspace platform.
//!
//! Provides shared types, error definitions, strongly-typed identifiers,
//! and cross-cutting concerns used by every other crate in the workspace.

pub mod error;
pub mod id;
pub mod logging;
pub mod object_model;
pub mod panic_handler;
pub mod paths;
pub mod puri;
pub mod security;
pub mod types;

// ── Convenience re-exports ──────────────────────────────────────────────────

pub use error::{CanonicalError, ErrorSeverity, PlazaError, PlazaResult};
pub use id::{
    BackendId, DriverId, ExecutionTarget, ExecutionTargetId, ImageId, MountId, PackageName,
    PluginId, RuntimeBackendKind, RuntimeId, ServiceId, SessionId, WorkspaceId,
};
pub use object_model::PlazaObjectHeader;
pub use puri::PlazaUri;
pub use types::{Architecture, HealthStatus, OperatingSystem, Timestamp};
