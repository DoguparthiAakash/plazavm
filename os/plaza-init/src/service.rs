use plaza_foundation::core::PlazaResult;

pub struct ServiceManager;

impl Default for ServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn start_service(&self, _name: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
