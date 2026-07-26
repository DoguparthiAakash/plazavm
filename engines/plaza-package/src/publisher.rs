use plaza_foundation::core::PlazaResult;

pub struct PackagePublisher;

impl PackagePublisher {
    pub fn new() -> Self {
        Self
    }

    pub async fn publish(&self, _manifest: &crate::model::PackageManifest) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::storage("Package publishing not implemented for DP1"))
    }
}
