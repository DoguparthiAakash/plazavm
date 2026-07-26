//! Workspace Session Engine & Lifecycle Manager.

use plaza_foundation::core::id::{DriverId, RuntimeBackendKind, WorkspaceId};
use plaza_foundation::core::types::Timestamp;
use plaza_foundation::core::PlazaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Status of an active or saved Workspace Session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionStatus {
    Active,
    Suspended,
    Terminated,
}

/// Persistent Workspace Session object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSession {
    pub session_id: Uuid,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub created_at: Timestamp,
    pub last_active_at: Timestamp,
    pub backend_kind: RuntimeBackendKind,
    pub driver_id: DriverId,
    pub current_dir: PathBuf,
    pub environment: HashMap<String, String>,
    pub active_services: Vec<String>,
    pub command_history: Vec<String>,
    pub status: SessionStatus,
}

impl WorkspaceSession {
    pub fn new(
        workspace_id: WorkspaceId,
        name: impl Into<String>,
        backend_kind: RuntimeBackendKind,
        driver_id: DriverId,
        current_dir: PathBuf,
    ) -> Self {
        Self {
            session_id: Uuid::new_v4(),
            workspace_id,
            name: name.into(),
            created_at: Timestamp::now(),
            last_active_at: Timestamp::now(),
            backend_kind,
            driver_id,
            current_dir,
            environment: HashMap::new(),
            active_services: Vec::new(),
            command_history: Vec::new(),
            status: SessionStatus::Active,
        }
    }

    pub fn record_command(&mut self, command: impl Into<String>) {
        self.command_history.push(command.into());
        self.last_active_at = Timestamp::now();
    }
}

/// Single telemetry record in `.space/sessions/history.jsonl`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredCommandEntry {
    pub timestamp: Timestamp,
    pub command: String,
    pub exit_code: i32,
    pub duration_ms: u64,
    pub workspace_id: WorkspaceId,
    pub session_id: Uuid,
    pub backend: String,
    pub cwd: PathBuf,
}

/// Manages persistence, suspension, resumption, and switching of workspace sessions.
pub struct SessionManager;

impl SessionManager {
    /// Saves a workspace session to `.space/sessions/<session_id>.json`.
    pub fn save_session(space_dir: &Path, session: &WorkspaceSession) -> PlazaResult<()> {
        let sessions_dir = space_dir.join("sessions");
        fs::create_dir_all(&sessions_dir)?;
        let file_path = sessions_dir.join(format!("{}.json", session.session_id));
        let content = serde_json::to_string_pretty(session).map_err(|e| {
            plaza_foundation::core::PlazaError::serialization(format!("Failed to serialize session: {}", e))
        })?;
        fs::write(file_path, content)?;

        // Update current active session link/pointer
        let active_pointer = sessions_dir.join("active.id");
        fs::write(active_pointer, session.session_id.to_string())?;

        Ok(())
    }

    /// Appends a structured telemetry record to `.space/sessions/history.jsonl`.
    pub fn append_history_jsonl(
        space_dir: &Path,
        entry: &StructuredCommandEntry,
    ) -> PlazaResult<()> {
        let sessions_dir = space_dir.join("sessions");
        fs::create_dir_all(&sessions_dir)?;
        let history_file = sessions_dir.join("history.jsonl");

        let line = serde_json::to_string(entry).map_err(|e| {
            plaza_foundation::core::PlazaError::serialization(format!(
                "Failed to serialize history entry: {}",
                e
            ))
        })?;

        use std::io::Write;
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(history_file)?;
        writeln!(file, "{}", line)?;

        Ok(())
    }

    /// Loads the active session pointer from `.space/sessions/active.id`.
    pub fn load_active_session(space_dir: &Path) -> PlazaResult<Option<WorkspaceSession>> {
        let sessions_dir = space_dir.join("sessions");
        let active_pointer = sessions_dir.join("active.id");
        if !active_pointer.exists() {
            return Ok(None);
        }

        let session_id_str = fs::read_to_string(active_pointer)?.trim().to_string();
        let session_file = sessions_dir.join(format!("{}.json", session_id_str));
        if !session_file.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(session_file)?;
        let session: WorkspaceSession = serde_json::from_str(&content).map_err(|e| {
            plaza_foundation::core::PlazaError::serialization(format!("Failed to deserialize session: {}", e))
        })?;

        Ok(Some(session))
    }

    /// Lists all sessions stored under `.space/sessions/`.
    pub fn list_sessions(space_dir: &Path) -> PlazaResult<Vec<WorkspaceSession>> {
        let sessions_dir = space_dir.join("sessions");
        if !sessions_dir.exists() {
            return Ok(Vec::new());
        }

        let mut sessions = Vec::new();
        for entry in fs::read_dir(sessions_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(session) = serde_json::from_str::<WorkspaceSession>(&content) {
                        sessions.push(session);
                    }
                }
            }
        }
        Ok(sessions)
    }
}

