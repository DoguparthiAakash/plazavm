use plaza_foundation::core::PlazaResult;

pub struct PackageCache;

impl Default for PackageCache {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageCache {
    pub fn new() -> Self {
        Self
    }

    pub async fn clear(&self) -> PlazaResult<()> {
        Ok(())
    }
}
