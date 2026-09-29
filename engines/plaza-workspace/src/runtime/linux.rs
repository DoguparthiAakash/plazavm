//! Linux guest runtime implementation.
//!
//! Boots an Alpine Linux kernel via QEMU with minimal footprint.
//! Uses `rdinit=/bin/sh` and triggers readiness via serial echo.

use async_trait::async_trait;
use plaza_foundation::config::machine_section;
use plaza_foundation::core::{PlazaError, PlazaResult};
use plaza_image::block::{Ext4WritableFilesystemProvider, VirtualBlockDevice, WritableFilesystemProvider};
use plaza_runtime::runtime::{
    GuestRuntime, GuestRuntimeKind, GuestRuntimeParams, ResolvedArtifacts,
};
use plaza_runtime::{MachineConfig, OperatingSystemTarget, RuntimeStorage};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

/// Marker printed by the guest to signal it is ready for commands.
const OS_READY_MARKER: &str = "SUCCESS_PLAZA_GUEST_READY";

/// Linux guest runtime.
pub struct LinuxGuestRuntime;

impl LinuxGuestRuntime {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LinuxGuestRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GuestRuntime for LinuxGuestRuntime {
    fn kind(&self) -> GuestRuntimeKind {
        GuestRuntimeKind::Linux
    }

    fn display_name(&self) -> &str {
        "Linux (Alpine)"
    }

    async fn provision_image(
        &self,
        params: &GuestRuntimeParams,
        _image_manager: Arc<plaza_image::ImageManager>,
    ) -> PlazaResult<String> {
        info!(
            "LinuxGuestRuntime: Provisioning Alpine image for workspace '{}'",
            params.workspace_name
        );
        let image_id = format!("linux-{}", params.workspace_id);
        Ok(image_id)
    }

    async fn create_workspace_device(
        &self,
        params: &GuestRuntimeParams,
    ) -> PlazaResult<Box<dyn VirtualBlockDevice>> {
        info!(
            "LinuxGuestRuntime: Creating workspace device for '{}'",
            params.workspace_name
        );

        let storage_dir = params.workspace_dir.join(".plaza").join("storage");
        tokio::fs::create_dir_all(&storage_dir)
            .await
            .map_err(PlazaError::Io)?;

        let cow_path = storage_dir.join(format!("{}.img", params.workspace_id));

        // Create a 256MB ext4 writable workspace layer (reduced for low resource usage)
        let provider = Ext4WritableFilesystemProvider::new(cow_path, 4096);
        provider.create(256 * 1024 * 1024).await
    }

    async fn resolve_artifacts(
        &self,
        params: &GuestRuntimeParams,
    ) -> PlazaResult<ResolvedArtifacts> {
        use crate::image::acquisition::AlpineAcquisitionSource;

        info!(
            "LinuxGuestRuntime: Resolving Alpine artifacts for '{}'",
            params.workspace_name
        );

        let source = AlpineAcquisitionSource::new()?;
        let (kernel_path, initrd_path, modloop_path) =
            source.fetch_kernel_and_initrd("alpine:3.19.1").await?;

        Ok(ResolvedArtifacts {
            kernel_path,
            initrd_path,
            modloop_path,
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

        // Boot via initramfs (rdinit=/bin/sh)
        // No root= needed since initramfs provides the root filesystem.
        // The readiness trigger in QemuProcess will periodically send
        // `echo 'SUCCESS_PLAZA_GUEST_READY'` until the marker appears.
        let kernel_args = "console=ttyS0 rdinit=/bin/sh".to_string();

        info!(
            "LinuxGuestRuntime: Building MachineConfig with kernel={:?}, initrd={:?}",
            artifacts.kernel_path, artifacts.initrd_path
        );

        Ok(MachineConfig {
            workspace_id: params.workspace_id.clone(),
            instance_id: params.workspace_id.clone(),
            machine: machine_section,
            capabilities: plaza_foundation::core::CapabilityPolicy::default(),
            os_target: OperatingSystemTarget::Linux,
            boot_device: PathBuf::from("dummy"),
            kernel_path: Some(artifacts.kernel_path),
            initrd_path: Some(artifacts.initrd_path),
            kernel_args: Some(kernel_args),
            modloop_path: artifacts.modloop_path,
            workspace_sqfs_path: None,
            volume_mounts: std::collections::HashMap::new(),
            port_forwards: std::collections::HashMap::new(),
            env_vars: std::collections::HashMap::new(),
        })
    }

    fn readiness_marker(&self) -> &str {
        OS_READY_MARKER
    }

    fn default_backend_id(&self) -> &str {
        "qemu"
    }

    async fn check_availability(&self) -> PlazaResult<bool> {
        Ok(true)
    }
}
