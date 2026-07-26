//! Persisted event store for audit trail and replay.

use plaza_foundation::core::id::WorkspaceId;
use plaza_foundation::core::PlazaResult;
use plaza_foundation::events::PlazaEvent;
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
        let conn = self.conn.lock().map_err(|e| {
            plaza_foundation::core::PlazaError::Storage(format!("event store connection lock poisoned: {e}"))
        })?;
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
        .map_err(|e| plaza_foundation::core::PlazaError::Storage(e.to_string()))?;

        Ok(())
    }

    /// Query historical events for a specific workspace.
    pub fn get_workspace_events(&self, id: &WorkspaceId) -> PlazaResult<Vec<PlazaEvent>> {
        let conn = self.conn.lock().map_err(|e| {
            plaza_foundation::core::PlazaError::Storage(format!("event store connection lock poisoned: {e}"))
        })?;
        let mut stmt = conn
            .prepare("SELECT payload_json FROM events WHERE workspace_id = ?1 ORDER BY id ASC")
            .map_err(|e| plaza_foundation::core::PlazaError::Storage(e.to_string()))?;

        let rows = stmt
            .query_map(params![id.to_string()], |row| {
                let payload: String = row.get(0)?;
                Ok(payload)
            })
            .map_err(|e| plaza_foundation::core::PlazaError::Storage(e.to_string()))?;

        let mut events = Vec::new();
        for r in rows {
            let json_str = r.map_err(|e| plaza_foundation::core::PlazaError::Storage(e.to_string()))?;
            if let Ok(ev) = serde_json::from_str::<PlazaEvent>(&json_str) {
                events.push(ev);
            }
        }
        Ok(events)
    }

    /// Retrieve all events globally, optionally starting after a specific event ID.
    /// This is required for system-wide event replay and recovery at boot.
    pub fn get_all_events(&self, since_id: Option<i64>) -> PlazaResult<Vec<(i64, PlazaEvent)>> {
        let conn = self.conn.lock().map_err(|e| {
            plaza_foundation::core::PlazaError::Storage(format!("event store connection lock poisoned: {e}"))
        })?;

        let (query, params_list) = if let Some(id) = since_id {
            ("SELECT id, payload_json FROM events WHERE id > ?1 ORDER BY id ASC", vec![rusqlite::types::Value::Integer(id)])
        } else {
            ("SELECT id, payload_json FROM events ORDER BY id ASC", vec![])
        };

        let mut stmt = conn
            .prepare(query)
            .map_err(|e| plaza_foundation::core::PlazaError::Storage(e.to_string()))?;

        let rows = stmt
            .query_map(rusqlite::params_from_iter(params_list), |row| {
                let id: i64 = row.get(0)?;
                let payload: String = row.get(1)?;
                Ok((id, payload))
            })
            .map_err(|e| plaza_foundation::core::PlazaError::Storage(e.to_string()))?;

        let mut events = Vec::new();
        for r in rows {
            let (id, json_str) = r.map_err(|e| plaza_foundation::core::PlazaError::Storage(e.to_string()))?;
            if let Ok(ev) = serde_json::from_str::<PlazaEvent>(&json_str) {
                events.push((id, ev));
            }
        }
        Ok(events)
    }
}

