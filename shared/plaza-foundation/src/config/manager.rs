//! Configuration Manager providing import, export, default reset, and version migration.

use crate::config::app_config::PlazaConfig;
use crate::core::paths;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ConfigManager;

impl ConfigManager {
    pub fn config_file() -> PathBuf {
        let dir = paths::config_dir();
        fs::create_dir_all(&dir).ok();
        dir.join("plaza.toml")
    }

    pub fn load_active() -> anyhow::Result<PlazaConfig> {
        let path = Self::config_file();
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            Ok(PlazaConfig::parse_toml(&content)?)
        } else {
            let default_cfg = PlazaConfig::default();
            Self::save_active(&default_cfg)?;
            Ok(default_cfg)
        }
    }

    pub fn save_active(config: &PlazaConfig) -> anyhow::Result<()> {
        let path = Self::config_file();
        let toml_str = toml::to_string_pretty(config)?;
        fs::write(path, toml_str)?;
        Ok(())
    }

    pub fn export_config(target_path: &Path) -> anyhow::Result<()> {
        let active = Self::load_active()?;
        let toml_str = toml::to_string_pretty(&active)?;
        fs::write(target_path, toml_str)?;
        Ok(())
    }

    pub fn import_config(source_path: &Path) -> anyhow::Result<PlazaConfig> {
        let content = fs::read_to_string(source_path)?;
        let parsed = PlazaConfig::parse_toml(&content)?;
        Self::save_active(&parsed)?;
        Ok(parsed)
    }

    pub fn reset_to_defaults() -> anyhow::Result<PlazaConfig> {
        let default_cfg = PlazaConfig::default();
        Self::save_active(&default_cfg)?;
        Ok(default_cfg)
    }
}



