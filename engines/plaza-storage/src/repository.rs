//! Workspace SQLite repository implementation.

use crate::migrations::run_migrations;
use plaza_foundation::core::id::WorkspaceId;
use plaza_foundation::core::{PlazaError, PlazaResult};
use rusqlite::{params, Connection};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Thread-safe SQLite workspace repository.
#[derive(Clone)]
pub struct SqliteWorkspaceRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteWorkspaceRepository {
    /// Connect to SQLite database file or memory and run migrations.
    pub fn open(db_path: PathBuf) -> PlazaResult<Self> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&db_path)
            .map_err(|e| PlazaError::Storage(format!("failed to open sqlite DB: {e}")))?;

        // Enable Write-Ahead Logging and Foreign Keys for production readiness
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;"
        ).map_err(|e| PlazaError::Storage(format!("failed to set pragmas: {e}")))?;

        run_migrations(&conn)?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Open in-memory SQLite database (for tests).
    pub fn open_in_memory() -> PlazaResult<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|e| PlazaError::Storage(format!("failed to open memory DB: {e}")))?;

        conn.execute_batch("PRAGMA foreign_keys = ON;")
            .map_err(|e| PlazaError::Storage(format!("failed to set pragmas: {e}")))?;

        run_migrations(&conn)?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// Save or update raw workspace records.
    pub fn save_raw(
        &self,
        id: &WorkspaceId,
        name: &str,
        description: Option<&str>,
        spec_json: &str,
        status_json: &str,
        metadata_json: &str,
    ) -> PlazaResult<()> {
        let conn = self.conn.lock().map_err(|e| {
            PlazaError::Storage(format!("repository connection lock poisoned: {e}"))
        })?;
        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            r#"
            INSERT INTO workspaces (id, name, description, spec_json, status_json, metadata_json, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?7)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                description = excluded.description,
                spec_json = excluded.spec_json,
                status_json = excluded.status_json,
                metadata_json = excluded.metadata_json,
                updated_at = excluded.updated_at
            "#,
            params![id.to_string(), name, description, spec_json, status_json, metadata_json, now],
        ).map_err(|e| PlazaError::Storage(e.to_string()))?;

        Ok(())
    }

    /// Retrieve a raw workspace record by ID.
    #[allow(clippy::type_complexity)]
    pub fn get_raw(
        &self,
        id: &WorkspaceId,
    ) -> PlazaResult<Option<(String, Option<String>, String, String, String)>> {
        let conn = self.conn.lock().map_err(|e| {
            PlazaError::Storage(format!("repository connection lock poisoned: {e}"))
        })?;
        let mut stmt = conn
            .prepare("SELECT name, description, spec_json, status_json, metadata_json FROM workspaces WHERE id = ?1")
            .map_err(|e| PlazaError::Storage(e.to_string()))?;

        let mut rows = stmt
            .query(params![id.to_string()])
            .map_err(|e| PlazaError::Storage(e.to_string()))?;

        if let Some(row) = rows
            .next()
            .map_err(|e| PlazaError::Storage(e.to_string()))?
        {
            let name: String = row.get(0).unwrap();
            let desc: Option<String> = row.get(1).unwrap();
            let spec: String = row.get(2).unwrap();
            let status: String = row.get(3).unwrap();
            let meta: String = row.get(4).unwrap();
            Ok(Some((name, desc, spec, status, meta)))
        } else {
            Ok(None)
        }
    }

    /// Delete a workspace record.
    pub fn delete(&self, id: &WorkspaceId) -> PlazaResult<()> {
        let conn = self.conn.lock().map_err(|e| {
            PlazaError::Storage(format!("repository connection lock poisoned: {e}"))
        })?;
        conn.execute(
            "DELETE FROM workspaces WHERE id = ?1",
            params![id.to_string()],
        )
        .map_err(|e| PlazaError::Storage(e.to_string()))?;
        Ok(())
    }

    /// List all workspace records raw tuples.
    #[allow(clippy::type_complexity)]
    pub fn list_raw(
        &self,
    ) -> PlazaResult<Vec<(String, String, Option<String>, String, String, String)>> {
        let conn = self.conn.lock().map_err(|e| {
            PlazaError::Storage(format!("repository connection lock poisoned: {e}"))
        })?;
        let mut stmt = conn
            .prepare("SELECT id, name, description, spec_json, status_json, metadata_json FROM workspaces")
            .map_err(|e| PlazaError::Storage(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            })
            .map_err(|e| PlazaError::Storage(e.to_string()))?;

        let mut res = Vec::new();
        for r in rows {
            res.push(r.map_err(|e| PlazaError::Storage(e.to_string()))?);
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_crud_operations() {
        let repo = SqliteWorkspaceRepository::open_in_memory().unwrap();
        let id = WorkspaceId::new();

        repo.save_raw(&id, "dev", Some("desc"), "{}", "{}", "{}")
            .unwrap();
        let fetched = repo.get_raw(&id).unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().0, "dev");

        let list = repo.list_raw().unwrap();
        assert_eq!(list.len(), 1);

        repo.delete(&id).unwrap();
        assert!(repo.get_raw(&id).unwrap().is_none());
    }
}

