//! Workspace Builder & Directory Hierarchy Provisioner.

use super::model::{Workspace, WorkspaceSpec};
use plaza_core::paths;
use plaza_core::PlazaResult;
use std::fs;
use std::path::PathBuf;

/// Constructs a fully provisioned Workspace OS directory layout.
pub struct WorkspaceBuilder;

impl WorkspaceBuilder {
    /// Builds and provisions the standardized workspace directory structure.
    pub fn build(name: impl Into<String>, spec: WorkspaceSpec) -> PlazaResult<(Workspace, PathBuf)> {
        let workspace = Workspace::new(name, spec);
        let root_dir = paths::workspaces_dir().join(workspace.id.to_string());
        let plaza_dir = root_dir.join(".plaza");

        // Create standard directory tree
        fs::create_dir_all(root_dir.join("src"))?;
        fs::create_dir_all(&plaza_dir)?;
        fs::create_dir_all(plaza_dir.join("config"))?;
        fs::create_dir_all(plaza_dir.join("services"))?;
        fs::create_dir_all(plaza_dir.join("logs"))?;
        fs::create_dir_all(plaza_dir.join("cache"))?;
        fs::create_dir_all(plaza_dir.join("models"))?;
        fs::create_dir_all(plaza_dir.join("datasets"))?;
        fs::create_dir_all(plaza_dir.join("artifacts"))?;
        fs::create_dir_all(plaza_dir.join("snapshots"))?;
        fs::create_dir_all(plaza_dir.join("secrets"))?;
        fs::create_dir_all(plaza_dir.join("temp"))?;

        // Write workspace configuration manifest (plaza.yaml)
        let manifest_path = plaza_dir.join("plaza.yaml");
        let manifest_content = serde_yaml::to_string(&workspace)
            .map_err(|e| plaza_core::PlazaError::serialization(format!("Failed to serialize plaza.yaml: {}", e)))?;
        fs::write(manifest_path, manifest_content)?;

        Ok((workspace, root_dir))
    }
}
