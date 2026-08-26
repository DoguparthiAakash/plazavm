use plaza_foundation::core::PlazaResult;

pub struct PackageManager;

impl Default for PackageManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn install(&self, _package: &str) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::storage(
            "Package installation not implemented for DP1",
        ))
    }
}
