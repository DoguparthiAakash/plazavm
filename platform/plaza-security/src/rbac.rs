use plaza_foundation::core::PlazaResult;

pub struct RoleBasedAccessControl;

impl Default for RoleBasedAccessControl {
    fn default() -> Self {
        Self::new()
    }
}

impl RoleBasedAccessControl {
    pub fn new() -> Self {
        Self
    }

    pub async fn check_permission(&self, _role: &str, _action: &str) -> PlazaResult<bool> {
        Ok(true) // DP1 Stub
    }
}
