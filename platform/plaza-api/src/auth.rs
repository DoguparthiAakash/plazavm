use plaza_foundation::core::PlazaResult;

pub struct AuthMiddleware;

impl Default for AuthMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthMiddleware {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_token(&self, _token: &str) -> PlazaResult<bool> {
        Ok(true) // DP1 Stub
    }
}
