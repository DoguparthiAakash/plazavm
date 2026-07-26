//! Centralized logging system for PlazaVM.

use crate::paths;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::sync::Mutex;
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref CURRENT_SESSION_ID: String = Uuid::new_v4().to_string();
    static ref LOG_MUTEX: Mutex<()> = Mutex::new(());
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub session_id: String,
    pub correlation_id: Option<String>,
    pub message: String,
}

pub struct Logger;

impl Logger {
    pub fn session_id() -> String {
        CURRENT_SESSION_ID.clone()
    }

    /// Returns the log directory, creating it if needed.
    ///
    /// Delegates to [`paths::log_dir()`] for the canonical path.
    pub fn log_dir() -> PathBuf {
        let dir = paths::log_dir();
        fs::create_dir_all(&dir).ok();
        dir
    }

    pub fn main_log_file() -> PathBuf {
        Self::log_dir().join("plazavm.log")
    }

    pub fn session_log_file() -> PathBuf {
        Self::log_dir().join(format!("session_{}.log", *CURRENT_SESSION_ID))
    }

    pub fn log(level: &str, message: &str, correlation_id: Option<&str>) {
        // A logging system must never crash the process.
        let _guard = match LOG_MUTEX.lock() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };

        let timestamp = chrono::Local::now().to_rfc3339();
        let entry = LogEntry {
            timestamp: timestamp.clone(),
            level: level.to_uppercase(),
            session_id: CURRENT_SESSION_ID.clone(),
            correlation_id: correlation_id.map(|s| s.to_string()),
            message: message.to_string(),
        };

        let formatted = format!(
            "[{}] [{}] [session:{}] [{}] {}\n",
            entry.timestamp,
            entry.level,
            &entry.session_id[..8],
            entry.correlation_id.as_deref().unwrap_or("none"),
            entry.message
        );

        // Write to main log & session log
        for path in [Self::main_log_file(), Self::session_log_file()] {
            if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(path) {
                let _ = f.write_all(formatted.as_bytes());
            }
        }
    }

    pub fn info(message: &str) {
        Self::log("INFO", message, None);
    }

    pub fn warn(message: &str) {
        Self::log("WARN", message, None);
    }

    pub fn error(message: &str) {
        Self::log("ERROR", message, None);
    }

    pub fn debug(message: &str) {
        Self::log("DEBUG", message, None);
    }

    /// Read the most recent log lines without loading the entire file.
    ///
    /// Uses a bounded line-by-line read from the end to avoid OOM on large logs.
    pub fn read_recent_logs(max_lines: usize) -> Vec<String> {
        let path = Self::main_log_file();
        let file = match fs::File::open(&path) {
            Ok(f) => f,
            Err(_) => return vec!["No log file found.".into()],
        };

        let reader = std::io::BufReader::new(file);
        // Collect into a VecDeque-style ring: keep only the last `max_lines`.
        let mut ring: Vec<String> = Vec::with_capacity(max_lines);
        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => continue,
            };
            if ring.len() == max_lines {
                ring.remove(0);
            }
            ring.push(line);
        }

        // Return in reverse-chronological order (newest first).
        ring.reverse();
        ring
    }
}
