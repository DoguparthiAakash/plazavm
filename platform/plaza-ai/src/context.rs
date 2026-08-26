use plaza_foundation::core::PlazaResult;

pub struct ContextAwarenessManager;

impl Default for ContextAwarenessManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextAwarenessManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn gather_context(&self, _workspace_id: &str) -> PlazaResult<String> {
        Ok("".to_string()) // DP1 Stub
    }
}
