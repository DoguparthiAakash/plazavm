use plaza_foundation::core::PlazaResult;

pub struct PluginIpcBroker;

impl PluginIpcBroker {
    pub fn new() -> Self {
        Self
    }

    pub async fn send_message(&self, _plugin_id: &str, _message: &[u8]) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::storage("IPC not implemented for DP1"))
    }
}
