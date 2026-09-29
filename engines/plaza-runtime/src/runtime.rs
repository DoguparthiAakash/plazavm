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
///
/// ## Open-source runtimes (source vendored in project)
/// These are compiled from source inside an Inferno build workspace.
/// Source paths are resolved from the `inferno-os/` directory or user-overridden
/// via `plaza.yaml` (`env.source_path`).
///
/// ## Reference platforms (manual / user-built)
/// Windows-compatible and macOS-compatible environments are NOT shipped with
/// PlazaVM source. See `docs/manual/platforms/windows.md` (ReactOS) and
/// `docs/manual/platforms/macos.md` (Darling) for build instructions.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GuestRuntimeKind {
    /// Standard Linux runtime — LTS kernel + musl libc + BusyBox.
    /// Source: `vendors/linux/` (configurable via plaza.yaml `env.source_path`)
    Linux,
    /// Inferno OS runtime (default).
    /// Source: `inferno-os/`
    Inferno,
    /// FreeBSD runtime.
    /// Source: `inferno-os/FreeBSD/` (configurable via plaza.yaml `env.source_path`)
    #[serde(rename = "freebsd")]
    FreeBsd,
    /// OpenBSD runtime.
    /// Source: `inferno-os/OpenBSD/` (configurable via plaza.yaml `env.source_path`)
    #[serde(rename = "openbsd")]
    OpenBsd,
    /// NetBSD runtime.
    /// Source: `inferno-os/NetBSD/` (configurable via plaza.yaml `env.source_path`)
    #[serde(rename = "netbsd")]
    NetBsd,
    /// DragonFlyBSD runtime.
    /// Source: `inferno-os/DragonFly/` (configurable via plaza.yaml `env.source_path`)
    #[serde(rename = "dragonfly")]
    DragonFly,
}

impl Default for GuestRuntimeKind {
    fn default() -> Self {
        Self::Inferno
    }
}

impl std::fmt::Display for GuestRuntimeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Linux     => write!(f, "linux"),
            Self::Inferno   => write!(f, "inferno"),
            Self::FreeBsd   => write!(f, "freebsd"),
            Self::OpenBsd   => write!(f, "openbsd"),
            Self::NetBsd    => write!(f, "netbsd"),
            Self::DragonFly => write!(f, "dragonfly"),
        }
    }
}

impl std::str::FromStr for GuestRuntimeKind {
    type Err = PlazaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "linux"                     => Ok(Self::Linux),
            "inferno"                   => Ok(Self::Inferno),
            "freebsd" | "free-bsd"      => Ok(Self::FreeBsd),
            "openbsd" | "open-bsd"      => Ok(Self::OpenBsd),
            "netbsd"  | "net-bsd"       => Ok(Self::NetBsd),
            "dragonfly" | "dragonflybsd" | "dragonfly-bsd" => Ok(Self::DragonFly),
            other => Err(PlazaError::config(format!(
                "Unknown guest runtime kind: '{}'. Supported: linux, inferno, freebsd, openbsd, netbsd, dragonfly",
                other
            ))),
        }
    }
}

impl GuestRuntimeKind {
    /// Returns all supported runtime kinds in display order.
    pub fn all() -> &'static [GuestRuntimeKind] {
        &[
            GuestRuntimeKind::Inferno,
            GuestRuntimeKind::Linux,
            GuestRuntimeKind::FreeBsd,
            GuestRuntimeKind::OpenBsd,
            GuestRuntimeKind::NetBsd,
            GuestRuntimeKind::DragonFly,
        ]
    }

    /// Returns true if this runtime is built from open-source source trees
    /// shipped within the PlazaVM project.
    pub fn is_open_source_vendored(&self) -> bool {
        matches!(self,
            Self::Linux | Self::FreeBsd | Self::OpenBsd | Self::NetBsd | Self::DragonFly
        )
    }

    /// Returns the default vendored source directory for this runtime,
    /// relative to the workspace root.
    ///
    /// Returns `None` for `Inferno` (uses `inferno-os/` natively, no build step)
    /// and for any future reference-only platform.
    pub fn default_source_dir(&self) -> Option<std::path::PathBuf> {
        match self {
            Self::Linux     => Some(std::path::PathBuf::from("vendors/linux")),
            Self::Inferno   => None,
            Self::FreeBsd   => Some(std::path::PathBuf::from("inferno-os/FreeBSD")),
            Self::OpenBsd   => Some(std::path::PathBuf::from("inferno-os/OpenBSD")),
            Self::NetBsd    => Some(std::path::PathBuf::from("inferno-os/NetBSD")),
            Self::DragonFly => Some(std::path::PathBuf::from("inferno-os/DragonFly")),
        }
    }

    /// Human-readable display name for UIs and logs.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Linux     => "Linux (LTS kernel + musl + BusyBox)",
            Self::Inferno   => "Inferno OS",
            Self::FreeBsd   => "FreeBSD",
            Self::OpenBsd   => "OpenBSD",
            Self::NetBsd    => "NetBSD",
            Self::DragonFly => "DragonFlyBSD",
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
    /// The project source directory on the host.
    pub project_path: Option<PathBuf>,
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
    fn guest_runtime_kind_from_str_bsd_variants() {
        assert_eq!(GuestRuntimeKind::from_str("freebsd").unwrap(), GuestRuntimeKind::FreeBsd);
        assert_eq!(GuestRuntimeKind::from_str("FreeBSD").unwrap(), GuestRuntimeKind::FreeBsd);
        assert_eq!(GuestRuntimeKind::from_str("free-bsd").unwrap(), GuestRuntimeKind::FreeBsd);

        assert_eq!(GuestRuntimeKind::from_str("openbsd").unwrap(), GuestRuntimeKind::OpenBsd);
        assert_eq!(GuestRuntimeKind::from_str("OpenBSD").unwrap(), GuestRuntimeKind::OpenBsd);
        assert_eq!(GuestRuntimeKind::from_str("open-bsd").unwrap(), GuestRuntimeKind::OpenBsd);

        assert_eq!(GuestRuntimeKind::from_str("netbsd").unwrap(), GuestRuntimeKind::NetBsd);
        assert_eq!(GuestRuntimeKind::from_str("NetBSD").unwrap(), GuestRuntimeKind::NetBsd);
        assert_eq!(GuestRuntimeKind::from_str("net-bsd").unwrap(), GuestRuntimeKind::NetBsd);

        assert_eq!(GuestRuntimeKind::from_str("dragonfly").unwrap(), GuestRuntimeKind::DragonFly);
        assert_eq!(GuestRuntimeKind::from_str("DragonFly").unwrap(), GuestRuntimeKind::DragonFly);
        assert_eq!(GuestRuntimeKind::from_str("dragonflybsd").unwrap(), GuestRuntimeKind::DragonFly);
        assert_eq!(GuestRuntimeKind::from_str("dragonfly-bsd").unwrap(), GuestRuntimeKind::DragonFly);
    }

    #[test]
    fn guest_runtime_kind_from_str_invalid() {
        assert!(GuestRuntimeKind::from_str("docker").is_err());
        assert!(GuestRuntimeKind::from_str("windows").is_err());
        assert!(GuestRuntimeKind::from_str("macos").is_err());
        assert!(GuestRuntimeKind::from_str("").is_err());
    }

    #[test]
    fn guest_runtime_kind_display() {
        assert_eq!(GuestRuntimeKind::Linux.to_string(),     "linux");
        assert_eq!(GuestRuntimeKind::Inferno.to_string(),   "inferno");
        assert_eq!(GuestRuntimeKind::FreeBsd.to_string(),   "freebsd");
        assert_eq!(GuestRuntimeKind::OpenBsd.to_string(),   "openbsd");
        assert_eq!(GuestRuntimeKind::NetBsd.to_string(),    "netbsd");
        assert_eq!(GuestRuntimeKind::DragonFly.to_string(), "dragonfly");
    }

    #[test]
    fn guest_runtime_kind_serde_roundtrip() {
        let cases = [
            (GuestRuntimeKind::Linux,     "\"linux\""),
            (GuestRuntimeKind::Inferno,   "\"inferno\""),
            (GuestRuntimeKind::FreeBsd,   "\"freebsd\""),
            (GuestRuntimeKind::OpenBsd,   "\"openbsd\""),
            (GuestRuntimeKind::NetBsd,    "\"netbsd\""),
            (GuestRuntimeKind::DragonFly, "\"dragonfly\""),
        ];
        for (kind, expected_json) in &cases {
            let json = serde_json::to_string(kind).unwrap();
            assert_eq!(json, *expected_json, "serialize {:?}", kind);
            let deserialized: GuestRuntimeKind = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, *kind, "deserialize {:?}", kind);
        }
    }

    #[test]
    fn guest_runtime_kind_serde_default_for_missing() {
        #[derive(serde::Deserialize)]
        struct TestSpec {
            #[serde(default)]
            guest_runtime: GuestRuntimeKind,
        }
        let spec: TestSpec = serde_json::from_str("{}").unwrap();
        assert_eq!(spec.guest_runtime, GuestRuntimeKind::Inferno);
    }

    #[test]
    fn guest_runtime_kind_all_covers_every_variant() {
        let all = GuestRuntimeKind::all();
        assert_eq!(all.len(), 6);
        assert!(all.contains(&GuestRuntimeKind::Inferno));
        assert!(all.contains(&GuestRuntimeKind::Linux));
        assert!(all.contains(&GuestRuntimeKind::FreeBsd));
        assert!(all.contains(&GuestRuntimeKind::OpenBsd));
        assert!(all.contains(&GuestRuntimeKind::NetBsd));
        assert!(all.contains(&GuestRuntimeKind::DragonFly));
    }

    #[test]
    fn guest_runtime_kind_is_open_source_vendored() {
        assert!(!GuestRuntimeKind::Inferno.is_open_source_vendored());
        assert!(GuestRuntimeKind::Linux.is_open_source_vendored());
        assert!(GuestRuntimeKind::FreeBsd.is_open_source_vendored());
        assert!(GuestRuntimeKind::OpenBsd.is_open_source_vendored());
        assert!(GuestRuntimeKind::NetBsd.is_open_source_vendored());
        assert!(GuestRuntimeKind::DragonFly.is_open_source_vendored());
    }

    #[test]
    fn guest_runtime_kind_default_source_dirs() {
        assert_eq!(
            GuestRuntimeKind::Linux.default_source_dir(),
            Some(std::path::PathBuf::from("vendors/linux"))
        );
        assert_eq!(GuestRuntimeKind::Inferno.default_source_dir(), None);
        assert_eq!(
            GuestRuntimeKind::FreeBsd.default_source_dir(),
            Some(std::path::PathBuf::from("inferno-os/FreeBSD"))
        );
        assert_eq!(
            GuestRuntimeKind::OpenBsd.default_source_dir(),
            Some(std::path::PathBuf::from("inferno-os/OpenBSD"))
        );
        assert_eq!(
            GuestRuntimeKind::NetBsd.default_source_dir(),
            Some(std::path::PathBuf::from("inferno-os/NetBSD"))
        );
        assert_eq!(
            GuestRuntimeKind::DragonFly.default_source_dir(),
            Some(std::path::PathBuf::from("inferno-os/DragonFly"))
        );
    }
}

