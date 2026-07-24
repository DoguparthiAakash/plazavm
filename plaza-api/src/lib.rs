//! # plaza-api
//!
//! Application layer state, DTO contracts, and bootstrap composition root.

pub mod bootstrap;
pub mod diagnostics;
pub mod dto;
pub mod state;
pub mod updater;

pub use bootstrap::{BootstrapBuilder, Container};
pub use diagnostics::DiagnosticsBundle;
pub use dto::{CreateWorkspaceRequest, WorkspaceDto};
pub use state::AppState;
pub use updater::{UpdateChannel, UpdateService, VersionCheckResult};
