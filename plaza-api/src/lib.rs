//! # plaza-api
//!
//! Application layer state, DTO contracts, and bootstrap composition root.

pub mod bootstrap;
pub mod dto;
pub mod state;

pub use bootstrap::{BootstrapBuilder, Container};
pub use dto::{CreateWorkspaceRequest, WorkspaceDto};
pub use state::AppState;
