//! # plaza-config
//!
//! Configuration parsing, schema versioning, and validation.
//!
//! Supports `plaza.yaml` (workspace definition files) and `plaza.toml`
//! (application system configuration).

pub mod app_config;
pub mod capabilities;
pub mod engine_section;
pub mod image_section;
pub mod machine_section;
pub mod manager;
pub mod project_section;
pub mod runtime_section;
pub mod validation;
pub mod workspace_config;

pub use app_config::PlazaConfig;
pub use capabilities::CapabilityGrants;
pub use manager::ConfigManager;
pub use workspace_config::{
    IntentConfig, PlazaYaml, PlazaYamlVersion, WorkspaceConfig, WorkspaceConfigVersion,
};
