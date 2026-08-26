use plaza_foundation::core::PlazaResult;

pub struct ServiceDiscovery;

impl Default for ServiceDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceDiscovery {
    pub fn new() -> Self {
        Self
    }

    pub async fn register_service(&self, _service_name: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
