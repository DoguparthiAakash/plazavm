//! Plugin manifest — parsed from `plugin.toml` in each plugin directory.

use serde::{Deserialize, Serialize};

/// Plugin manifest describing a PlazaVM plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Unique plugin identifier (e.g., `"docker"`, `"virtualbox"`).
    pub id: String,
    /// Human-readable name (e.g., `"Docker Runtime"`).
    pub name: String,
    /// Plugin version.
    pub version: semver::Version,
    /// Short description.
    pub description: String,
    /// Author name.
    pub author: String,
    /// License identifier.
    pub license: Option<String>,
    /// What kind of plugin this is.
    pub plugin_type: PluginType,
    /// Minimum PlazaVM version required.
    pub min_plaza_version: Option<semver::Version>,
    /// Other plugins this plugin depends on.
    #[serde(default)]
    pub dependencies: Vec<PluginDependency>,
    /// Capability IDs this plugin advertises.
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// Host platforms this plugin supports (e.g., `["windows", "linux"]`).
    #[serde(default)]
    pub platforms: Vec<String>,
}

/// Plugin type classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginType {
    /// Execution backend (Docker, QEMU, VirtualBox, etc.)
    Runtime,
    /// Device passthrough plugin
    Device,
    /// Monitoring / metrics plugin
    Monitor,
    /// AI provider plugin
    Ai,
    /// Storage backend plugin
    Storage,
    /// Generic extension
    Extension,
}

/// A dependency on another plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    /// Plugin ID of the dependency.
    pub id: String,
    /// Required version range (semver).
    pub version: Option<String>,
}

