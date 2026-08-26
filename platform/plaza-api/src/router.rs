use plaza_foundation::core::PlazaResult;

pub struct ApiRouter;

impl Default for ApiRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiRouter {
    pub fn new() -> Self {
        Self
    }

    pub async fn serve(&self, _port: u16) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
