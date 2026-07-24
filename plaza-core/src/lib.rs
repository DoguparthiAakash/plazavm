//! # plaza-core
//!
//! Foundation crate for the PlazaVM workspace platform.
//!
//! Provides shared types, error definitions, strongly-typed identifiers,
//! and cross-cutting concerns used by every other crate in the workspace.

pub mod error;
pub mod id;
pub mod paths;
pub mod security;
pub mod types;

// ── Convenience re-exports ──────────────────────────────────────────────────

pub use error::{PlazaError, PlazaResult};
pub use id::{PluginId, RuntimeId, WorkspaceId};
pub use types::{Architecture, HealthStatus, OperatingSystem, Timestamp};
