use plaza_foundation::core::PlazaResult;
use std::path::PathBuf;

pub struct PluginDiscovery {
    _plugin_dir: PathBuf,
}

impl PluginDiscovery {
    pub fn new(plugin_dir: PathBuf) -> Self {
        Self { _plugin_dir: plugin_dir }
    }

    pub async fn scan_directory(&self) -> PlazaResult<Vec<crate::manifest::PluginManifest>> {
        Ok(vec![])
    }
}
