//! Workspace Service Manager for scoped background services.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSpec {
    pub name: String,
    pub image: String,
    pub ports: Vec<u16>,
    pub environment: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceStatus {
    Stopped,
    Starting,
    Running,
    Error(String),
}

/// Centralized Manager for workspace services (PostgreSQL, Redis, Ollama, Nginx, etc.).
pub struct WorkspaceServiceManager {
    services: Arc<Mutex<HashMap<String, ServiceStatus>>>,
}

impl Default for WorkspaceServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceServiceManager {
    pub fn new() -> Self {
        Self {
            services: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, name: String) {
        let mut map = self.services.lock().unwrap();
        map.insert(name, ServiceStatus::Stopped);
    }

    pub fn set_status(&self, name: &str, status: ServiceStatus) {
        let mut map = self.services.lock().unwrap();
        map.insert(name.to_string(), status);
    }

    pub fn get_status(&self, name: &str) -> ServiceStatus {
        let map = self.services.lock().unwrap();
        map.get(name).cloned().unwrap_or(ServiceStatus::Stopped)
    }

    pub fn list_services(&self) -> HashMap<String, ServiceStatus> {
        let map = self.services.lock().unwrap();
        map.clone()
    }
}
