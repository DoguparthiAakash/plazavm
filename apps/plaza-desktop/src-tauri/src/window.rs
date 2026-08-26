use plaza_foundation::core::PlazaResult;

pub struct WindowManager;

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowManager {
    pub fn new() -> Self {
        Self
    }

    pub fn create_window(&self, _label: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
