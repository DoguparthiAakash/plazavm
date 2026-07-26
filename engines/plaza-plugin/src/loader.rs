use plaza_foundation::core::PlazaResult;

pub struct PluginLoader;

impl PluginLoader {
    pub fn new() -> Self {
        Self
    }

    pub async fn load_wasm(&self, _path: &std::path::Path) -> PlazaResult<Box<dyn crate::Plugin>> {
        Err(plaza_foundation::core::PlazaError::storage("WASM loading not implemented for DP1"))
    }
    
    pub async fn load_native(&self, _path: &std::path::Path) -> PlazaResult<Box<dyn crate::Plugin>> {
        Err(plaza_foundation::core::PlazaError::storage("Native plugin loading not implemented for DP1"))
    }
}
