use plaza_foundation::core::PlazaResult;

pub struct PluginSandbox;

impl Default for PluginSandbox {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginSandbox {
    pub fn new() -> Self {
        Self
    }

    pub fn prepare_wasmtime_env(&self) -> PlazaResult<()> {
        Ok(())
    }
}
