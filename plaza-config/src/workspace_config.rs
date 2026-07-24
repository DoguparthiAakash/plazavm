//! Workspace configuration schema parser for `plaza.yaml`.

use plaza_core::security::SecurityPolicy;
use plaza_core::types::{Architecture, OperatingSystem};
use plaza_core::{PlazaError, PlazaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Version string for `plaza.yaml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WorkspaceConfigVersion {
    #[default]
    #[serde(rename = "1")]
    V1,
}

/// Parsed `plaza.yaml` workspace configuration document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    #[serde(default)]
    pub version: WorkspaceConfigVersion,
    pub workspace: WorkspaceMetadataConfig,
    pub runtime: RuntimeConfigSection,
    #[serde(default)]
    pub resources: ResourceConfigSection,
    #[serde(default)]
    pub networking: NetworkConfigSection,
    #[serde(default)]
    pub storage: StorageConfigSection,
    #[serde(default)]
    pub environment: HashMap<String, String>,
    #[serde(default)]
    pub security: SecurityPolicy,
    #[serde(default)]
    pub intent: Option<IntentConfig>,
    #[serde(default)]
    pub extensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMetadataConfig {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfigSection {
    pub kind: String, // container, microvm, vm, remote, cloud
    pub image: Option<String>,
    #[serde(default = "default_backend_auto")]
    pub backend: String, // auto, preferred:docker, pinned:qemu
    #[serde(default = "default_os_linux")]
    pub os: OperatingSystem,
    #[serde(default = "default_arch_x86")]
    pub arch: Architecture,
}

fn default_backend_auto() -> String {
    "auto".into()
}
fn default_os_linux() -> OperatingSystem {
    OperatingSystem::Linux
}
fn default_arch_x86() -> Architecture {
    Architecture::X86_64
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceConfigSection {
    pub cpu: Option<CpuResourceConfig>,
    pub memory: Option<MemoryResourceConfig>,
    pub gpu: Option<GpuResourceConfig>,
    #[serde(default = "default_priority")]
    pub priority: String,
}

fn default_priority() -> String {
    "normal".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuResourceConfig {
    pub cores: u32,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryResourceConfig {
    pub size: String,
    pub limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuResourceConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkConfigSection {
    #[serde(default = "default_net_mode")]
    pub mode: String,
    #[serde(default)]
    pub ports: Vec<PortMappingConfig>,
    #[serde(default)]
    pub dns: Vec<String>,
}

fn default_net_mode() -> String {
    "nat".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMappingConfig {
    pub host: u16,
    pub guest: u16,
    #[serde(default = "default_protocol_tcp")]
    pub protocol: String,
}

fn default_protocol_tcp() -> String {
    "tcp".into()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageConfigSection {
    #[serde(default)]
    pub volumes: Vec<VolumeConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeConfig {
    pub name: String,
    pub host_path: Option<String>,
    pub mount_path: String,
    pub size: Option<String>,
}

/// Intent-based high-level configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentConfig {
    pub purpose: Option<String>,
    pub performance: Option<String>, // low, medium, high, maximum
    pub startup: Option<String>,     // fast, normal, thorough
    pub gpu: Option<String>,         // required, preferred, none
    pub security: Option<String>,    // minimal, standard, strict
}

impl WorkspaceConfig {
    /// Parse `plaza.yaml` string.
    pub fn parse_yaml(content: &str) -> PlazaResult<Self> {
        serde_yaml::from_str(content).map_err(|e| PlazaError::Config(e.to_string()))
    }

    /// Validate the workspace configuration.
    pub fn validate(&self) -> PlazaResult<()> {
        if self.workspace.name.trim().is_empty() {
            return Err(PlazaError::Config("workspace.name cannot be empty".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sample_plaza_yaml() {
        let yaml = r#"
version: "1"
workspace:
  name: "my-dev-env"
  description: "Development workspace"
runtime:
  kind: container
  image: "ubuntu:24.04"
  backend: auto
resources:
  cpu:
    cores: 4
  memory:
    size: "4Gi"
intent:
  purpose: "AI Development"
  performance: "high"
"#;
        let config = WorkspaceConfig::parse_yaml(yaml).unwrap();
        assert_eq!(config.workspace.name, "my-dev-env");
        assert_eq!(config.runtime.kind, "container");
        assert!(config.intent.is_some());
        config.validate().unwrap();
    }
}
