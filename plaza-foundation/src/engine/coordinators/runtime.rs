use plaza_runtime::RuntimeBackend;
use std::sync::Arc;

/// Thin coordinator delegating runtime driver orchestration to `plaza-runtime`.
pub struct RuntimeCoordinator {
    runtime_backend: Option<Arc<dyn RuntimeBackend>>,
}

impl RuntimeCoordinator {
    pub fn new(runtime_backend: Option<Arc<dyn RuntimeBackend>>) -> Self {
        Self { runtime_backend }
    }

    pub fn runtime_backend(&self) -> Option<&Arc<dyn RuntimeBackend>> {
        self.runtime_backend.as_ref()
    }
}
