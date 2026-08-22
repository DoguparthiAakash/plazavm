use plaza_foundation::core::PlazaResult;

pub struct PluginManifestValidator;

impl Default for PluginManifestValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginManifestValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, _manifest: &crate::manifest::PluginManifest) -> PlazaResult<bool> {
        Ok(true)
    }
}
