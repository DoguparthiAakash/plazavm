//! Crash recovery and integrity validation for workspaces.
//!
//! The RecoveryManager is invoked by the reconciler when it detects an
//! Error state. It attempts to bring the workspace back to a known-good
//! Stopped state by validating filesystem integrity, resetting stale locks,
//! and clearing corrupted transient state.

use crate::model::{Workspace, WorkspaceState};
use crate::reconciler::validate_transition;
use plaza_foundation::core::PlazaResult;
use tracing::{info, warn};

/// Analyzes workspaces and orchestrates recovery procedures after crashes or corruption.
pub struct RecoveryManager;

impl Default for RecoveryManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RecoveryManager {
    pub fn new() -> Self {
        Self
    }

    /// Inspects the workspace state and attempts to recover consistency.
    ///
    /// Recovery is only attempted when the workspace is in `Error` state.
    /// On success, the workspace is transitioned to `Stopped`.
    pub async fn attempt_recovery(&self, workspace: &mut Workspace) -> PlazaResult<bool> {
        info!(
            workspace = %workspace.name,
            id = %workspace.id,
            state = %workspace.status.state,
            "attempting recovery"
        );

        if workspace.status.state != WorkspaceState::Error {
            return Ok(false);
        }

        // Validate that Error -> Stopped is a legal transition.
        validate_transition(WorkspaceState::Error, WorkspaceState::Stopped)?;

        // Phase 1: Clear stale lock files.
        self.clear_stale_locks(workspace).await?;

        // Phase 2: Validate filesystem integrity.
        self.validate_integrity(workspace).await?;

        // Phase 3: Reset state.
        workspace.status.state = WorkspaceState::Stopped;
        workspace.status.message = Some("Recovered from Error state".to_string());

        warn!(
            workspace = %workspace.name,
            "recovery succeeded, workspace moved to Stopped"
        );

        Ok(true)
    }

    /// Validates the filesystem integrity of the workspace directory.
    pub async fn validate_integrity(&self, workspace: &Workspace) -> PlazaResult<()> {
        let ws_dir = plaza_foundation::core::paths::workspaces_dir().join(&workspace.name);

        // Check critical paths exist.
        let critical_dirs = [".plaza", "runtime", "config"];
        for dir_name in &critical_dirs {
            let path = ws_dir.join(dir_name);
            if ws_dir.exists() && !path.exists() {
                warn!(
                    workspace = %workspace.name,
                    missing_dir = dir_name,
                    "missing critical directory during integrity check"
                );
                // For DP1, we create missing directories rather than failing.
                std::fs::create_dir_all(&path).map_err(|e| {
                    plaza_foundation::core::PlazaError::storage(format!(
                        "failed to recreate {}: {}",
                        dir_name, e
                    ))
                })?;
            }
        }

        Ok(())
    }

    /// Removes stale lock files that may have survived a crash.
    async fn clear_stale_locks(&self, workspace: &Workspace) -> PlazaResult<()> {
        let locks_dir = plaza_foundation::core::paths::workspaces_dir()
            .join(&workspace.name)
            .join("locks");

        if locks_dir.exists() {
            let entries = std::fs::read_dir(&locks_dir).map_err(|e| {
                plaza_foundation::core::PlazaError::storage(format!(
                    "failed to read locks directory: {}",
                    e
                ))
            })?;

            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "lock") {
                    info!(lock = %path.display(), "removing stale lock file");
                    let _ = std::fs::remove_file(&path);
                }
            }
        }

        Ok(())
    }
}
