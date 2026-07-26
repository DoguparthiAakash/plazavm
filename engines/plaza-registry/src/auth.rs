use plaza_foundation::core::PlazaResult;

pub struct RegistryAuthenticator;

impl RegistryAuthenticator {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn authenticate(&self, _registry_url: &str) -> PlazaResult<String> {
        Err(plaza_foundation::core::PlazaError::storage("Registry authentication not implemented for DP1"))
    }
}
