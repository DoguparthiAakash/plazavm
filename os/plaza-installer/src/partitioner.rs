use plaza_foundation::core::PlazaResult;

pub struct DiskPartitioner;

impl Default for DiskPartitioner {
    fn default() -> Self {
        Self::new()
    }
}

impl DiskPartitioner {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn partition_uefi(&self, _device: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
