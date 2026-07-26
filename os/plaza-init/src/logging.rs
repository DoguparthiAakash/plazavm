use plaza_foundation::core::PlazaResult;

pub struct StdoutLogger;

impl StdoutLogger {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn capture_logs(&self, _pid: u32) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
