//! Engine section of `plaza.yaml`.
//!
//! Represents the base OS engine constraints for the workspace.

use serde::{Deserialize, Serialize};

/// The `engine:` section of `plaza.yaml`.
///
/// # Example
///
/// ```yaml
/// engine:
///   distribution: alpine
///   version: stable
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineSection {
    /// The engine distribution (e.g., "alpine", "fedora", "arch").
    pub distribution: String,

    /// The engine version (e.g., "stable", "latest", "3.20").
    #[serde(default)]
    pub version: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_engine_section() {
        let yaml = r#"
distribution: alpine
version: stable
"#;
        let section: EngineSection = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(section.distribution, "alpine");
        assert_eq!(section.version.as_deref(), Some("stable"));
    }
}
