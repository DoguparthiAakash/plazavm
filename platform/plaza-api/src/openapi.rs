use plaza_foundation::core::PlazaResult;

pub struct OpenApiSchema;

impl Default for OpenApiSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenApiSchema {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate(&self) -> PlazaResult<String> {
        Ok("{}".to_string()) // DP1 Stub
    }
}
