//! # plaza-config
//!
//! Configuration parsing, schema versioning, and validation.
//!
//! Supports `plaza.yaml` (workspace definition files) and `plaza.toml`
//! (application system configuration).

pub mod app_config;
pub mod manager;
pub mod workspace_config;

pub use app_config::PlazaConfig;
pub use manager::ConfigManager;
pub use workspace_config::{IntentConfig, WorkspaceConfig, WorkspaceConfigVersion};
