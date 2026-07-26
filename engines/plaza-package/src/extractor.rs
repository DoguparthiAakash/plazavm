use plaza_foundation::core::PlazaResult;

pub struct PackageExtractor;

impl PackageExtractor {
    pub fn new() -> Self {
        Self
    }

    pub async fn extract(&self, _archive: &[u8]) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::storage("Package extraction not implemented for DP1"))
    }
}
