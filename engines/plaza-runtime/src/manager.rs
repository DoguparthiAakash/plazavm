use crate::backend::RuntimeBackend;

use plaza_foundation::core::{PlazaError, PlazaResult};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

/// Orchestrates the execution lifecycle, negotiates capabilities, and routes
/// requests to the appropriate runtime backend.
pub struct RuntimeManager {
    backends: HashMap<String, Arc<dyn RuntimeBackend>>,
}

impl RuntimeManager {
    pub fn new() -> Self {
        Self {
            backends: HashMap::new(),
        }
    }

    /// Registers a new runtime backend.
    pub fn register_backend(&mut self, backend: Arc<dyn RuntimeBackend>) {
        info!("Registering runtime backend: {} ({})", backend.display_name(), backend.id());
        self.backends.insert(backend.id().to_string(), backend);
    }

    /// Retrieves a backend by ID.
    pub fn get_backend(&self, id: &str) -> PlazaResult<Arc<dyn RuntimeBackend>> {
        self.backends
            .get(id)
            .cloned()
            .ok_or_else(|| PlazaError::RuntimeUnavailable(format!("Backend not found: {}", id)))
    }

    /// Negotiates capabilities to find the best backend for a workspace specification.
    ///
    /// In DP1, we simplify this to just returning the first available backend or
    /// matching by a requested backend ID.
    pub async fn negotiate_backend(&self, requested_id: Option<&str>) -> PlazaResult<Arc<dyn RuntimeBackend>> {
        if let Some(id) = requested_id {
            return self.get_backend(id);
        }

        // Return first available (naive DP1 negotiation)
        for backend in self.backends.values() {
            if backend.is_available().await {
                return Ok(backend.clone());
            }
        }

        Err(PlazaError::NoSuitableRuntime {
            reason: "No available backends found".into(),
        })
    }
}

impl Default for RuntimeManager {
    fn default() -> Self {
        Self::new()
    }
}
