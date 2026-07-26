use plaza_foundation::core::PlazaResult;

pub struct ApiCodeGenerator;

impl ApiCodeGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate(&self, _spec: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
