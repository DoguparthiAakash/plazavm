//! Application configuration schema parser for `plaza.toml`.

use plaza_core::PlazaResult;
use serde::{Deserialize, Serialize};

/// Global system configuration for PlazaVM daemon and desktop client.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlazaConfig {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub defaults: DefaultPreferences,
    #[serde(default)]
    pub ai: AiConfigSection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_bind_addr")]
    pub bind_address: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_bind_addr() -> String {
    "127.0.0.1".into()
}
fn default_port() -> u16 {
    8080
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: default_bind_addr(),
            port: default_port(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DefaultPreferences {
    pub preferred_backend: Option<String>,
    pub auto_suspend_idle_mins: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AiConfigSection {
    pub enabled: bool,
    pub provider: Option<String>,
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
}

impl PlazaConfig {
    pub fn parse_toml(content: &str) -> PlazaResult<Self> {
        toml::from_str(content).map_err(|e| plaza_core::PlazaError::Config(e.to_string()))
    }
}
