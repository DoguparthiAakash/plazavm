use plaza_resource::vhal::{HardwareProfileKind, VirtualHardwareProfile};

/// Thin coordinator delegating hardware profile resolution to `plaza-resource`.
pub struct ResourceCoordinator;

impl ResourceCoordinator {
    pub fn new() -> Self {
        Self
    }

    pub fn resolve_profile(&self, kind: HardwareProfileKind) -> VirtualHardwareProfile {
        VirtualHardwareProfile::for_kind(kind)
    }
}

impl Default for ResourceCoordinator {
    fn default() -> Self {
        Self::new()
    }
}
