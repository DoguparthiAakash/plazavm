use plaza_foundation::core::PlazaResult;

pub struct PluginIpcBroker;

impl Default for PluginIpcBroker {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginIpcBroker {
    pub fn new() -> Self {
        Self
    }

    pub async fn send_message(&self, _plugin_id: &str, _message: &[u8]) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::storage("IPC not implemented for DP1"))
    }
}
