//! Global panic and crash handling system for PlazaVM.

use crate::paths;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReport {
    pub id: String,
    pub timestamp: String,
    pub version: String,
    pub panic_message: String,
    pub location: String,
    pub backtrace: String,
    pub os: String,
}

pub struct CrashHandler;

impl CrashHandler {
    pub fn crash_dir() -> PathBuf {
        let dir = paths::data_dir().join("crashes");
        fs::create_dir_all(&dir).ok();
        dir
    }

    pub fn init() {
        std::panic::set_hook(Box::new(|info| {
            let timestamp = chrono::Local::now().to_rfc3339();
            let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = info.payload().downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic payload".to_string()
            };

            let location = if let Some(loc) = info.location() {
                format!("{}:{}:{}", loc.file(), loc.line(), loc.column())
            } else {
                "Unknown location".to_string()
            };

            let report = CrashReport {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: timestamp.clone(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                panic_message: payload.clone(),
                location: location.clone(),
                backtrace: format!("{:?}", std::backtrace::Backtrace::capture()),
                os: std::env::consts::OS.to_string(),
            };

            let filename = format!("panic_{}.json", timestamp.replace([':', '-'], "_"));
            let crash_file = Self::crash_dir().join(filename);
            if let Ok(json) = serde_json::to_string_pretty(&report) {
                let _ = fs::write(&crash_file, json);
            }

            crate::logging::Logger::error(&format!(
                "CRASH PANIC DETECTED: {payload} at {location}"
            ));
        }));
    }

    pub fn list_crash_reports() -> Vec<CrashReport> {
        let dir = Self::crash_dir();
        let mut reports = Vec::new();

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if entry.path().extension().is_some_and(|e| e == "json") {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(report) = serde_json::from_str::<CrashReport>(&content) {
                            reports.push(report);
                        }
                    }
                }
            }
        }

        reports
    }
}
