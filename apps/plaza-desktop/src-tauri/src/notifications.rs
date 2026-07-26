use plaza_foundation::core::PlazaResult;

pub struct DesktopNotifications;

impl DesktopNotifications {
    pub fn new() -> Self {
        Self
    }
    
    pub fn notify(&self, _title: &str, _body: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
