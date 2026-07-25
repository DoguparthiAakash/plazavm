use plaza_config::PlazaConfig;

/// Thin coordinator delegating configuration operations to `plaza-config`.
pub struct ConfigurationCoordinator {
    config: PlazaConfig,
}

impl ConfigurationCoordinator {
    pub fn new() -> Self {
        Self {
            config: PlazaConfig::default(),
        }
    }

    pub fn config(&self) -> &PlazaConfig {
        &self.config
    }
}

impl Default for ConfigurationCoordinator {
    fn default() -> Self {
        Self::new()
    }
}
