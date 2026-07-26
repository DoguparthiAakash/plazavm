//! Workspace State Control (WSC) & Execution State Timeline Engine.

use crate::model::WorkspaceSpec;
use plaza_foundation::core::types::Timestamp;
use plaza_foundation::core::PlazaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// A single immutable workspace execution state commit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceCommit {
    pub commit_id: String,
    pub parent_id: Option<String>,
    pub author: String,
    pub message: String,
    pub timestamp: Timestamp,
    pub manifest_snapshot: WorkspaceSpec,
    pub runtime_version: String,
    pub package_graph: Vec<String>,
    pub environment: HashMap<String, String>,
    pub active_services: Vec<String>,
    pub snapshot_ref: Option<String>,
}

/// Workspace Execution State Timeline.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceTimeline {
    pub head_commit_id: Option<String>,
    pub commits: Vec<WorkspaceCommit>,
}

/// Workspace State Control (WSC) Engine.
pub struct WscEngine;

impl WscEngine {
    /// Creates a new workspace commit and records it in `.space/state/commits/`.
    pub fn commit(
        space_dir: &Path,
        author: impl Into<String>,
        message: impl Into<String>,
        spec: WorkspaceSpec,
        environment: HashMap<String, String>,
        active_services: Vec<String>,
    ) -> PlazaResult<WorkspaceCommit> {
        let state_dir = space_dir.join("state");
        let commits_dir = state_dir.join("commits");
        fs::create_dir_all(&commits_dir)?;

        let timeline = Self::load_timeline(space_dir)?;
        let parent_id = timeline.head_commit_id.clone();

        let msg_str = message.into();
        let raw_bytes = format!(
            "{}:{}:{}",
            Timestamp::now(),
            msg_str,
            parent_id.as_deref().unwrap_or("")
        );
        let hash_bytes = md5::compute(raw_bytes.as_bytes());
        let commit_id = hash_bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        let commit = WorkspaceCommit {
            commit_id: commit_id.clone(),
            parent_id,
            author: author.into(),
            message: msg_str,
            timestamp: Timestamp::now(),
            manifest_snapshot: spec,
            runtime_version: env!("CARGO_PKG_VERSION").to_string(),
            package_graph: Vec::new(),
            environment,
            active_services,
            snapshot_ref: None,
        };

        // Write commit JSON to .space/state/commits/<commit_id>.json
        let file_path = commits_dir.join(format!("{}.json", commit_id));
        let content = serde_json::to_string_pretty(&commit).map_err(|e| {
            plaza_foundation::core::PlazaError::serialization(format!("Failed to serialize commit: {}", e))
        })?;
        fs::write(file_path, content)?;

        // Update timeline HEAD pointer
        let mut new_timeline = timeline;
        new_timeline.head_commit_id = Some(commit_id);
        new_timeline.commits.push(commit.clone());
        Self::save_timeline(space_dir, &new_timeline)?;

        Ok(commit)
    }

    /// Loads the current workspace timeline graph from `.space/state/timeline.json`.
    pub fn load_timeline(space_dir: &Path) -> PlazaResult<WorkspaceTimeline> {
        let timeline_file = space_dir.join("state").join("timeline.json");
        if !timeline_file.exists() {
            return Ok(WorkspaceTimeline::default());
        }

        let content = fs::read_to_string(timeline_file)?;
        let timeline: WorkspaceTimeline = serde_json::from_str(&content).map_err(|e| {
            plaza_foundation::core::PlazaError::serialization(format!("Failed to deserialize timeline: {}", e))
        })?;
        Ok(timeline)
    }

    fn save_timeline(space_dir: &Path, timeline: &WorkspaceTimeline) -> PlazaResult<()> {
        let state_dir = space_dir.join("state");
        fs::create_dir_all(&state_dir)?;
        let timeline_file = state_dir.join("timeline.json");
        let content = serde_json::to_string_pretty(timeline).map_err(|e| {
            plaza_foundation::core::PlazaError::serialization(format!("Failed to serialize timeline: {}", e))
        })?;
        fs::write(timeline_file, content)?;
        Ok(())
    }
}

// Inline fallback md5 hasher
mod md5 {
    pub fn compute(input: &[u8]) -> [u8; 16] {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        input.hash(&mut h);
        let val = h.finish();
        let mut bytes = [0u8; 16];
        bytes[..8].copy_from_slice(&val.to_le_bytes());
        bytes[8..16].copy_from_slice(&val.to_be_bytes());
        bytes
    }
}

