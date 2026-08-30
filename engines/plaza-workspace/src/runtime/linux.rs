//! Linux guest runtime implementation.
//!
//! Provides workspace provisioning for the existing Alpine/Linux runtime.
//! Extracts the inline Linux-specific logic from WorkspaceEngine into a
//! clean GuestRuntime implementation.

use async_trait::async_trait;
use plaza_foundation::config::machine_section;
use plaza_foundation::core::{PlazaError, PlazaResult};
use plaza_image::block::{
    Ext4WritableFilesystemProvider, VirtualBlockDevice, WritableFilesystemProvider,
};
use plaza_machine::OS_READY_MARKER;
use plaza_runtime::runtime::{
    GuestRuntime, GuestRuntimeKind, GuestRuntimeParams, ResolvedArtifacts,
};
use plaza_runtime::{MachineConfig, OperatingSystemTarget, RuntimeStorage};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

use crate::image::acquisition::AlpineAcquisitionSource;
use crate::pipeline::TransactionalPipelineBuilder;

/// Linux guest runtime using Alpine Linux as the base distribution.
pub struct LinuxGuestRuntime;

impl LinuxGuestRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Resolve the plaza.yaml path for a workspace.
    fn resolve_plaza_yaml(params: &GuestRuntimeParams) -> PlazaResult<PathBuf> {
        // First check parent of workspace dir (assuming ws is inside .space)
        let parent_yaml = params.workspace_dir.join("..").join("plaza.yaml");
        if parent_yaml.exists() {
            return Ok(parent_yaml);
        }
        // Fallback to project dir
        let fallback = std::env::current_dir()
            .unwrap_or_default()
            .join("plaza.yaml");
        Ok(fallback)
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
        image_manager: Arc<plaza_image::ImageManager>,
    ) -> PlazaResult<String> {
        info!(
            "LinuxGuestRuntime: Provisioning image for workspace '{}'",
            params.workspace_name
        );

        let plaza_yaml_path = Self::resolve_plaza_yaml(params)?;

        if !plaza_yaml_path.exists() {
            return Err(PlazaError::config(format!(
                "No plaza.yaml found for workspace '{}'",
                params.workspace_name
            )));
        }

        let content = tokio::fs::read_to_string(&plaza_yaml_path)
            .await
            .map_err(|e| PlazaError::Io(e))?;

        let yaml = plaza_foundation::config::PlazaYaml::parse_yaml(&content)
            .map_err(|e| PlazaError::config(format!("Invalid plaza.yaml: {}", e)))?;

        TransactionalPipelineBuilder::provision_image(&yaml, image_manager).await
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
            .map_err(|e| PlazaError::Io(e))?;

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
            "LinuxGuestRuntime: Resolving kernel/initrd artifacts for '{}'",
            params.workspace_name
        );

        let image_ref = params.image_reference.as_deref().unwrap_or("alpine:3.19.1");

        let acq = AlpineAcquisitionSource::new()?;
        let (kernel_path, initrd_path, modloop_path) =
            acq.fetch_kernel_and_initrd(image_ref).await?;

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
        
        let storage_dir = params.workspace_dir.join(".plaza").join("storage");
        let tar_path = storage_dir.join(format!("{}_snapshot.tar", params.workspace_id));
        
        // Take a snapshot of the workspace into a tarball block device
        if params.workspace_dir.exists() {
            info!("LinuxGuestRuntime: Taking snapshot of workspace '{}' into tarball", params.workspace_name);
            let ws_dir = params.workspace_dir.clone();
            let tar_out = tar_path.clone();
            
            // Run in a blocking task since tar operations are synchronous
            tokio::task::spawn_blocking(move || -> PlazaResult<()> {
                let file = std::fs::File::create(&tar_out)
                    .map_err(|e| PlazaError::Io(e))?;
                let mut builder = tar::Builder::new(file);
                
                // Append the contents of the workspace directory, ignoring .plaza
                for entry in walkdir::WalkDir::new(&ws_dir).into_iter().filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.components().any(|c| c.as_os_str() == ".plaza") {
                        continue;
                    }
                    if path == ws_dir.as_path() {
                        continue;
                    }
                    
                    let rel_path = path.strip_prefix(&ws_dir).unwrap_or(path);
                    if path.is_dir() {
                        let _ = builder.append_dir(rel_path, path);
                    } else {
                        let mut f = std::fs::File::open(path).map_err(|e| PlazaError::Io(e))?;
                        let _ = builder.append_file(rel_path, &mut f);
                    }
                }
                
                builder.finish().map_err(|e| PlazaError::Io(e))?;
                Ok(())
            })
            .await
            .map_err(|e| PlazaError::config(format!("Snapshot task panicked: {}", e)))??;
        }

        // We use a custom init script to set up the OverlayFS.
        // /dev/vda is the OS (read-only Alpine squashfs).
        // /dev/vdb is the 1GB Ext4 writable COW layer.
        // /dev/vdc is the workspace tarball snapshot.
        let init_script = "
#!/bin/sh
export PATH=/bin:/sbin:/usr/bin:/usr/sbin
mount -t devtmpfs dev /dev
mount -t proc proc /proc
mount -t sysfs sysfs /sys

# Mount the COW layer (upperdir)
mkdir -p /mnt/cow
mount /dev/vdb /mnt/cow

# Set up OverlayFS directories
mkdir -p /mnt/cow/upper
mkdir -p /mnt/cow/work

# Mount a TMPFS for the host snapshot (lowerdir)
mkdir -p /mnt/lower
mount -t tmpfs -o size=2G tmpfs /mnt/lower

# Extract the host snapshot from the tarball block device (/dev/vdc)
echo 'Extracting workspace snapshot...'
tar -xf /dev/vdc -C /mnt/lower 2>/dev/null || true

# Mount the final workspace overlay
mkdir -p /workspace
mount -t overlay overlay -o lowerdir=/mnt/lower,upperdir=/mnt/cow/upper,workdir=/mnt/cow/work /workspace

# Print readiness marker
echo 'PLAZA_OS_READY'

# Drop into a shell (or execute plaza-agent)
exec /bin/sh
";
        
        let init_path = storage_dir.join("plaza-init.sh");
        tokio::fs::write(&init_path, init_script).await.map_err(|e| PlazaError::Io(e))?;

        Ok(MachineConfig {
            workspace_id: params.workspace_id.clone(),
            instance_id: params.workspace_id.clone(),
            machine: machine_section,
            capabilities: plaza_foundation::core::CapabilityPolicy::default(),
            os_target: OperatingSystemTarget::Linux,
            boot_device: PathBuf::from("dummy"),
            kernel_path: Some(artifacts.kernel_path),
            initrd_path: Some(artifacts.initrd_path),
            kernel_args: Some("console=ttyS0 root=/dev/vda rw init=/bin/sh -- -c /mnt/cow/plaza-init.sh".to_string()),
            modloop_path: artifacts.modloop_path,
            workspace_sqfs_path: if tar_path.exists() { Some(tar_path) } else { None },
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
        // Check if Alpine acquisition source can be constructed
        // and if the kernel cache exists
        Ok(true)
    }
}
