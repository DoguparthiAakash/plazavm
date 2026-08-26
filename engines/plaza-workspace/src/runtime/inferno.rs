//! Inferno guest runtime implementation.
//!
//! Provides workspace provisioning for the Inferno OS runtime.
//! Inferno is the PRIMARY and DEFAULT runtime for PlazaVM.
//!
//! # Architecture
//!
//! Inferno OS provides:
//! - Compact runtime (~1-5 MB target for minimal guest)
//! - Dis bytecode VM
//! - Limbo programming environment
//! - Namespace-oriented resource model
//! - 9P/Styx resource access
//! - Built-in TCP/IP networking
//!
//! # Boot Mechanism
//!
//! Inferno boots from a native kernel binary via QEMU's `-kernel` flag.
//! Unlike Linux, Inferno does not use an initrd - the kernel contains
//! its own init system and namespace setup.
//!
//! # Limitations
//!
//! Inferno is NOT Linux. It does not provide:
//! - Linux syscalls
//! - glibc
//! - /proc, /sys (as Linux understands them)
//! - Linux process semantics
//! - Linux networking stack
//!
//! Projects requiring Linux compatibility should use LinuxGuestRuntime.

use async_trait::async_trait;
use plaza_foundation::config::machine_section;
use plaza_foundation::core::{PlazaError, PlazaResult};
use plaza_image::block::{
    Ext4WritableFilesystemProvider, VirtualBlockDevice, WritableFilesystemProvider,
};
use plaza_runtime::runtime::{
    GuestRuntime, GuestRuntimeKind, GuestRuntimeParams, ResolvedArtifacts,
};
use plaza_runtime::{MachineConfig, OperatingSystemTarget, RuntimeStorage};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

use crate::image::inferno_acquisition::InfernoAcquisitionSource;
use crate::image::inferno_builder::InfernoImageBuilder;

/// Inferno OS guest runtime.
///
/// This is the primary runtime for PlazaVM. Inferno provides a minimal,
/// capability-oriented guest environment that avoids shipping a full
/// Linux distribution.
///
/// Status: EXPERIMENTAL — real Inferno boot is not yet verified.
/// The provisioning pipeline is implemented but requires a real Inferno
/// kernel to complete the boot process.
pub struct InfernoGuestRuntime;

impl InfernoGuestRuntime {
    pub fn new() -> Self {
        Self
    }
}

impl Default for InfernoGuestRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GuestRuntime for InfernoGuestRuntime {
    fn kind(&self) -> GuestRuntimeKind {
        GuestRuntimeKind::Inferno
    }

    fn display_name(&self) -> &str {
        "Inferno OS (Primary)"
    }

    async fn provision_image(
        &self,
        params: &GuestRuntimeParams,
        image_manager: Arc<plaza_image::ImageManager>,
    ) -> PlazaResult<String> {
        info!(
            "InfernoGuestRuntime: Provisioning image for workspace '{}'",
            params.workspace_name
        );

        // For Inferno, we need to:
        // 1. Acquire the REAL Inferno kernel (not a placeholder)
        // 2. Build a minimal root filesystem image
        // 3. Import it into the ImageManager

        let acq = InfernoAcquisitionSource::new()?;
        let kernel_path = acq.fetch_kernel().await?;

        // Verify this is a real kernel, not a placeholder
        if kernel_path.to_string_lossy().contains("placeholder") {
            return Err(PlazaError::InfernoKernelUnavailable {
                reason: "Only placeholder kernel found. A real Inferno kernel is required.\n\
                         Build with: ./scripts/build-inferno-kernel.sh"
                    .to_string(),
            });
        }

        info!(
            "InfernoGuestRuntime: Resolved kernel at {:?}",
            kernel_path
        );

        // Build the Inferno image
        let image_id = format!("inferno-{}", params.workspace_id);
        InfernoImageBuilder::build(&image_id, kernel_path, image_manager).await?;

        Ok(image_id)
    }

    async fn create_workspace_device(
        &self,
        params: &GuestRuntimeParams,
    ) -> PlazaResult<Box<dyn VirtualBlockDevice>> {
        info!(
            "InfernoGuestRuntime: Creating workspace device for '{}'",
            params.workspace_name
        );

        let storage_dir = params.workspace_dir.join(".plaza").join("storage");
        tokio::fs::create_dir_all(&storage_dir)
            .await
            .map_err(PlazaError::Io)?;

        let cow_path = storage_dir.join(format!("{}.img", params.workspace_id));

        let provider = Ext4WritableFilesystemProvider::new(cow_path, 4096);
        // 1 GB workspace writable layer
        provider.create(1024 * 1024 * 1024).await
    }

    async fn resolve_artifacts(
        &self,
        params: &GuestRuntimeParams,
    ) -> PlazaResult<ResolvedArtifacts> {
        info!(
            "InfernoGuestRuntime: Resolving kernel artifacts for '{}'",
            params.workspace_name
        );

        let acq = InfernoAcquisitionSource::new()?;
        let kernel_path = acq.fetch_kernel().await?;

        // Verify this is a real kernel
        if kernel_path.to_string_lossy().contains("placeholder") {
            return Err(PlazaError::InfernoKernelUnavailable {
                reason: "Only placeholder kernel found. A real Inferno kernel is required."
                    .to_string(),
            });
        }

        // Inferno does not use a separate initrd - the kernel contains
        // its own init system.
        let initrd_path = kernel_path
            .parent()
            .unwrap_or(&PathBuf::from("."))
            .join("inferno-initrd-dummy");

        Ok(ResolvedArtifacts {
            kernel_path,
            initrd_path,
            modloop_path: None,
        })
    }

    async fn build_machine_config(
        &self,
        params: &GuestRuntimeParams,
        _base_image_path: &PathBuf,
        _storage: &RuntimeStorage,
    ) -> PlazaResult<MachineConfig> {
        let artifacts = self.resolve_artifacts(params).await?;

        let machine_section = machine_section::MachineSection::default();

        // Inferno uses Multiboot format and reads configuration from CONFADDR
        // The kernel is loaded directly via QEMU's -kernel flag
        // No traditional Linux kernel args are needed - Inferno uses Plan 9 ini format
        // stored in memory at CONFADDR
        //
        // For QEMU, we pass configuration via multiboot command line:
        // - bootdisk: which device to boot from
        // - noboot: skip boot menu
        // - console settings
        let kernel_args = Some(
            "bootdisk=sdC0 \
             noboot=true \
             nobreak=true"
                .to_string(),
        );

        info!(
            "InfernoGuestRuntime: Building MachineConfig with kernel={:?}",
            artifacts.kernel_path
        );

        Ok(MachineConfig {
            workspace_id: params.workspace_id.clone(),
            instance_id: params.workspace_id.clone(),
            machine: machine_section,
            capabilities: plaza_foundation::core::CapabilityPolicy::default(),
            os_target: OperatingSystemTarget::Inferno,
            boot_device: PathBuf::from("inferno-rootfs"),
            kernel_path: Some(artifacts.kernel_path),
            initrd_path: Some(artifacts.initrd_path),
            kernel_args,
            modloop_path: None,
            volume_mounts: std::collections::HashMap::new(),
            port_forwards: std::collections::HashMap::new(),
            env_vars: std::collections::HashMap::new(),
        })
    }

    fn readiness_marker(&self) -> &str {
        "SUCCESS_INFERNO_GUEST_READY"
    }

    fn default_backend_id(&self) -> &str {
        "qemu"
    }

    async fn check_availability(&self) -> PlazaResult<bool> {
        // Check if real Inferno artifacts are available
        let acq = InfernoAcquisitionSource::new()?;
        match acq.fetch_kernel().await {
            Ok(kernel_path) => {
                // Verify it's not a placeholder
                if kernel_path.to_string_lossy().contains("placeholder") {
                    info!("InfernoGuestRuntime: Only placeholder kernel available");
                    return Ok(false);
                }
                info!("InfernoGuestRuntime: Real kernel available at {:?}", kernel_path);
                Ok(true)
            }
            Err(e) => {
                info!(
                    "InfernoGuestRuntime: Kernel not available: {}",
                    e
                );
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inferno_runtime_kind() {
        let runtime = InfernoGuestRuntime::new();
        assert_eq!(runtime.kind(), GuestRuntimeKind::Inferno);
    }

    #[test]
    fn inferno_display_name() {
        let runtime = InfernoGuestRuntime::new();
        assert_eq!(runtime.display_name(), "Inferno OS (Primary)");
    }

    #[test]
    fn inferno_readiness_marker() {
        let runtime = InfernoGuestRuntime::new();
        assert_eq!(
            runtime.readiness_marker(),
            "SUCCESS_INFERNO_GUEST_READY"
        );
    }

    #[test]
    fn inferno_default_backend() {
        let runtime = InfernoGuestRuntime::new();
        assert_eq!(runtime.default_backend_id(), "qemu");
    }

    #[tokio::test]
    async fn inferno_check_availability() {
        let runtime = InfernoGuestRuntime::new();
        // Should return Ok(false) since no real kernel is available in test env
        let result = runtime.check_availability().await;
        assert!(result.is_ok());
        // In test environment, this will be false (no real kernel)
        // This is expected behavior - no placeholder fallback
    }
}
