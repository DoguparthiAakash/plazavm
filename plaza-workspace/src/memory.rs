//! Workspace Memory Manager (Structured Knowledge & Telemetry Store).

use plaza_core::PlazaResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Structured workspace knowledge & telemetry memory object.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceMemory {
    pub project_name: String,
    pub preferred_runtime: String,
    pub installed_sdks: Vec<String>,
    pub package_graph: HashMap<String, String>,
    pub environment_keys: Vec<String>,
    pub services_configured: Vec<String>,
    pub custom_scripts: HashMap<String, String>,
    pub workspace_aliases: HashMap<String, String>,
    pub toolchain_versions: HashMap<String, String>,
    pub frequent_commands: Vec<(String, u32)>,
    pub build_cache_bytes: u64,
}

/// Workspace Memory Manager.
pub struct WorkspaceMemoryManager;

impl WorkspaceMemoryManager {
    /// Loads workspace memory from `.space/state/memory.json`.
    pub fn load(space_dir: &Path) -> PlazaResult<WorkspaceMemory> {
        let memory_file = space_dir.join("state").join("memory.json");
        if !memory_file.exists() {
            return Ok(WorkspaceMemory::default());
        }

        let content = fs::read_to_string(memory_file)?;
        let memory: WorkspaceMemory = serde_json::from_str(&content).map_err(|e| {
            plaza_core::PlazaError::serialization(format!(
                "Failed to deserialize workspace memory: {}",
                e
            ))
        })?;
        Ok(memory)
    }

    /// Saves workspace memory to `.space/state/memory.json`.
    pub fn save(space_dir: &Path, memory: &WorkspaceMemory) -> PlazaResult<()> {
        let state_dir = space_dir.join("state");
        fs::create_dir_all(&state_dir)?;
        let memory_file = state_dir.join("memory.json");
        let content = serde_json::to_string_pretty(memory).map_err(|e| {
            plaza_core::PlazaError::serialization(format!(
                "Failed to serialize workspace memory: {}",
                e
            ))
        })?;
        fs::write(memory_file, content)?;
        Ok(())
    }
}
