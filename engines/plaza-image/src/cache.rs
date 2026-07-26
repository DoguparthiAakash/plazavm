use plaza_foundation::core::PlazaResult;
use std::path::PathBuf;

pub struct ImageCache {
    _cache_dir: PathBuf,
}

impl ImageCache {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self { _cache_dir: cache_dir }
    }
    
    pub async fn prune(&self) -> PlazaResult<u64> {
        Ok(0)
    }
}
