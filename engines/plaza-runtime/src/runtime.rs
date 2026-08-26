//! Guest runtime abstraction layer.
//!
//! Defines the trait that each guest runtime (Linux, Inferno, future runtimes)
//! must implement to provide workspace-specific provisioning, boot configuration,
//! and readiness detection.
//!
//! This is the primary abstraction point that decouples the WorkspaceEngine
//! from any specific guest operating system.

use async_trait::async_trait;
use plaza_foundation::core::{PlazaError, PlazaResult};
use std::path::PathBuf;

use crate::MachineConfig;
use crate::RuntimeStorage;
use std::sync::Arc;

/// The type of guest runtime to use inside the workspace VM.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GuestRuntimeKind {
    /// Standard Linux runtime (Alpine, etc.)
    Linux,
    /// Experimental Inferno OS runtime
    Inferno,
}

impl Default for GuestRuntimeKind {
    fn default() -> Self {
        Self::Inferno
    }
}

impl std::fmt::Display for GuestRuntimeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Linux => write!(f, "linux"),
            Self::Inferno => write!(f, "inferno"),
        }
    }
}

impl std::str::FromStr for GuestRuntimeKind {
    type Err = PlazaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "linux" => Ok(Self::Linux),
            "inferno" => Ok(Self::Inferno),
            other => Err(PlazaError::config(format!(
                "Unknown guest runtime kind: '{}'. Supported: linux, inferno",
                other
            ))),
        }
    }
}

/// Parameters passed to a GuestRuntime for provisioning.
///
/// This struct captures the workspace-level configuration needed by the
/// guest runtime without depending on plaza-workspace types directly.
#[derive(Debug, Clone)]
pub struct GuestRuntimeParams {
    /// The unique workspace identifier.
    pub workspace_id: String,
    /// The human-readable workspace name.
    pub workspace_name: String,
    /// The path to the workspace directory on the host.
    pub workspace_dir: PathBuf,
    /// The image reference (e.g. "alpine:3.19.1") or None for runtime default.
    pub image_reference: Option<String>,
}

/// Result of guest runtime artifact resolution.
///
/// Contains the resolved paths to the kernel, initrd, and optional modloop
/// that the runtime needs to boot inside QEMU.
#[derive(Debug, Clone)]
pub struct ResolvedArtifacts {
    /// Path to the kernel binary (e.g. vmlinuz).
    pub kernel_path: PathBuf,
    /// Path to the initial ramdisk.
    pub initrd_path: PathBuf,
    /// Optional path to a modloop or modules image.
    pub modloop_path: Option<PathBuf>,
}

/// The core abstraction for a guest runtime.
///
/// Each guest runtime (Linux, Inferno, etc.) implements this trait to provide
/// the workspace engine with runtime-specific provisioning, configuration,
/// and readiness detection.
///
/// # Design Principles
///
/// - **Minimal interface**: Only methods that vary between runtimes are required.
/// - **No QEMU coupling**: This trait is backend-agnostic. The QEMU plugin
///   consumes `MachineConfig` and `RuntimeStorage`, not this trait.
/// - **Explicit capability errors**: If a runtime cannot satisfy a request,
///   it returns a typed error rather than silently falling back.
#[async_trait]
pub trait GuestRuntime: Send + Sync {
    /// Unique identifier for this runtime kind (e.g. "linux", "inferno").
    fn kind(&self) -> GuestRuntimeKind;

    /// Human-readable display name.
    fn display_name(&self) -> &str;

    /// Resolve the immutable base image for this runtime.
    ///
    /// Uses the ImageManager to provision or locate the base image blob.
    /// Returns the image ID that can be used to look up the manifest.
    async fn provision_image(
        &self,
        params: &GuestRuntimeParams,
        image_manager: Arc<plaza_image::ImageManager>,
    ) -> PlazaResult<String>;

    /// Create the workspace writable storage device.
    ///
    /// Returns a `VirtualBlockDevice` that represents the writable workspace
    /// layer. The caller wraps this into `RuntimeStorage`.
    async fn create_workspace_device(
        &self,
        params: &GuestRuntimeParams,
    ) -> PlazaResult<Box<dyn plaza_image::block::VirtualBlockDevice>>;

    /// Resolve runtime-specific artifacts (kernel, initrd, etc.).
    ///
    /// For Linux, this fetches the Alpine kernel and initrd.
    /// For Inferno, this would resolve the Inferno kernel and boot image.
    async fn resolve_artifacts(
        &self,
        params: &GuestRuntimeParams,
    ) -> PlazaResult<ResolvedArtifacts>;

    /// Build the MachineConfig for this runtime.
    ///
    /// Constructs a `MachineConfig` with the correct OS target, kernel args,
    /// boot parameters, and any runtime-specific configuration.
    async fn build_machine_config(
        &self,
        params: &GuestRuntimeParams,
        base_image_path: &PathBuf,
        storage: &RuntimeStorage,
    ) -> PlazaResult<MachineConfig>;

    /// Get the expected readiness marker string emitted by the guest.
    ///
    /// The QEMU plugin monitors serial output for this string to determine
    /// when the guest has finished initialization.
    fn readiness_marker(&self) -> &str;

    /// Get the default execution backend ID for this runtime.
    ///
    /// For Linux, this is typically "qemu". For Inferno, it could be "qemu"
    /// or a future Inferno-native backend.
    fn default_backend_id(&self) -> &str;

    /// Check whether the required runtime artifacts are available on this host.
    ///
    /// Returns Ok(true) if all required artifacts exist, Ok(false) if missing,
    /// or Err if the check itself failed.
    async fn check_availability(&self) -> PlazaResult<bool>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn guest_runtime_kind_default_is_inferno() {
        assert_eq!(GuestRuntimeKind::default(), GuestRuntimeKind::Inferno);
    }

    #[test]
    fn guest_runtime_kind_from_str_linux() {
        assert_eq!(GuestRuntimeKind::from_str("linux").unwrap(), GuestRuntimeKind::Linux);
        assert_eq!(GuestRuntimeKind::from_str("Linux").unwrap(), GuestRuntimeKind::Linux);
        assert_eq!(GuestRuntimeKind::from_str("LINUX").unwrap(), GuestRuntimeKind::Linux);
    }

    #[test]
    fn guest_runtime_kind_from_str_inferno() {
        assert_eq!(GuestRuntimeKind::from_str("inferno").unwrap(), GuestRuntimeKind::Inferno);
        assert_eq!(GuestRuntimeKind::from_str("Inferno").unwrap(), GuestRuntimeKind::Inferno);
    }

    #[test]
    fn guest_runtime_kind_from_str_invalid() {
        let result = GuestRuntimeKind::from_str("docker");
        assert!(result.is_err());
    }

    #[test]
    fn guest_runtime_kind_display() {
        assert_eq!(GuestRuntimeKind::Linux.to_string(), "linux");
        assert_eq!(GuestRuntimeKind::Inferno.to_string(), "inferno");
    }

    #[test]
    fn guest_runtime_kind_serde_roundtrip() {
        let linux = GuestRuntimeKind::Linux;
        let json = serde_json::to_string(&linux).unwrap();
        assert_eq!(json, "\"linux\"");
        let deserialized: GuestRuntimeKind = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, GuestRuntimeKind::Linux);

        let inferno = GuestRuntimeKind::Inferno;
        let json = serde_json::to_string(&inferno).unwrap();
        assert_eq!(json, "\"inferno\"");
        let deserialized: GuestRuntimeKind = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, GuestRuntimeKind::Inferno);
    }

    #[test]
    fn guest_runtime_kind_serde_default_for_missing() {
        // When deserializing from an empty JSON object, guest_runtime should default to Inferno
        #[derive(serde::Deserialize)]
        struct TestSpec {
            #[serde(default)]
            guest_runtime: GuestRuntimeKind,
        }
        let spec: TestSpec = serde_json::from_str("{}").unwrap();
        assert_eq!(spec.guest_runtime, GuestRuntimeKind::Inferno);
    }
}
