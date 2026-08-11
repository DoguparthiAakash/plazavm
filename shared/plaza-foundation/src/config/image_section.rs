//! Image section of `plaza.yaml`.
//!
//! Images are independent from machine configuration.
//! `plaza.yaml` references an image; it does not embed image contents.

use serde::{Deserialize, Serialize};

/// The `image:` section of `plaza.yaml`.
///
/// # Example
///
/// ```yaml
/// image:
///   name: alpine-dev
///   version: "1.0"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSection {
    /// Image name (e.g., `"alpine-dev"`).
    pub name: String,

    /// Optional image version/tag (e.g., `"1.0"`, `"latest"`).
    #[serde(default)]
    pub version: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_minimal() {
        let yaml = r#"
name: alpine-dev
"#;
        let section: ImageSection = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(section.name, "alpine-dev");
        assert!(section.version.is_none());
    }

    #[test]
    fn deserialize_with_version() {
        let yaml = r#"
name: alpine-dev
version: "1.0"
"#;
        let section: ImageSection = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(section.name, "alpine-dev");
        assert_eq!(section.version.as_deref(), Some("1.0"));
    }
}
