use plaza_foundation::core::PlazaResult;

pub struct DnsResolver;

impl Default for DnsResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl DnsResolver {
    pub fn new() -> Self {
        Self
    }

    pub async fn configure_dns(&self, _workspace_id: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
