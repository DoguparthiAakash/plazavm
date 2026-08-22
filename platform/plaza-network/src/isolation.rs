use plaza_foundation::core::PlazaResult;

pub struct NetworkIsolation;

impl Default for NetworkIsolation {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkIsolation {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn isolate(&self, _workspace_id: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
