//! BlendOS abstractions for multi-distribution workspaces.
//!
//! This module provides the capability to layer multiple package managers
//! and features from different operating systems (e.g., apt from Debian,
//! dnf from Fedora) into a single cohesive `.img` overlay root filesystem.

use plaza_foundation::core::PlazaResult;
use std::path::PathBuf;

/// Defines a blended layer from a specific OS distribution.
#[derive(Debug, Clone)]
pub struct BlendLayer {
    /// The name of the distribution (e.g., "ubuntu", "fedora", "alpine").
    pub distribution: String,

    /// The specific package manager to map into the workspace (e.g., "apt", "dnf", "apk").
    pub package_manager: String,

    /// Priority in the overlayfs stack (lower number = higher priority).
    pub priority: u32,

    /// Path to the extracted root filesystem of this distribution layer.
    pub source_path: PathBuf,
}

/// Orchestrator for blending multiple OS features.
pub struct BlendOrchestrator {
    layers: Vec<BlendLayer>,
}

impl BlendOrchestrator {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    /// Add a new OS layer to the blend stack.
    pub fn add_layer(&mut self, layer: BlendLayer) {
        self.layers.push(layer);
        self.layers.sort_by(|a, b| a.priority.cmp(&b.priority));
    }

    /// Compute the overlayfs mount parameters for the blended layers.
    pub fn compute_overlay_args(
        &self,
        workdir: &PathBuf,
        upperdir: &PathBuf,
    ) -> PlazaResult<String> {
        let mut lowerdirs = Vec::new();
        for layer in &self.layers {
            lowerdirs.push(layer.source_path.to_string_lossy().to_string());
        }

        // Reverse lowerdirs because overlayfs processes right-to-left
        lowerdirs.reverse();

        let lowerdir_arg = lowerdirs.join(":");
        Ok(format!(
            "lowerdir={},upperdir={},workdir={}",
            lowerdir_arg,
            upperdir.display(),
            workdir.display()
        ))
    }
}
