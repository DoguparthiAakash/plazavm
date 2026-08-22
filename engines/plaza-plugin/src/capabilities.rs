use plaza_foundation::core::PlazaResult;

pub struct PluginCapabilitiesManager;

impl Default for PluginCapabilitiesManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginCapabilitiesManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn negotiate(&self, _plugin_id: &str) -> PlazaResult<()> {
        Ok(())
    }
}
