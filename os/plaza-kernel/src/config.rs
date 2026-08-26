use plaza_foundation::core::PlazaResult;

pub struct KernelConfigManager;

impl Default for KernelConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

impl KernelConfigManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn apply_defconfig(&self, _path: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
