//! Standard filesystem paths for PlazaVM data, configuration, and plugins.

use std::path::PathBuf;

/// Application name used in directory naming.
pub const APP_NAME: &str = "plazavm";

/// Application display name.
pub const APP_DISPLAY_NAME: &str = "PlazaVM";

/// Returns the base data directory: `~/.plazavm/`
pub fn data_dir() -> PathBuf {
    dirs_base().join(".plazavm")
}

/// Returns the configuration directory: `~/.plazavm/config/`
pub fn config_dir() -> PathBuf {
    data_dir().join("config")
}

/// Returns the plugin directory: `~/.plazavm/plugins/`
pub fn plugin_dir() -> PathBuf {
    data_dir().join("plugins")
}

/// Returns the database directory: `~/.plazavm/data/`
pub fn db_dir() -> PathBuf {
    data_dir().join("data")
}

/// Returns the workspace storage directory: `~/.plazavm/workspaces/`
pub fn workspaces_dir() -> PathBuf {
    data_dir().join("workspaces")
}

/// Returns the log directory: `~/.plazavm/logs/`
pub fn log_dir() -> PathBuf {
    data_dir().join("logs")
}

/// Returns the cache directory: `~/.plazavm/cache/`
pub fn cache_dir() -> PathBuf {
    data_dir().join("cache")
}

/// Returns the registry directory: `~/.plazavm/registry/`
pub fn registry_dir() -> PathBuf {
    data_dir().join("registry")
}

/// Returns the path to the main SQLite database.
pub fn database_path() -> PathBuf {
    db_dir().join("plazavm.db")
}

/// Returns the path to the application configuration file.
pub fn app_config_path() -> PathBuf {
    config_dir().join("plaza.toml")
}

/// Ensure all required directories exist.
pub fn ensure_directories() -> std::io::Result<()> {
    for dir in [
        data_dir(),
        config_dir(),
        plugin_dir(),
        db_dir(),
        workspaces_dir(),
        log_dir(),
        cache_dir(),
        registry_dir(),
    ] {
        std::fs::create_dir_all(&dir)?;
    }
    Ok(())
}

fn dirs_base() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_are_under_data_dir() {
        let base = data_dir();
        assert!(config_dir().starts_with(&base));
        assert!(plugin_dir().starts_with(&base));
        assert!(db_dir().starts_with(&base));
        assert!(database_path().starts_with(&base));
    }
}
