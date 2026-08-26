use plaza_foundation::core::PlazaResult;

pub struct AuditLogger;

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditLogger {
    pub fn new() -> Self {
        Self
    }

    pub async fn log_event(&self, _event: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
