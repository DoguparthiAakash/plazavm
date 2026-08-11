//! Root `plaza.yaml` configuration document.
//!
//! This module defines `PlazaYaml` — the top-level struct that represents
//! a complete, parsed `plaza.yaml` workspace configuration file.
//!
//! # Schema
//!
//! ```yaml
//! version: "1"
//!
//! workspace:
//!   name: my-project
//!
//! image:
//!   name: alpine-dev
//!   version: "1.0"
//!
//! machine:
//!   architecture: x86_64
//!   cpu:
//!     cores: 4
//!   memory:
//!     size: 4096MiB
//!
//! runtime:
//!   backend: auto
//!   acceleration:
//!     enabled: false
//!
//! capabilities:
//!   filesystem:
//!     - path: "./project"
//!       mode: read-write
//!   network:
//!     enabled: true
//!     mode: nat
//! ```

use crate::config::capabilities::CapabilityGrants;
use crate::config::image_section::ImageSection;
use crate::config::machine_section::MachineSection;
use crate::config::runtime_section::RuntimeSection;
use crate::core::{PlazaError, PlazaResult};
use serde::{Deserialize, Serialize};

/// Schema version for `plaza.yaml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PlazaYamlVersion {
    #[default]
    #[serde(rename = "1")]
    V1,
}

/// Parsed `plaza.yaml` workspace configuration document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlazaYaml {
    /// Schema version (currently `"1"`).
    #[serde(default)]
    pub version: PlazaYamlVersion,

    /// Workspace identity.
    pub workspace: WorkspaceSection,

    /// Image reference (name + optional version).
    #[serde(default)]
    pub image: Option<ImageSection>,

    /// Virtual machine hardware configuration.
    #[serde(default)]
    pub machine: Option<MachineSection>,

    /// Runtime backend selection and acceleration policy.
    #[serde(default)]
    pub runtime: Option<RuntimeSection>,

    /// Explicit capability grants. Default-deny: missing = DENIED.
    #[serde(default)]
    pub capabilities: Option<CapabilityGrants>,
}

/// The `workspace:` section of `plaza.yaml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSection {
    /// Workspace name.
    pub name: String,

    /// Optional description.
    #[serde(default)]
    pub description: Option<String>,
}

impl PlazaYaml {
    /// Parse a `plaza.yaml` string into a `PlazaYaml` struct.
    pub fn parse_yaml(content: &str) -> PlazaResult<Self> {
        serde_yaml::from_str(content).map_err(|e| PlazaError::Config(e.to_string()))
    }

    /// Validate the configuration using the full validation pipeline.
    pub fn validate(&self) -> PlazaResult<()> {
        crate::config::validation::validate_plaza_yaml(self)
    }

    /// Generate a minimal secure `plaza.yaml` for `plaza init`.
    ///
    /// The generated configuration follows the default-deny model:
    /// no capabilities are granted.
    pub fn generate_minimal(name: &str) -> String {
        format!(
            r#"version: "1"

workspace:
  name: {}

image:
  name: alpine-dev

machine:
  architecture: x86_64
  cpu:
    cores: 2
  memory:
    size: 2048MiB

runtime:
  backend: auto
"#,
            name
        )
    }
}

// ── Backward compatibility re-exports ───────────────────────────────────────
// The old `WorkspaceConfig` name is preserved as a type alias so that
// downstream code that hasn't migrated yet continues to compile.

/// Deprecated alias — use `PlazaYaml` directly.
pub type WorkspaceConfig = PlazaYaml;

/// Deprecated alias — use `PlazaYamlVersion` directly.
pub type WorkspaceConfigVersion = PlazaYamlVersion;

// ── Legacy IntentConfig ─────────────────────────────────────────────────────
// Kept for backward compatibility with downstream crates that import it.

/// Intent-based high-level configuration (legacy).
///
/// This is preserved for backward compatibility. New code should use
/// the `machine` and `runtime` sections of `plaza.yaml` instead.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentConfig {
    pub purpose: Option<String>,
    pub performance: Option<String>,
    pub startup: Option<String>,
    pub gpu: Option<String>,
    pub security: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_plaza_yaml() {
        let yaml = r#"
version: "1"
workspace:
  name: my-project
"#;
        let config = PlazaYaml::parse_yaml(yaml).unwrap();
        assert_eq!(config.workspace.name, "my-project");
        assert!(config.image.is_none());
        assert!(config.machine.is_none());
        assert!(config.runtime.is_none());
        assert!(config.capabilities.is_none());
        config.validate().unwrap();
    }

    #[test]
    fn parse_full_plaza_yaml() {
        let yaml = r#"
version: "1"

workspace:
  name: alpine-dev
  description: "Full development workspace"

image:
  name: alpine-dev
  version: "1.0"

machine:
  architecture: x86_64
  cpu:
    cores: 4
  memory:
    size: 4096MiB
  display:
    enabled: true
  console:
    serial: true

runtime:
  backend: auto
  acceleration:
    enabled: false

capabilities:
  filesystem:
    - path: "./project"
      mode: read-write
  network:
    enabled: true
    mode: nat
  clipboard:
    read: true
    write: true
"#;
        let config = PlazaYaml::parse_yaml(yaml).unwrap();
        assert_eq!(config.workspace.name, "alpine-dev");

        let image = config.image.as_ref().unwrap();
        assert_eq!(image.name, "alpine-dev");
        assert_eq!(image.version.as_deref(), Some("1.0"));

        let machine = config.machine.as_ref().unwrap();
        assert_eq!(machine.cpu.cores, 4);
        assert_eq!(machine.memory.size, "4096MiB");

        let runtime = config.runtime.as_ref().unwrap();
        assert_eq!(runtime.backend, "auto");
        assert!(!runtime.acceleration.enabled);

        let caps = config.capabilities.as_ref().unwrap();
        assert!(caps.has_filesystem());
        assert!(caps.has_network());
        assert!(caps.has_clipboard());

        config.validate().unwrap();
    }

    #[test]
    fn generate_minimal_follows_default_deny() {
        let yaml = PlazaYaml::generate_minimal("test-project");
        let config = PlazaYaml::parse_yaml(&yaml).unwrap();
        assert_eq!(config.workspace.name, "test-project");
        // No capabilities section → everything denied.
        assert!(config.capabilities.is_none());
        config.validate().unwrap();
    }

    #[test]
    fn missing_capabilities_is_default_deny() {
        let yaml = r#"
version: "1"
workspace:
  name: test
"#;
        let config = PlazaYaml::parse_yaml(yaml).unwrap();
        let caps = config
            .capabilities
            .as_ref()
            .cloned()
            .unwrap_or_default();
        assert!(!caps.has_filesystem());
        assert!(!caps.has_network());
        assert!(!caps.has_clipboard());
        assert!(!caps.has_any_device());
    }

    #[test]
    fn unknown_schema_version_rejected() {
        let yaml = r#"
version: "999"
workspace:
  name: test
"#;
        let result = PlazaYaml::parse_yaml(yaml);
        assert!(result.is_err());
    }
}
