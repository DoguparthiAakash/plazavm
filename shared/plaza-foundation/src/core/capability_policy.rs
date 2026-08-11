//! Validated capability policy.
//!
//! While `CapabilityGrants` represents the raw parsed YAML,
//! `CapabilityPolicy` represents the semantically validated,
//! path-canonicalized policy that is attached to a `MachineConfig`
//! and enforced by the engine.

use crate::config::capabilities::{CapabilityGrants, FilesystemMode, NetworkMode};
use crate::core::PlazaResult;
use std::path::{Path, PathBuf};

/// The finalized capability policy attached to a machine.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CapabilityPolicy {
    pub filesystem: Vec<ResolvedFilesystemGrant>,
    pub network: ResolvedNetworkPolicy,
    pub clipboard: ResolvedClipboardPolicy,
    pub environment: ResolvedEnvironmentPolicy,
    pub devices: ResolvedDevicePolicy,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResolvedFilesystemGrant {
    /// The canonical, absolute path on the host.
    pub host_path: PathBuf,
    pub mode: FilesystemMode,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ResolvedNetworkPolicy {
    pub enabled: bool,
    pub mode: NetworkMode,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ResolvedClipboardPolicy {
    pub read: bool,
    pub write: bool,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ResolvedEnvironmentPolicy {
    pub allowed_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ResolvedDevicePolicy {
    pub gpu: bool,
    pub camera: bool,
    pub microphone: bool,
    pub audio: bool,
    pub usb: bool,
}

impl CapabilityPolicy {
    /// Construct a resolved policy from raw YAML grants.
    ///
    /// Requires the workspace's root directory to resolve relative paths
    /// and canonicalize them against the host filesystem.
    pub fn resolve(grants: Option<&CapabilityGrants>, workspace_dir: &Path) -> PlazaResult<Self> {
        let grants = match grants {
            Some(g) => g,
            None => return Ok(CapabilityPolicy::default()), // Default-deny all
        };

        let filesystem = Self::resolve_filesystem(grants, workspace_dir)?;
        
        let network = grants
            .network
            .as_ref()
            .map(|n| ResolvedNetworkPolicy {
                enabled: n.enabled,
                mode: n.mode,
            })
            .unwrap_or_default();

        let clipboard = grants
            .clipboard
            .as_ref()
            .map(|c| ResolvedClipboardPolicy {
                read: c.read,
                write: c.write,
            })
            .unwrap_or_default();

        let environment = grants
            .environment
            .as_ref()
            .map(|e| ResolvedEnvironmentPolicy {
                allowed_keys: e.allow.clone(),
            })
            .unwrap_or_default();

        let devices = grants
            .devices
            .as_ref()
            .map(|d| ResolvedDevicePolicy {
                gpu: d.gpu,
                camera: d.camera,
                microphone: d.microphone,
                audio: d.audio,
                usb: d.usb,
            })
            .unwrap_or_default();

        Ok(Self {
            filesystem,
            network,
            clipboard,
            environment,
            devices,
        })
    }

    fn resolve_filesystem(
        grants: &CapabilityGrants,
        workspace_dir: &Path,
    ) -> PlazaResult<Vec<ResolvedFilesystemGrant>> {
        let mut resolved = Vec::new();

        if let Some(fs_grants) = &grants.filesystem {
            for grant in fs_grants {
                let joined_path = workspace_dir.join(&grant.path);
                
                // std::fs::canonicalize requires the path to exist on disk.
                // For DP1, we assume the paths must exist if they are granted.
                let canonical_path = std::fs::canonicalize(&joined_path)
                    .map_err(|e| crate::core::PlazaError::config(format!(
                        "failed to resolve path '{}': {}", grant.path, e
                    )))?;

                resolved.push(ResolvedFilesystemGrant {
                    host_path: canonical_path,
                    mode: grant.mode,
                });
            }
        }

        Ok(resolved)
    }
}
