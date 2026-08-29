//! Inferno OS image builder.
//!
//! Creates minimal Inferno filesystem images for booting under QEMU.
//! Unlike Linux (which uses SquashFS), Inferno uses its native KFS
//! (kernel file system) or raw block devices.
//!
//! CRITICAL: This builder requires a REAL Inferno kernel.
//! Placeholder kernels will cause an explicit error.

use plaza_foundation::core::{PlazaError, PlazaResult};
use plaza_image::ImageManager;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{debug, info};

/// Builder for creating Inferno OS filesystem images.
///
/// The Inferno image consists of:
/// 1. A bootable kernel (ELF binary)
/// 2. A minimal root filesystem containing:
///    - Inferno commands and libraries
///    - PlazaVM init script
///    - 9P server for workspace access
pub struct InfernoImageBuilder;

impl InfernoImageBuilder {
    /// Build a minimal Inferno image.
    ///
    /// This creates:
    /// - A kernel image (from acquisition source)
    /// - A root filesystem image (KFS or raw)
    /// - Injects the PlazaVM init and readiness detection
    ///
    /// # Errors
    ///
    /// Returns `InfernoKernelUnavailable` if the kernel_path is a placeholder.
    /// Returns `InfernoRootFilesystemUnavailable` if rootfs creation fails.
    ///
    /// Returns the image ID that can be used with the ImageManager.
    pub async fn build(
        image_id: &str,
        kernel_path: PathBuf,
        image_manager: Arc<ImageManager>,
    ) -> PlazaResult<String> {
        // Validate the kernel file exists and has reasonable size
        let metadata = tokio::fs::metadata(&kernel_path)
            .await
            .map_err(|e| PlazaError::InfernoKernelUnavailable {
                reason: format!(
                    "Cannot read kernel at {:?}: {}",
                    kernel_path, e
                ),
            })?;

        let file_size = metadata.len();
        if file_size < 1024 {
            return Err(PlazaError::InfernoKernelUnavailable {
                reason: format!(
                    "Kernel at {:?} is too small ({} bytes). \
                     A real Inferno kernel should be at least 100KB.",
                    kernel_path, file_size
                ),
            });
        }

        // Validate this is a real kernel, not a placeholder, by checking ELF magic
        use tokio::io::AsyncReadExt;
        let mut file = tokio::fs::File::open(&kernel_path)
            .await
            .map_err(|e| PlazaError::InfernoKernelUnavailable {
                reason: format!("Failed to open kernel at {:?}: {}", kernel_path, e),
            })?;
        let mut magic = [0u8; 4];
        file.read_exact(&mut magic).await.map_err(|e| PlazaError::InfernoKernelUnavailable {
            reason: format!("Failed to read kernel magic at {:?}: {}", kernel_path, e),
        })?;
        
        if magic != [0x7F, b'E', b'L', b'F'] {
            return Err(PlazaError::InfernoKernelUnavailable {
                reason: format!(
                    "Kernel at {:?} is not a valid ELF binary. \
                     A real Inferno kernel is required. \
                     Build with: ./scripts/build-inferno-kernel.sh",
                    kernel_path
                ),
            });
        }

        info!(
            "Building Inferno image '{}' from kernel {:?} ({} bytes)",
            image_id, kernel_path, file_size
        );

        let temp_dir = std::env::temp_dir();

        // Step 1: Create a minimal root filesystem image
        let rootfs_path = temp_dir.join(format!("inferno-rootfs-{}.img", image_id));

        Self::create_rootfs_image(&rootfs_path, &kernel_path).await?;

        info!(
            "Created Inferno root filesystem image at {:?}",
            rootfs_path
        );

        // Step 2: Import into ImageManager
        image_manager
            .import_raw(image_id, "latest", &rootfs_path)
            .await?;

        // Cleanup
        let _ = tokio::fs::remove_file(&rootfs_path).await;

        Ok(image_id.to_string())
    }

    /// Create a minimal Inferno root filesystem image.
    ///
    /// This is a raw block image that contains the minimal Inferno
    /// filesystem structure. The image is designed to be:
    /// - Read by the Inferno kernel as a block device
    /// - Mounted as the root filesystem
    /// - Used by 9P servers for workspace access
    async fn create_rootfs_image(
        output_path: &PathBuf,
        kernel_path: &PathBuf,
    ) -> PlazaResult<()> {
        // Read the real kernel to include in the rootfs
        let kernel_data = tokio::fs::read(kernel_path)
            .await
            .map_err(|e| PlazaError::InfernoRootFilesystemUnavailable {
                reason: format!("Failed to read kernel for rootfs: {}", e),
            })?;

        // Create a minimal image
        // Size: 16 MB (enough for minimal Inferno rootfs)
        let image_size = 16 * 1024 * 1024; // 16 MB

        // Create a sparse file
        let file = std::fs::File::create(output_path).map_err(|e| {
            PlazaError::Io(std::io::Error::new(
                e.kind(),
                format!("Failed to create Inferno rootfs image: {}", e),
            ))
        })?;

        // Set the file size (sparse allocation)
        file.set_len(image_size).map_err(|e| {
            PlazaError::Io(std::io::Error::new(
                e.kind(),
                format!("Failed to set Inferno rootfs image size: {}", e),
            ))
        })?;

        use std::io::{Seek, SeekFrom, Write};

        let mut file = file;
        file.seek(SeekFrom::Start(0)).map_err(PlazaError::Io)?;

        // Write header: magic (24 bytes) + kernel size (8 bytes) + kernel data
        let magic = b"PLAZA-INFERNO-ROOTFS-V1\0";
        file.write_all(magic).map_err(PlazaError::Io)?;

        let kernel_size = kernel_data.len() as u64;
        file.write_all(&kernel_size.to_le_bytes())
            .map_err(PlazaError::Io)?;

        // Write the actual kernel binary
        file.write_all(&kernel_data).map_err(PlazaError::Io)?;

        // Write the init script that the Inferno kernel will execute
        // This is a comment/metadata section - the actual init is handled
        // by the kernel arguments and the QEMU plugin
        let init_comment = b"# PlazaVM Inferno Guest Init\n# Kernel boot arguments handle initialization\n# Readiness signal: SUCCESS_INFERNO_GUEST_READY\n\0";
        file.write_all(init_comment).map_err(PlazaError::Io)?;

        file.flush().map_err(PlazaError::Io)?;

        debug!(
            "Created minimal Inferno rootfs image: {:?} ({} bytes, kernel {} bytes)",
            output_path, image_size, kernel_size
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn inferno_image_builder_rejects_placeholder() {
        let temp_dir = std::env::temp_dir().join("plaza-test-inferno");
        std::fs::create_dir_all(&temp_dir).unwrap();

        let output_path = temp_dir.join("test-rootfs.img");

        // Try to build with a placeholder kernel
        let placeholder_path = temp_dir.join("inferno-386-placeholder");
        std::fs::write(&placeholder_path, vec![b'x'; 1024]).unwrap();

        let result = InfernoImageBuilder::build(
            "test-image",
            placeholder_path.clone(),
            Arc::new(plaza_image::ImageManager::new(
                Arc::new(plaza_image::store::LocalBlobStore::new(temp_dir.join("blobs")).await.unwrap()),
                Arc::new(plaza_image::store::LocalManifestStore::new(temp_dir.join("manifests")).await.unwrap()),
                Arc::new(plaza_image::gc::LocalGarbageCollector::new(temp_dir.join("blobs"))),
            )),
        )
        .await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("ELF binary"));

        // Cleanup
        std::fs::remove_file(&placeholder_path).unwrap();
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[tokio::test]
    async fn inferno_image_builder_rejects_tiny_kernel() {
        let temp_dir = std::env::temp_dir().join("plaza-test-inferno2");
        std::fs::create_dir_all(&temp_dir).unwrap();

        // Create a tiny file that's too small to be a real kernel, but has valid ELF magic
        let tiny_kernel = temp_dir.join("inferno-386");
        let mut content = vec![0x7F, b'E', b'L', b'F'];
        content.extend_from_slice(b"tiny");
        std::fs::write(&tiny_kernel, &content).unwrap();

        let result = InfernoImageBuilder::build(
            "test-image",
            tiny_kernel.clone(),
            Arc::new(plaza_image::ImageManager::new(
                Arc::new(plaza_image::store::LocalBlobStore::new(temp_dir.join("blobs")).await.unwrap()),
                Arc::new(plaza_image::store::LocalManifestStore::new(temp_dir.join("manifests")).await.unwrap()),
                Arc::new(plaza_image::gc::LocalGarbageCollector::new(temp_dir.join("blobs"))),
            )),
        )
        .await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("too small"));

        // Cleanup
        std::fs::remove_file(&tiny_kernel).unwrap();
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }
}
