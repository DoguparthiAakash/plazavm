use plaza_foundation::core::PlazaResult;

pub struct ProcessReaper;

impl Default for ProcessReaper {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessReaper {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn waitpid_loop(&self) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
