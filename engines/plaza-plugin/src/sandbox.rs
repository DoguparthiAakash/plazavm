use plaza_foundation::core::PlazaResult;

pub struct PluginSandbox;

impl PluginSandbox {
    pub fn new() -> Self {
        Self
    }

    pub fn prepare_wasmtime_env(&self) -> PlazaResult<()> {
        Ok(())
    }
}
