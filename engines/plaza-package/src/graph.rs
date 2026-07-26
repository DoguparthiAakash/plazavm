use plaza_foundation::core::PlazaResult;

pub struct DependencyGraph;

impl DependencyGraph {
    pub fn new() -> Self {
        Self
    }

    pub async fn build(&self, _manifest: &crate::model::PackageManifest) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::storage("Dependency graph building not implemented for DP1"))
    }
}
