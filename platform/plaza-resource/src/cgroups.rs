use plaza_foundation::core::PlazaResult;

pub struct CgroupsV2Manager;

impl Default for CgroupsV2Manager {
    fn default() -> Self {
        Self::new()
    }
}

impl CgroupsV2Manager {
    pub fn new() -> Self {
        Self
    }

    pub async fn apply_limits(&self, _workspace_id: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
