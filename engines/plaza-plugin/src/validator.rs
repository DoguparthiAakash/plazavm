use plaza_foundation::core::PlazaResult;

pub struct PluginManifestValidator;

impl PluginManifestValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, _manifest: &crate::manifest::PluginManifest) -> PlazaResult<bool> {
        Ok(true)
    }
}
