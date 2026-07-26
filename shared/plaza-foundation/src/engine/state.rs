use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineCheckpoint {
    pub timestamp: String,
    pub active_workspaces: usize,
}

pub struct EngineStateStore;

impl EngineStateStore {
    pub fn new() -> Self {
        Self
    }

    pub fn checkpoint(&self, active: usize) -> EngineCheckpoint {
        EngineCheckpoint {
            timestamp: chrono::Utc::now().to_rfc3339(),
            active_workspaces: active,
        }
    }
}

impl Default for EngineStateStore {
    fn default() -> Self {
        Self::new()
    }
}
