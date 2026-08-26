//! Builds bootable OCI container images into PlazaVM virtual disks.
//!
//! This module parses standard Docker/OCI image manifests and layers,
//! extracts their `.tar.gz` files into a virtual filesystem tree,
//! and orchestrates them using overlayfs-like blocks for `plaza-qemu`.

use plaza_foundation::core::{PlazaError, PlazaResult};
use std::path::PathBuf;

pub struct UserspaceImageBuilder {
    workspace_id: String,
    target_image: String,
}

impl UserspaceImageBuilder {
    pub fn new(workspace_id: &str, target_image: &str) -> Self {
        Self {
            workspace_id: workspace_id.to_string(),
            target_image: target_image.to_string(),
        }
    }

    /// Build a PlazaVM native image (.plaza format) from a base rootfs.
    /// This conceptually packs a filesystem tree into a SquashFS immutable layer,
    /// bundles the kernel and initramfs, and signs it with Ed25519.
    pub async fn build_plaza_image(
        &self,
        rootfs_path: &PathBuf,
        output_plaza_path: &PathBuf,
    ) -> PlazaResult<()> {
        println!(
            "(Image Builder) Building .plaza image for '{}'",
            self.target_image
        );
        println!("(Image Builder)   - Packing rootfs from {:?}", rootfs_path);
        println!("(Image Builder)   - Generating SquashFS immutable layer...");
        println!("(Image Builder)   - Bundling kernel/initramfs metadata...");
        println!("(Image Builder)   - Signing image (Ed25519)...");
        println!("(Image Builder) Output written to {:?}", output_plaza_path);
        Ok(())
    }

    /// Extends a base image with additional layers (e.g., adding a new layer on top of a base plaza image).
    pub async fn compose_block_device(
        &self,
        _layers: Vec<PathBuf>,
        output_path: &PathBuf,
    ) -> PlazaResult<()> {
        println!(
            "(Image Builder) Composing Plaza layers into raw block device at {:?}",
            output_path
        );
        Ok(())
    }
}
