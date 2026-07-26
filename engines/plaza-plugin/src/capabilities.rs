use plaza_foundation::core::PlazaResult;

pub struct PluginCapabilitiesManager;

impl PluginCapabilitiesManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn negotiate(&self, _plugin_id: &str) -> PlazaResult<()> {
        Ok(())
    }
}
