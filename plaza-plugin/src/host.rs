//! Plugin host for loading and managing registered plugins.

use crate::manifest::PluginManifest;
use crate::{Plugin, RuntimePlugin};
use plaza_core::id::PluginId;
use plaza_core::PlazaResult;
use plaza_events::EventBus;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Plugin host managing registered plugins and runtime plugins.
pub struct PluginHost {
    #[allow(dead_code)]
    plugins: RwLock<HashMap<PluginId, Box<dyn Plugin>>>,
    runtime_plugins: RwLock<HashMap<String, Arc<dyn RuntimePlugin>>>,
    event_bus: Arc<EventBus>,
    plugin_dir: PathBuf,
}

impl PluginHost {
    /// Create a new plugin host.
    pub fn new(event_bus: Arc<EventBus>, plugin_dir: PathBuf) -> Self {
        Self {
            plugins: RwLock::new(HashMap::new()),
            runtime_plugins: RwLock::new(HashMap::new()),
            event_bus,
            plugin_dir,
        }
    }

    /// Register a runtime plugin programmatically.
    pub async fn register_runtime_plugin(&self, plugin: Arc<dyn RuntimePlugin>) -> PlazaResult<()> {
        let manifest = plugin.manifest();
        let plugin_id = PluginId::new(&manifest.id);
        let backend_id = plugin.id().to_string();

        let plugin_name = manifest.name.clone();

        info!(
            id = %plugin_id,
            backend = %backend_id,
            name = %plugin_name,
            version = %manifest.version,
            "registering runtime plugin"
        );

        self.runtime_plugins
            .write()
            .await
            .insert(backend_id.clone(), plugin);

        self.event_bus
            .publish(plaza_events::PlazaEvent::PluginLoaded {
                id: plugin_id,
                name: plugin_name,
            })
            .await;

        Ok(())
    }

    /// Retrieve a runtime plugin by its backend ID.
    pub async fn get_runtime_plugin(&self, backend_id: &str) -> Option<Arc<dyn RuntimePlugin>> {
        self.runtime_plugins.read().await.get(backend_id).cloned()
    }

    /// List all available runtime plugins.
    pub async fn available_runtime_plugins(&self) -> Vec<Arc<dyn RuntimePlugin>> {
        self.runtime_plugins
            .read()
            .await
            .values()
            .cloned()
            .collect()
    }

    /// Discover plugin manifests in the plugin directory.
    pub async fn discover_manifests(&self) -> PlazaResult<Vec<PluginManifest>> {
        let mut manifests = Vec::new();
        if !self.plugin_dir.exists() {
            return Ok(manifests);
        }

        let mut entries = tokio::fs::read_dir(&self.plugin_dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let manifest_path = entry.path().join("plugin.toml");
            if manifest_path.exists() {
                if let Ok(content) = tokio::fs::read_to_string(&manifest_path).await {
                    if let Ok(manifest) = toml::from_str::<PluginManifest>(&content) {
                        manifests.push(manifest);
                    }
                }
            }
        }

        Ok(manifests)
    }
}
