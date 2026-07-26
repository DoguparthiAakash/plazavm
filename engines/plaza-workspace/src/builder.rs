//! Workspace Builder & Directory Hierarchy Provisioner.

use super::model::{Workspace, WorkspaceSpec};
use plaza_foundation::core::paths;
use plaza_foundation::core::PlazaResult;
use std::fs;
use std::path::PathBuf;

/// Constructs a fully provisioned Workspace OS directory layout.
pub struct WorkspaceBuilder;

impl WorkspaceBuilder {
    /// Builds and provisions the standardized workspace directory structure (`.space/`).
    pub fn build(
        name: impl Into<String>,
        spec: WorkspaceSpec,
    ) -> PlazaResult<(Workspace, PathBuf)> {
        let workspace = Workspace::new(name, spec);
        let root_dir = paths::workspaces_dir().join(workspace.id.to_string());
        let space_dir = root_dir.join(".space");

        // Create standard directory tree
        fs::create_dir_all(root_dir.join("src"))?;
        fs::create_dir_all(&space_dir)?;
        fs::create_dir_all(space_dir.join("config"))?;
        fs::create_dir_all(space_dir.join("runtime"))?;
        fs::create_dir_all(space_dir.join("sessions"))?;
        fs::create_dir_all(space_dir.join("cache"))?;
        fs::create_dir_all(space_dir.join("backend"))?;
        fs::create_dir_all(space_dir.join("mounts"))?;
        fs::create_dir_all(space_dir.join("locks"))?;
        fs::create_dir_all(space_dir.join("registry"))?;
        fs::create_dir_all(space_dir.join("logs"))?;
        fs::create_dir_all(space_dir.join("telemetry"))?;
        fs::create_dir_all(space_dir.join("images"))?;
        fs::create_dir_all(space_dir.join("snapshots"))?;
        fs::create_dir_all(space_dir.join("plugins"))?;
        fs::create_dir_all(space_dir.join("env"))?;
        fs::create_dir_all(space_dir.join("sockets"))?;
        fs::create_dir_all(space_dir.join("state"))?;

        // Backward compatibility: also ensure .plaza dir exists
        let plaza_dir = root_dir.join(".plaza");
        fs::create_dir_all(&plaza_dir)?;

        // Write workspace configuration manifest at PROJECT ROOT (workspace.yaml)
        let root_manifest_path = root_dir.join("workspace.yaml");
        let manifest_content = serde_yaml::to_string(&workspace).map_err(|e| {
            plaza_foundation::core::PlazaError::serialization(format!(
                "Failed to serialize workspace.yaml: {}",
                e
            ))
        })?;
        fs::write(&root_manifest_path, &manifest_content)?;
        // Keep copy in .space/runtime/ and .plaza/ for runtime & backward compatibility
        fs::write(
            space_dir.join("runtime").join("compiled_manifest.yaml"),
            &manifest_content,
        )?;
        fs::write(plaza_dir.join("plaza.yaml"), manifest_content)?;

        // Write lockfile at PROJECT ROOT (workspace.lock)
        let root_lock_path = root_dir.join("workspace.lock");
        let lock_content = format!(
            "# PlazaVM Workspace Lockfile (Auto-generated)\nworkspace_id: {}\ncreated_at: {}\n",
            workspace.id,
            plaza_foundation::core::types::Timestamp::now()
        );
        fs::write(&root_lock_path, &lock_content)?;

        // Write .gitignore inside root_dir if not present
        let gitignore_path = root_dir.join(".gitignore");
        if !gitignore_path.exists() {
            let gitignore_content =
                ".space/logs/\n.space/cache/\n.space/temp/\n.space/sockets/\n.space/runtime/\n";
            let _ = fs::write(gitignore_path, gitignore_content);
        }

        Ok((workspace, root_dir))
    }
}

