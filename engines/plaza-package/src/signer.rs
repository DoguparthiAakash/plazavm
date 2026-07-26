use plaza_foundation::core::PlazaResult;

pub struct PackageSigner;

impl PackageSigner {
    pub fn new() -> Self {
        Self
    }

    pub async fn sign(&self, _data: &[u8]) -> PlazaResult<Vec<u8>> {
        Err(plaza_foundation::core::PlazaError::storage("Package signing not implemented for DP1"))
    }
}
