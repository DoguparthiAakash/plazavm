use plaza_foundation::core::PlazaResult;

pub struct ManifestParser;

impl ManifestParser {
    pub fn new() -> Self {
        Self
    }
    
    pub fn parse(&self, _data: &[u8]) -> PlazaResult<crate::model::ImageManifest> {
        Err(plaza_foundation::core::PlazaError::storage("Manifest parsing not implemented for DP1"))
    }
}
