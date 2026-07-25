use crate::engine::errors::{PfeError, PfeResult};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

/// Type-Safe Dependency Injection Container.
pub struct ServiceContainer {
    services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl ServiceContainer {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register<T: Any + Send + Sync>(&mut self, service: Arc<T>) {
        self.services.insert(TypeId::of::<T>(), service);
    }

    pub fn resolve<T: Any + Send + Sync>(&self) -> PfeResult<Arc<T>> {
        let type_id = TypeId::of::<T>();
        if let Some(service) = self.services.get(&type_id) {
            if let Ok(downcasted) = service.clone().downcast::<T>() {
                return Ok(downcasted);
            }
        }
        Err(PfeError::ServiceNotFound(std::any::type_name::<T>().into()))
    }
}

impl Default for ServiceContainer {
    fn default() -> Self {
        Self::new()
    }
}
