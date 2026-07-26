//! Workspace Process Manager for execution within isolated workspace environments.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessSpec {
    pub id: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub cwd: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessState {
    Pending,
    Running { pid: u32 },
    Exited { code: i32 },
    Failed { reason: String },
}

/// Centralized Process Manager for workspace tasks.
pub struct WorkspaceProcessManager {
    processes: Arc<Mutex<HashMap<String, ProcessState>>>,
}

impl Default for WorkspaceProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceProcessManager {
    pub fn new() -> Self {
        Self {
            processes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, id: String) {
        let mut map = self.processes.lock().unwrap();
        map.insert(id, ProcessState::Pending);
    }

    pub fn update_state(&self, id: &str, state: ProcessState) {
        let mut map = self.processes.lock().unwrap();
        map.insert(id.to_string(), state);
    }

    pub fn get_state(&self, id: &str) -> Option<ProcessState> {
        let map = self.processes.lock().unwrap();
        map.get(id).cloned()
    }

    pub fn list(&self) -> HashMap<String, ProcessState> {
        let map = self.processes.lock().unwrap();
        map.clone()
    }
}

