use plaza_foundation::core::PlazaResult;

pub struct ApiCodeGenerator;

impl Default for ApiCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiCodeGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate(&self, _spec: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
