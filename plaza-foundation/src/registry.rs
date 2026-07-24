//! Provider Registry for Platform, Runtime, Package, Storage, and Hardware Providers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderCategory {
    Platform,
    Runtime,
    Hardware,
    Package,
    Storage,
    Security,
    Network,
    Ai,
    Plugin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDescriptor {
    pub id: String,
    pub name: String,
    pub category: ProviderCategory,
    pub version: semver::Version,
    pub capabilities: Vec<String>,
}

/// Universal Provider Registry.
pub struct ProviderRegistry {
    providers: Arc<Mutex<HashMap<String, ProviderDescriptor>>>,
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, descriptor: ProviderDescriptor) {
        let mut map = self.providers.lock().unwrap();
        map.insert(descriptor.id.clone(), descriptor);
    }

    pub fn get(&self, id: &str) -> Option<ProviderDescriptor> {
        let map = self.providers.lock().unwrap();
        map.get(id).cloned()
    }

    pub fn list(&self) -> Vec<ProviderDescriptor> {
        let map = self.providers.lock().unwrap();
        map.values().cloned().collect()
    }
}
