//! Persisted event store for audit trail and replay.

use plaza_core::id::WorkspaceId;
use plaza_core::PlazaResult;
use plaza_events::PlazaEvent;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

/// SQLite event store.
#[derive(Clone)]
pub struct SqliteEventStore {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteEventStore {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Store a domain event into the database.
    pub fn append(&self, event: &PlazaEvent) -> PlazaResult<()> {
        let conn = self.conn.lock().unwrap();
        let event_type = event.event_type();
        let payload = serde_json::to_string(event).unwrap_or_default();
        let now = chrono::Utc::now().to_rfc3339();

        let workspace_id = match event {
            PlazaEvent::WorkspaceCreated { id, .. }
            | PlazaEvent::WorkspaceStarting { id }
            | PlazaEvent::WorkspaceStarted { id, .. }
            | PlazaEvent::WorkspaceStopping { id }
            | PlazaEvent::WorkspaceStopped { id }
            | PlazaEvent::WorkspacePaused { id }
            | PlazaEvent::WorkspaceResumed { id }
            | PlazaEvent::WorkspaceError { id, .. }
            | PlazaEvent::WorkspaceHealthChanged { id, .. }
            | PlazaEvent::WorkspaceDeleted { id } => Some(id.to_string()),
            _ => None,
        };

        conn.execute(
            "INSERT INTO events (workspace_id, event_type, payload_json, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![workspace_id, event_type, payload, now],
        )
        .map_err(|e| plaza_core::PlazaError::Storage(e.to_string()))?;

        Ok(())
    }

    /// Query historical events for a specific workspace.
    pub fn get_workspace_events(&self, id: &WorkspaceId) -> PlazaResult<Vec<PlazaEvent>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT payload_json FROM events WHERE workspace_id = ?1 ORDER BY id ASC")
            .map_err(|e| plaza_core::PlazaError::Storage(e.to_string()))?;

        let rows = stmt
            .query_map(params![id.to_string()], |row| {
                let payload: String = row.get(0)?;
                Ok(payload)
            })
            .map_err(|e| plaza_core::PlazaError::Storage(e.to_string()))?;

        let mut events = Vec::new();
        for r in rows {
            let json_str = r.map_err(|e| plaza_core::PlazaError::Storage(e.to_string()))?;
            if let Ok(ev) = serde_json::from_str::<PlazaEvent>(&json_str) {
                events.push(ev);
            }
        }
        Ok(events)
    }
}
