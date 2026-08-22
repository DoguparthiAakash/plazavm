//! Builds bootable OCI container images into PlazaVM virtual disks.
//!
//! This module parses standard Docker/OCI image manifests and layers,
//! extracts their `.tar.gz` files into a virtual filesystem tree,
//! and orchestrates them using overlayfs-like blocks for `plaza-qemu`.

use plaza_foundation::core::{PlazaResult, PlazaError};
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

    /// Pull the requested image from the OCI registry (e.g. Docker Hub)
    /// and prepare its layers in the local cache.
    pub async fn prepare_layers(&self) -> PlazaResult<Vec<PathBuf>> {
        // TODO: Implement OCI pull via plaza-registry
        println!("(Image Builder) Pulling OCI image layers for '{}'", self.target_image);
        Ok(vec![])
    }

    /// Flattens the extracted OCI layers into a `.img` block device compatible with QEMU/v86.
    pub async fn compose_block_device(&self, _layers: Vec<PathBuf>, output_path: &PathBuf) -> PlazaResult<()> {
        // TODO: Implement actual `.tar.gz` flattening into the loopback block device
        println!("(Image Builder) Composing OCI layers into raw block device at {:?}", output_path);
        Ok(())
    }
}
