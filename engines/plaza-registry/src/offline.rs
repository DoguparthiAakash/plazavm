use plaza_foundation::core::PlazaResult;

pub struct OfflineSyncManager;

impl OfflineSyncManager {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn sync(&self) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::storage("Offline sync not implemented for DP1"))
    }
}
