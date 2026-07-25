use plaza_platform::kal::{KernelAdapter, LinuxKernelAdapter};

/// Thin coordinator delegating PAL/KAL security verification to `plaza-platform`.
pub struct SecurityCoordinator {
    adapter: LinuxKernelAdapter,
}

impl SecurityCoordinator {
    pub fn new() -> Self {
        Self {
            adapter: LinuxKernelAdapter,
        }
    }

    pub async fn check_capabilities(&self) -> plaza_core::PlazaResult<plaza_platform::kal::KernelCapabilities> {
        self.adapter.detect_capabilities().await
    }
}

impl Default for SecurityCoordinator {
    fn default() -> Self {
        Self::new()
    }
}
