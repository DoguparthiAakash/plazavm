use plaza_foundation::core::PlazaResult;

pub struct PortForwarder;

impl PortForwarder {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn forward_port(&self, _host_port: u16, _guest_port: u16) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
