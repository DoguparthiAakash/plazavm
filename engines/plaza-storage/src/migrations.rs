//! SQLite schema migrations.

use plaza_foundation::core::PlazaResult;
use rusqlite::Connection;
use tracing::info;

const MIGRATIONS: &[&str] = &[
    // Migration 1: Initial V2 Schema
    r#"
    CREATE TABLE IF NOT EXISTS workspaces (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL UNIQUE,
        description TEXT,
        spec_json TEXT NOT NULL,
        status_json TEXT NOT NULL,
        metadata_json TEXT NOT NULL,
        created_at TEXT NOT NULL,
        updated_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS events (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        workspace_id TEXT,
        event_type TEXT NOT NULL,
        payload_json TEXT NOT NULL,
        created_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS plugins (
        id TEXT PRIMARY KEY,
        manifest_json TEXT NOT NULL,
        enabled INTEGER NOT NULL DEFAULT 1,
        installed_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS snapshots (
        id TEXT PRIMARY KEY,
        workspace_id TEXT NOT NULL,
        tag TEXT NOT NULL,
        runtime_snapshot_id TEXT,
        created_at TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS schema_migrations (
        version INTEGER PRIMARY KEY,
        applied_at TEXT NOT NULL
    );
    "#,
];

pub fn run_migrations(conn: &Connection) -> PlazaResult<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS schema_migrations (version INTEGER PRIMARY KEY, applied_at TEXT NOT NULL);"
    ).map_err(|e| plaza_foundation::core::PlazaError::Storage(e.to_string()))?;

    let mut stmt = conn
        .prepare("SELECT MAX(version) FROM schema_migrations")
        .map_err(|e| plaza_foundation::core::PlazaError::Storage(e.to_string()))?;

    let current_version: Option<i32> = stmt
        .query_row([], |row| row.get(0))
        .map_err(|e| plaza_foundation::core::PlazaError::Storage(e.to_string()))?;

    let current = current_version.unwrap_or(0);

    for (idx, migration) in MIGRATIONS.iter().enumerate() {
        let version = (idx + 1) as i32;
        if version > current {
            info!(version, "applying database migration");
            conn.execute_batch(migration)
                .map_err(|e| plaza_foundation::core::PlazaError::Storage(e.to_string()))?;

            let now = chrono::Utc::now().to_rfc3339();
            conn.execute(
                "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
                rusqlite::params![version, now],
            )
            .map_err(|e| plaza_foundation::core::PlazaError::Storage(e.to_string()))?;
        }
    }

    Ok(())
}

