//! Workspace snapshot and restore.
//!
//! Manages snapshots of workspace state using COW block devices.
//! Snapshots capture the writable layer state at a point in time.

use plaza_foundation::core::{PlazaError, PlazaResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::{debug, info};

/// A snapshot of workspace state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSnapshot {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub size_bytes: u64,
    pub parent_snapshot_id: Option<String>,
    pub tags: Vec<String>,
}

/// Manages workspace snapshots.
pub struct SnapshotManager {
    storage_dir: PathBuf,
}

impl SnapshotManager {
    pub fn new(storage_dir: PathBuf) -> Self {
        Self { storage_dir }
    }

    /// Create a snapshot of the workspace writable layer.
    pub async fn create_snapshot(
        &self,
        workspace_id: &str,
        name: &str,
        description: Option<&str>,
        tags: Vec<String>,
    ) -> PlazaResult<WorkspaceSnapshot> {
        let snapshot_id = format!("snap-{}", uuid::Uuid::new_v4());
        let ws_dir = self.storage_dir.join(workspace_id);

        if !ws_dir.exists() {
            return Err(PlazaError::storage(format!(
                "Workspace '{}' storage not found",
                workspace_id
            )));
        }

        // Find the workspace writable image
        let mut entries = tokio::fs::read_dir(&ws_dir)
            .await
            .map_err(PlazaError::Io)?;

        let mut ws_image = None;
        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(PlazaError::Io)?
        {
            let path = entry.path();
            if path
                .extension()
                .map_or(false, |e| e == "img" || e == "ext4")
            {
                ws_image = Some(path);
                break;
            }
        }

        let ws_image = ws_image.ok_or_else(|| {
            PlazaError::storage(format!(
                "No writable image found for workspace '{}'",
                workspace_id
            ))
        })?;

        // Create snapshot by copying the COW image
        let snapshot_dir = ws_dir.join("snapshots");
        tokio::fs::create_dir_all(&snapshot_dir)
            .await
            .map_err(PlazaError::Io)?;

        let snapshot_path = snapshot_dir.join(format!("{}.img", snapshot_id));

        info!(
            "Creating snapshot '{}' for workspace '{}'",
            name, workspace_id
        );

        // Copy the COW image (this captures the current state)
        tokio::fs::copy(&ws_image, &snapshot_path)
            .await
            .map_err(PlazaError::Io)?;

        let size_bytes = tokio::fs::metadata(&snapshot_path)
            .await
            .map(|m| m.len())
            .unwrap_or(0);

        let snapshot = WorkspaceSnapshot {
            id: snapshot_id.clone(),
            workspace_id: workspace_id.to_string(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            created_at: chrono::Utc::now().to_rfc3339(),
            size_bytes,
            parent_snapshot_id: None,
            tags,
        };

        // Persist snapshot metadata
        let meta_path = snapshot_dir.join(format!("{}.json", snapshot_id));
        let json = serde_json::to_string_pretty(&snapshot)
            .map_err(|e| PlazaError::serialization(e.to_string()))?;
        tokio::fs::write(&meta_path, json)
            .await
            .map_err(PlazaError::Io)?;

        info!(
            "Snapshot '{}' created: {} bytes",
            name,
            size_bytes
        );

        Ok(snapshot)
    }

    /// Restore a workspace to a snapshot.
    pub async fn restore_snapshot(
        &self,
        workspace_id: &str,
        snapshot_id: &str,
    ) -> PlazaResult<()> {
        let ws_dir = self.storage_dir.join(workspace_id);
        let snapshots_dir = ws_dir.join("snapshots");
        let snapshot_path = snapshots_dir.join(format!("{}.img", snapshot_id));

        if !snapshot_path.exists() {
            return Err(PlazaError::storage(format!(
                "Snapshot '{}' not found",
                snapshot_id
            )));
        }

        // Find the current workspace writable image
        let mut entries = tokio::fs::read_dir(&ws_dir)
            .await
            .map_err(PlazaError::Io)?;

        let mut ws_image = None;
        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(PlazaError::Io)?
        {
            let path = entry.path();
            if path
                .extension()
                .map_or(false, |e| e == "img" || e == "ext4")
            {
                ws_image = Some(path);
                break;
            }
        }

        let ws_image = ws_image.ok_or_else(|| {
            PlazaError::storage(format!(
                "No writable image found for workspace '{}'",
                workspace_id
            ))
        })?;

        info!(
            "Restoring snapshot '{}' for workspace '{}'",
            snapshot_id, workspace_id
        );

        // Restore by copying the snapshot over the current image
        // The workspace must be stopped first for consistency
        tokio::fs::copy(&snapshot_path, &ws_image)
            .await
            .map_err(PlazaError::Io)?;

        info!("Snapshot '{}' restored", snapshot_id);

        Ok(())
    }

    /// List all snapshots for a workspace.
    pub async fn list_snapshots(&self, workspace_id: &str) -> PlazaResult<Vec<WorkspaceSnapshot>> {
        let ws_dir = self.storage_dir.join(workspace_id);
        let snapshots_dir = ws_dir.join("snapshots");

        if !snapshots_dir.exists() {
            return Ok(Vec::new());
        }

        let mut snapshots = Vec::new();
        let mut entries = tokio::fs::read_dir(&snapshots_dir)
            .await
            .map_err(PlazaError::Io)?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(PlazaError::Io)?
        {
            let path = entry.path();
            if path
                .extension()
                .map_or(false, |e| e == "json")
            {
                if let Ok(content) = tokio::fs::read_to_string(&path).await {
                    if let Ok(snap) = serde_json::from_str::<WorkspaceSnapshot>(&content) {
                        snapshots.push(snap);
                    }
                }
            }
        }

        snapshots.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(snapshots)
    }

    /// Delete a snapshot.
    pub async fn delete_snapshot(
        &self,
        workspace_id: &str,
        snapshot_id: &str,
    ) -> PlazaResult<()> {
        let ws_dir = self.storage_dir.join(workspace_id);
        let snapshots_dir = ws_dir.join("snapshots");

        let image_path = snapshots_dir.join(format!("{}.img", snapshot_id));
        let meta_path = snapshots_dir.join(format!("{}.json", snapshot_id));

        if image_path.exists() {
            tokio::fs::remove_file(&image_path)
                .await
                .map_err(PlazaError::Io)?;
        }
        if meta_path.exists() {
            tokio::fs::remove_file(&meta_path)
                .await
                .map_err(PlazaError::Io)?;
        }

        info!("Deleted snapshot '{}'", snapshot_id);
        Ok(())
    }

    /// Get snapshot details.
    pub async fn get_snapshot(
        &self,
        workspace_id: &str,
        snapshot_id: &str,
    ) -> PlazaResult<WorkspaceSnapshot> {
        let ws_dir = self.storage_dir.join(workspace_id);
        let snapshots_dir = ws_dir.join("snapshots");
        let meta_path = snapshots_dir.join(format!("{}.json", snapshot_id));

        if !meta_path.exists() {
            return Err(PlazaError::storage(format!(
                "Snapshot '{}' not found",
                snapshot_id
            )));
        }

        let content = tokio::fs::read_to_string(&meta_path)
            .await
            .map_err(PlazaError::Io)?;

        serde_json::from_str(&content)
            .map_err(|e| PlazaError::serialization(e.to_string()))
    }

    /// Calculate total snapshot storage usage.
    pub async fn calculate_usage(&self, workspace_id: &str) -> PlazaResult<u64> {
        let snapshots = self.list_snapshots(workspace_id).await?;
        let total: u64 = snapshots.iter().map(|s| s.size_bytes).sum();
        Ok(total)
    }
}
