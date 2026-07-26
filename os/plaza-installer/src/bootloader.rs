use plaza_foundation::core::PlazaResult;

pub struct BootloaderConfig;

impl BootloaderConfig {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn configure_grub(&self) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
