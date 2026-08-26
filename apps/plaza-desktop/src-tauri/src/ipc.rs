use plaza_foundation::core::PlazaResult;

pub struct IpcBridge;

impl Default for IpcBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl IpcBridge {
    pub fn new() -> Self {
        Self
    }

    pub fn send_message(&self, _msg: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
