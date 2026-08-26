use plaza_foundation::core::PlazaResult;

pub struct IoBandwidthThrottler;

impl Default for IoBandwidthThrottler {
    fn default() -> Self {
        Self::new()
    }
}

impl IoBandwidthThrottler {
    pub fn new() -> Self {
        Self
    }

    pub async fn set_throttle(&self, _workspace_id: &str, _mb_per_sec: u32) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
