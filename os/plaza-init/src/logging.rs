use plaza_foundation::core::PlazaResult;

pub struct StdoutLogger;

impl Default for StdoutLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl StdoutLogger {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn capture_logs(&self, _pid: u32) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
