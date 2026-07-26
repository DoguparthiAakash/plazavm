use plaza_foundation::core::PlazaResult;

pub struct PackageResolver;

impl PackageResolver {
    pub fn new() -> Self {
        Self
    }

    pub async fn resolve(&self, _name: &str) -> PlazaResult<crate::model::PackageManifest> {
        Err(plaza_foundation::core::PlazaError::storage("Package resolution not implemented for DP1"))
    }
}
