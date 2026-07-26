use plaza_foundation::core::PlazaResult;

pub struct ImageValidator;

impl ImageValidator {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn validate_signature(&self, _manifest: &crate::model::ImageManifest) -> PlazaResult<bool> {
        Ok(true)
    }
}
