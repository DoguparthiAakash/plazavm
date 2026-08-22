use plaza_foundation::core::PlazaResult;

pub struct PluginHotReloader;

impl Default for PluginHotReloader {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginHotReloader {
    pub fn new() -> Self {
        Self
    }

    pub async fn reload(&self, _plugin_id: &str) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::storage("Hot-reloading not implemented for DP1"))
    }
}
