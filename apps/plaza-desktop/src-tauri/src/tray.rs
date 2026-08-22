use plaza_foundation::core::PlazaResult;

pub struct SystemTrayManager;

impl Default for SystemTrayManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemTrayManager {
    pub fn new() -> Self {
        Self
    }
    
    pub fn build_tray(&self) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
