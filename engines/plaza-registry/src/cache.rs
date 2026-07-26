use plaza_foundation::core::PlazaResult;
use std::path::PathBuf;

pub struct RegistryCache {
    _cache_dir: PathBuf,
}

impl RegistryCache {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self { _cache_dir: cache_dir }
    }
    
    pub async fn clear(&self) -> PlazaResult<()> {
        Ok(())
    }
}
