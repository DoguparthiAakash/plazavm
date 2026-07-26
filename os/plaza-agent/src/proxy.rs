use plaza_foundation::core::PlazaResult;

pub struct CommandProxy;

impl CommandProxy {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn exec(&self, _cmd: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
