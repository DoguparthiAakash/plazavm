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
    pub async fn negotiate_backend(&self, config: &crate::machine::MachineConfig, requested_id: Option<&str>) -> PlazaResult<Arc<dyn RuntimeBackend>> {
        // Find matching backend using the resolver
        let candidates = self.backends.values();
        let backend = crate::resolver::resolve_backend(candidates, config, requested_id)
            .ok_or_else(|| PlazaError::NoSuitableRuntime {
                reason: format!("No available backends support the given MachineConfig. Architecture: {}", config.machine.architecture),
            })?;
        
        if !backend.is_available().await {
            return Err(PlazaError::NoSuitableRuntime {
                reason: format!("Resolved backend {} is not available on the host system", backend.id()),
            });
        }
        
        Ok(backend)
    }

}

impl Default for RuntimeManager {
    fn default() -> Self {
        Self::new()
    }
}
