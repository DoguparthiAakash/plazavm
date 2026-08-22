use plaza_foundation::core::PlazaResult;

pub struct CloudVolumeManager;

impl Default for CloudVolumeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudVolumeManager {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn attach_ebs(&self, _volume_id: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
