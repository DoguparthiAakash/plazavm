use plaza_foundation::core::PlazaResult;

pub struct RegistryMirror;

impl RegistryMirror {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn configure(&self, _primary_url: &str, _mirror_url: &str) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::storage("Registry mirroring not implemented for DP1"))
    }
}
