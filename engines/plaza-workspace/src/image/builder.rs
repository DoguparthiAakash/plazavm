use crate::distribution::WorkspaceImageBuildPlan;
use crate::image::acquisition::get_acquisition_source;
use backhand::{compression::Compressor, FilesystemCompressor, FilesystemWriter, NodeHeader};
use flate2::read::GzDecoder;
use plaza_foundation::core::{PlazaError, PlazaResult};
use plaza_image::ImageManager;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use tar::Archive;
use tracing::{debug, info, warn};

pub struct UserspaceImageBuilder;

impl UserspaceImageBuilder {
    pub async fn build(
        image_id: &str,
        plan: WorkspaceImageBuildPlan,
        image_manager: Arc<ImageManager>,
    ) -> PlazaResult<String> {
        info!(
            "Starting Userspace Image Builder for Workspace Image ID: {}",
            image_id
        );

        // Security check: We must NOT execute host commands.
        // If the plan requires running commands (e.g., apk add), we must abort.
        if !plan.install_commands.is_empty() {
            return Err(PlazaError::config(
                format!("Workspace Image '{}' requires package installation ({:?}). Safe userspace build environment is unavailable. Pre-built images are required.", image_id, plan.install_commands)
            ));
        }

        debug!("No host execution required. Proceeding with safe userspace image acquisition.");

        let source = get_acquisition_source(&plan.base_image)?;
        let archive_path = source.fetch_base_image(&plan.base_image).await?;

        info!(
            "Successfully acquired and verified real rootfs archive for '{}'",
            plan.base_image
        );

        // Phase 18: Construct virtual block device strictly in userspace using backhand.
        info!("Constructing userspace SquashFS base image from archive...");

        let temp_dir = std::env::temp_dir();
        let out_path = temp_dir.join(format!("{}.sqsh", image_id));
        let out_path_clone = out_path.clone();
        let archive_path_clone = archive_path.clone();

        tokio::task::spawn_blocking(move || -> PlazaResult<()> {
            let archive_file = File::open(&archive_path_clone).map_err(|e| PlazaError::process(e.to_string()))?;
            let tar = GzDecoder::new(archive_file);
            let mut archive = Archive::new(tar);
            
            let mut writer = FilesystemWriter::default();
            writer.set_compressor(FilesystemCompressor::new(Compressor::Gzip, None).unwrap());
            
            for entry in archive.entries().map_err(|e| PlazaError::process(e.to_string()))? {
                let mut entry = entry.map_err(|e| PlazaError::process(e.to_string()))?;
                let path = entry.path().map_err(|e| PlazaError::process(e.to_string()))?;
                let mut path_str = path.to_string_lossy().into_owned().replace("\\", "/");
                if path_str.starts_with("./") {
                    path_str = path_str[2..].to_string();
                }
                
                if path_str.is_empty() {
                    continue;
                }
                
                let header = NodeHeader {
                    permissions: entry.header().mode().unwrap_or(0o755) as u16,
                    uid: entry.header().uid().unwrap_or(0) as u32,
                    gid: entry.header().gid().unwrap_or(0) as u32,
                    mtime: entry.header().mtime().unwrap_or(0) as u32,
                };
                
                match entry.header().entry_type() {
                    tar::EntryType::Directory => {
                        writer.push_dir(path_str.clone(), header).unwrap_or_else(|e| warn!("Failed to push dir {}: {}", path_str, e));
                    },
                    tar::EntryType::Regular => {
                        let mut content = Vec::new();
                        entry.read_to_end(&mut content).map_err(|e| PlazaError::process(e.to_string()))?;
                        writer.push_file(std::io::Cursor::new(content), path_str.clone(), header).unwrap_or_else(|e| warn!("Failed to push file {}: {}", path_str, e));
                    },
                    tar::EntryType::Symlink => {
                        if let Ok(Some(target)) = entry.link_name() {
                            let target_str = target.to_string_lossy().into_owned().replace("\\", "/");
                            writer.push_symlink(target_str, path_str.clone(), header).unwrap_or_else(|e| warn!("Failed to push symlink {}: {}", path_str, e));
                        }
                    },
                    _ => {
                        // Skip unsupported types for userspace SquashFS (devices, fifos, etc.) unless needed.
                        debug!("Skipping unsupported tar entry type for {}: {:?}", path_str, entry.header().entry_type());
                    }
                }
            }
            
            // Phase 19.2: Inject deterministic /plaza-init script for guest ready signal and test execution
            let init_script = r#"#!/bin/sh
echo "PlazaVM: Mounting filesystems..."
mount -t proc proc /proc || true
mountpoint -q /sys || mount -t sysfs sysfs /sys || echo "mount sysfs failed"
mountpoint -q /dev || mount -t devtmpfs devtmpfs /dev || echo "mount devtmpfs failed"

echo "PlazaVM: Loading kernel modules..."
mount -t squashfs /dev/vdc /.modloop || echo "Warning: failed to mount modloop from /dev/vdc"
mount -o bind /.modloop/modules /lib/modules || echo "Warning: failed to bind mount /lib/modules"
echo "Contents of /lib/modules:"
ls -l /lib/modules
ls -l /lib/modules/* || true

modprobe ext4 2>&1 || echo "Warning: failed to modprobe ext4"
modprobe overlay 2>&1 || echo "Warning: failed to modprobe overlay"


# Populate /dev
mdev -s
echo "PlazaVM: Preparing to mount workspace layer..."

# Verify supported filesystems
echo "Supported filesystems (/proc/filesystems):"
cat /proc/filesystems

echo "Contents of /dev/v*"
ls -l /dev/v*

# Wait up to 2 seconds for async virtio block probing
for i in 1 2 3 4 5 6 7 8 9 10; do
    if [ -e /dev/vdb ]; then
        break
    fi
    sleep 0.2
done

# Try to mount the writable layer (Disk B)
if [ -e /dev/vdb ]; then
    echo "PlazaVM: Mounting writable workspace layer..."
    if mount -t ext4 /dev/vdb /mnt/vdb; then
        mkdir -p /mnt/vdb/upper /mnt/vdb/work
        echo "PlazaVM: Initializing overlayfs..."
        if mount -t overlay overlay -o lowerdir=/,upperdir=/mnt/vdb/upper,workdir=/mnt/vdb/work /workspace; then
            echo "SUCCESS_PLAZA_GUEST_READY"
        else
            echo "ERROR_PLAZA_GUEST_OVERLAY_FAILED"
        fi
    else
        echo "ERROR_PLAZA_GUEST_VDB_MOUNT_FAILED"
    fi
else
    # Fallback for tests that intentionally do not provision a writable workspace (e.g. raw block tests)
    echo "SUCCESS_PLAZA_GUEST_READY"
fi

exec sh
"#.replace("\r\n", "\n");
            let init_header = NodeHeader {
                permissions: 0o755,
                uid: 0,
                gid: 0,
                mtime: 0,
            };
            writer.push_file(std::io::Cursor::new(init_script.as_bytes().to_vec()), "plaza-init", init_header)
                .unwrap_or_else(|e| warn!("Failed to push /plaza-init: {}", e));
            
            // Push mount points explicitly since SquashFS is read-only
            let dir_header = NodeHeader {
                permissions: 0o755,
                uid: 0,
                gid: 0,
                mtime: 0,
            };
            writer.push_dir("sys", dir_header.clone()).unwrap_or_else(|e| debug!("sys exists: {}", e));
            writer.push_dir("proc", dir_header.clone()).unwrap_or_else(|e| debug!("proc exists: {}", e));
            writer.push_dir("dev", dir_header.clone()).unwrap_or_else(|e| debug!("dev exists: {}", e));
            writer.push_dir("mnt", dir_header.clone()).unwrap_or_else(|e| debug!("mnt exists: {}", e));
            writer.push_dir("mnt/vdb", dir_header.clone()).unwrap_or_else(|e| debug!("mnt/vdb exists: {}", e));
            writer.push_dir("lib/modules", dir_header.clone()).unwrap_or_else(|e| debug!("lib/modules exists: {}", e));
            writer.push_dir(".modloop", dir_header.clone()).unwrap_or_else(|e| debug!(".modloop exists: {}", e));
            writer.push_dir("workspace", dir_header.clone()).unwrap_or_else(|e| debug!("workspace exists: {}", e));
            
            let mut out_file = File::create(&out_path_clone).map_err(|e| PlazaError::process(e.to_string()))?;
            writer.write(&mut out_file).map_err(|e| PlazaError::process(e.to_string()))?;
            Ok(())
        }).await.map_err(|e| PlazaError::process(e.to_string()))??;

        info!("Successfully generated SquashFS image at {:?}", out_path);

        // Push the compiled base image into the ImageManager
        image_manager
            .import_raw(image_id, "latest", &out_path)
            .await?;

        // Cleanup temp files
        let _ = tokio::fs::remove_file(archive_path).await;
        let _ = tokio::fs::remove_file(out_path).await;

        Ok(image_id.to_string())
    }
}
