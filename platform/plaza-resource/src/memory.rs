use plaza_foundation::core::PlazaResult;

pub struct MemoryLimitEnforcer;

impl MemoryLimitEnforcer {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn set_limit(&self, _workspace_id: &str, _bytes: u64) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
