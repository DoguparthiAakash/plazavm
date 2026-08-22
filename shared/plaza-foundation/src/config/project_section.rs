//! Project section of `plaza.yaml`.
//!
//! Represents the project constraints, language, runtime, and dependencies for the workspace.

use serde::{Deserialize, Serialize};

/// The `project:` section of `plaza.yaml`.
///
/// # Example
///
/// ```yaml
/// project:
///   language: python
///   runtime:
///     version: "3.12"
///   dependencies:
///     - numpy
///     - pandas
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSection {
    /// The primary language of the project (e.g., "python", "node").
    pub language: String,

    /// Project runtime requirements.
    #[serde(default)]
    pub runtime: Option<ProjectRuntime>,

    /// Project-specific dependencies required in the environment.
    #[serde(default)]
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRuntime {
    /// The required runtime version (e.g., "3.12" or "22").
    pub version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_project_section() {
        let yaml = r#"
language: python
runtime:
  version: "3.12"
dependencies:
  - numpy
  - pandas
"#;
        let section: ProjectSection = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(section.language, "python");
        assert_eq!(section.runtime.as_ref().unwrap().version, "3.12");
        assert_eq!(section.dependencies.len(), 2);
        assert_eq!(section.dependencies[0], "numpy");
    }
}
