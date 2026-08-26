//! # plaza-api
//!
//! Application layer state, DTO contracts, and bootstrap composition root.

pub mod auth;
pub mod bootstrap;
pub mod diagnostics;
pub mod dto;
pub mod openapi;
pub mod rate_limit;
pub mod router;
pub mod state;
pub mod updater;
pub mod websocket;

pub use bootstrap::{BootstrapBuilder, Container};
pub use diagnostics::DiagnosticsBundle;
pub use dto::{CreateWorkspaceRequest, WorkspaceDto};
pub use state::AppState;
pub use updater::{UpdateChannel, UpdateService, VersionCheckResult};
