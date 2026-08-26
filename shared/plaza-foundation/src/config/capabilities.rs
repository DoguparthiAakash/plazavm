//! Capability grants for `plaza.yaml`.
//!
//! **Default-deny security model**: if a capability is not explicitly declared
//! in the `capabilities:` section of `plaza.yaml`, it is **DENIED**.
//!
//! Virtual hardware (CPU, RAM, disk, display, keyboard, mouse) is always
//! available as part of the emulated machine. These are NOT host capability
//! grants.
//!
//! Host capabilities (filesystem, network, clipboard, devices, environment,
//! process execution) must be explicitly granted.

use serde::{Deserialize, Serialize};

/// The `capabilities:` section of `plaza.yaml`.
///
/// Every field is `Option` — absence means **DENIED**.
///
/// # Example
///
/// ```yaml
/// capabilities:
///   filesystem:
///     - path: "./project"
///       mode: read-write
///   network:
///     enabled: true
///     mode: nat
///   clipboard:
///     read: true
///     write: true
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CapabilityGrants {
    /// Host filesystem access grants. Each entry is path-scoped.
    #[serde(default)]
    pub filesystem: Option<Vec<FilesystemGrant>>,

    /// Network access grant.
    #[serde(default)]
    pub network: Option<NetworkGrant>,

    /// Clipboard access grant.
    #[serde(default)]
    pub clipboard: Option<ClipboardGrant>,

    /// Host environment variable forwarding.
    #[serde(default)]
    pub environment: Option<EnvironmentGrant>,

    /// Physical device access grants.
    #[serde(default)]
    pub devices: Option<DeviceGrants>,
}

/// A single filesystem access grant. Must be path-scoped.
///
/// Bad:  `filesystem: true`
/// Good: `filesystem: [{ path: "./project", mode: read-write }]`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemGrant {
    /// Relative or absolute path to grant access to.
    pub path: String,

    /// Access mode.
    #[serde(default)]
    pub mode: FilesystemMode,
}

/// Filesystem access mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum FilesystemMode {
    /// Read-only access.
    #[default]
    ReadOnly,
    /// Read and write access.
    ReadWrite,
}

/// Network access grant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkGrant {
    /// Whether network access is enabled.
    #[serde(default)]
    pub enabled: bool,

    /// Network mode.
    #[serde(default)]
    pub mode: NetworkMode,
}

/// Network isolation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum NetworkMode {
    /// No network access.
    #[default]
    None,
    /// NAT — guest can reach the internet but is not directly reachable.
    Nat,
    /// Isolated virtual network between PlazaVM instances.
    Isolated,
    /// Bridged — guest appears on the host network.
    Bridge,
}

/// Clipboard access grant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardGrant {
    /// Whether the guest can read the host clipboard.
    #[serde(default)]
    pub read: bool,

    /// Whether the guest can write to the host clipboard.
    #[serde(default)]
    pub write: bool,
}

/// Host environment variable forwarding grant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentGrant {
    /// Explicit allowlist of environment variable names to forward.
    /// Only these variables are visible to the guest.
    #[serde(default)]
    pub allow: Vec<String>,
}

/// Physical device access grants.
///
/// All default to `false` (DENIED).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceGrants {
    /// GPU passthrough.
    #[serde(default)]
    pub gpu: bool,

    /// Camera access.
    #[serde(default)]
    pub camera: bool,

    /// Microphone access.
    #[serde(default)]
    pub microphone: bool,

    /// Audio output.
    #[serde(default)]
    pub audio: bool,

    /// USB device passthrough.
    #[serde(default)]
    pub usb: bool,
}

impl CapabilityGrants {
    /// Returns `true` if filesystem access is granted to any path.
    pub fn has_filesystem(&self) -> bool {
        self.filesystem
            .as_ref()
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }

    /// Returns `true` if network access is explicitly enabled.
    pub fn has_network(&self) -> bool {
        self.network.as_ref().map(|n| n.enabled).unwrap_or(false)
    }

    /// Returns `true` if any clipboard access is granted.
    pub fn has_clipboard(&self) -> bool {
        self.clipboard
            .as_ref()
            .map(|c| c.read || c.write)
            .unwrap_or(false)
    }

    /// Returns `true` if any device access is granted.
    pub fn has_any_device(&self) -> bool {
        self.devices
            .as_ref()
            .map(|d| d.gpu || d.camera || d.microphone || d.audio || d.usb)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_deny_all() {
        let caps = CapabilityGrants::default();
        assert!(!caps.has_filesystem());
        assert!(!caps.has_network());
        assert!(!caps.has_clipboard());
        assert!(!caps.has_any_device());
    }

    #[test]
    fn missing_capabilities_section_means_denied() {
        let yaml = "";
        let caps: CapabilityGrants = serde_yaml::from_str(yaml).unwrap();
        assert!(!caps.has_filesystem());
        assert!(!caps.has_network());
        assert!(!caps.has_clipboard());
        assert!(!caps.has_any_device());
    }

    #[test]
    fn filesystem_grant_parsed() {
        let yaml = r#"
filesystem:
  - path: "./project"
    mode: read-write
"#;
        let caps: CapabilityGrants = serde_yaml::from_str(yaml).unwrap();
        assert!(caps.has_filesystem());
        let grants = caps.filesystem.unwrap();
        assert_eq!(grants.len(), 1);
        assert_eq!(grants[0].path, "./project");
        assert_eq!(grants[0].mode, FilesystemMode::ReadWrite);
    }

    #[test]
    fn network_grant_parsed() {
        let yaml = r#"
network:
  enabled: true
  mode: nat
"#;
        let caps: CapabilityGrants = serde_yaml::from_str(yaml).unwrap();
        assert!(caps.has_network());
        let net = caps.network.unwrap();
        assert_eq!(net.mode, NetworkMode::Nat);
    }

    #[test]
    fn clipboard_grant_parsed() {
        let yaml = r#"
clipboard:
  read: true
  write: true
"#;
        let caps: CapabilityGrants = serde_yaml::from_str(yaml).unwrap();
        assert!(caps.has_clipboard());
    }

    #[test]
    fn devices_default_all_denied() {
        let yaml = r#"
devices:
  gpu: false
"#;
        let caps: CapabilityGrants = serde_yaml::from_str(yaml).unwrap();
        assert!(!caps.has_any_device());
    }
}
