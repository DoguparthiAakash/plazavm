/// Thin coordinator delegating networking operations to PAL/Platform layer.
pub struct NetworkingCoordinator;

impl NetworkingCoordinator {
    pub fn new() -> Self {
        Self
    }

    pub fn verify_bridge(&self) -> bool {
        true
    }
}

impl Default for NetworkingCoordinator {
    fn default() -> Self {
        Self::new()
    }
}
