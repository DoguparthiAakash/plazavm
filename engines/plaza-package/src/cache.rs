use plaza_foundation::core::PlazaResult;

pub struct PackageCache;

impl PackageCache {
    pub fn new() -> Self {
        Self
    }

    pub async fn clear(&self) -> PlazaResult<()> {
        Ok(())
    }
}
