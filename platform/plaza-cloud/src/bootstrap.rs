use plaza_foundation::core::PlazaResult;

pub struct CloudInitBootstrapper;

impl Default for CloudInitBootstrapper {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudInitBootstrapper {
    pub fn new() -> Self {
        Self
    }

    pub async fn run_cloud_init(&self) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
