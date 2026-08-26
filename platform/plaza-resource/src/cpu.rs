use plaza_foundation::core::PlazaResult;

pub struct CpuQuotaManager;

impl Default for CpuQuotaManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuQuotaManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn enforce_quota(&self, _workspace_id: &str, _quota: u32) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
