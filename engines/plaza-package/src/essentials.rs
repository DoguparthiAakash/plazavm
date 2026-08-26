use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Represents a parsed `plazaessentials.toml` file.
/// This file declaratively defines all external tools and dependencies
/// that should be injected into the Workspace sandbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlazaEssentials {
    /// Global dependencies (e.g., node, python, git)
    #[serde(default)]
    pub dependencies: HashMap<String, String>,

    /// Linux-specific dependencies (e.g., apt packages)
    #[serde(default)]
    pub linux: HashMap<String, String>,

    /// Windows-specific dependencies (e.g., winget packages)
    #[serde(default)]
    pub windows: HashMap<String, String>,
}

impl PlazaEssentials {
    /// Parse a `plazaessentials.toml` file from the given path.
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read plazaessentials.toml: {}", e))?;
        toml::from_str(&content).map_err(|e| format!("Invalid plazaessentials.toml syntax: {}", e))
    }

    /// Generate a minimal template for `plazaessentials.toml`.
    pub fn generate_template() -> String {
        r#"# Plaza Essentials Configuration
# Define dependencies to be injected into the workspace.

[dependencies]
# node = "20.x"
# python = "3.11"
# git = "latest"

[linux]
# ubuntu_features = ["build-essential", "curl"]

[windows]
# win_features = ["visual-studio-build-tools"]
"#
        .to_string()
    }
}
