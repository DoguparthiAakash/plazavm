use plaza_foundation::core::PlazaResult;

pub struct VsockIpc;

impl VsockIpc {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn send_packet(&self, _data: &[u8]) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
