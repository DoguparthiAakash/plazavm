use plaza_foundation::core::PlazaResult;

pub struct ImageFetcher;

impl ImageFetcher {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn fetch(&self, _reference: &str) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::storage("Image fetching not implemented for DP1"))
    }
}
