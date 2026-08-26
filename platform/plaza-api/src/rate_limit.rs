use plaza_foundation::core::PlazaResult;

pub struct RateLimiter;

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimiter {
    pub fn new() -> Self {
        Self
    }

    pub fn check_limit(&self, _client_ip: &str) -> PlazaResult<bool> {
        Ok(true) // DP1 Stub
    }
}
