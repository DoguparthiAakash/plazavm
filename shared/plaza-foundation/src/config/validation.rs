//! Configuration validation pipeline for `plaza.yaml`.
//!
//! Pipeline:
//!
//! ```text
//! YAML string
//!      ↓
//! YAML Parser (serde_yaml)
//!      ↓
//! PlazaYaml AST
//!      ↓
//! Schema Validation (required fields, type checks)
//!      ↓
//! Semantic Validation (architecture, memory size, path safety)
//!      ↓
//! Capability Validation (default-deny enforcement)
//!      ↓
//! Validated PlazaYaml
//! ```

use crate::config::capabilities::FilesystemGrant;
use crate::config::workspace_config::PlazaYaml;
use crate::core::types::ByteSize;
use crate::core::{PlazaError, PlazaResult};
use std::path::Path;

/// Validate a parsed `PlazaYaml` configuration.
///
/// This runs schema validation, semantic validation, and capability
/// validation in sequence. Returns `Ok(())` on success or a descriptive
/// `PlazaError::Config` on failure.
pub fn validate_plaza_yaml(config: &PlazaYaml) -> PlazaResult<()> {
    validate_schema(config)?;
    validate_semantics(config)?;
    validate_capabilities(config)?;
    Ok(())
}

/// Schema validation — required fields and basic type constraints.
fn validate_schema(config: &PlazaYaml) -> PlazaResult<()> {
    // Workspace name is mandatory and must not be blank.
    if config.workspace.name.trim().is_empty() {
        return Err(PlazaError::Config(
            "workspace.name cannot be empty".into(),
        ));
    }

    // If an image is specified, its name must not be blank.
    if let Some(ref image) = config.image {
        if image.name.trim().is_empty() {
            return Err(PlazaError::Config("image.name cannot be empty".into()));
        }
    }

    // Runtime backend must be a recognized value.
    if let Some(ref runtime) = config.runtime {
        runtime
            .parsed_backend()
            .map_err(|e| PlazaError::Config(e))?;
    }

    Ok(())
}

/// Semantic validation — domain-level constraints.
fn validate_semantics(config: &PlazaYaml) -> PlazaResult<()> {
    if let Some(ref machine) = config.machine {
        // CPU cores must be at least 1.
        if machine.cpu.cores == 0 {
            return Err(PlazaError::Config(
                "machine.cpu.cores must be at least 1".into(),
            ));
        }

        // Memory must be parseable and at least 64MiB.
        let mem_size = ByteSize::parse(&machine.memory.size)
            .map_err(|e| PlazaError::Config(format!("machine.memory.size: {}", e)))?;
        if mem_size.as_mb() < 64 {
            return Err(PlazaError::Config(
                "machine.memory.size must be at least 64MiB".into(),
            ));
        }
    }

    Ok(())
}

/// Capability validation — enforce default-deny rules and path safety.
fn validate_capabilities(config: &PlazaYaml) -> PlazaResult<()> {
    let caps = config.capabilities.as_ref().cloned().unwrap_or_default();

    // Validate filesystem grants.
    if let Some(ref fs_grants) = caps.filesystem {
        for grant in fs_grants {
            validate_filesystem_grant(grant)?;
        }
    }

    Ok(())
}

/// Validate a single filesystem grant for path safety.
fn validate_filesystem_grant(grant: &FilesystemGrant) -> PlazaResult<()> {
    let path = &grant.path;

    // Reject empty paths.
    if path.trim().is_empty() {
        return Err(PlazaError::config(
            "capabilities.filesystem[].path cannot be empty",
        ));
    }

    // Reject path traversal components.
    let normalized = Path::new(path);
    for component in normalized.components() {
        if let std::path::Component::ParentDir = component {
            return Err(PlazaError::config(format!(
                "capabilities.filesystem[].path contains path traversal: '{}'",
                path
            )));
        }
    }

    // Reject absolute paths that escape the workspace boundary.
    if normalized.is_absolute() || path.starts_with('/') || path.starts_with('\\') {
        return Err(PlazaError::config(format!(
            "capabilities.filesystem[].path must be relative to workspace, got absolute path: '{}'",
            path
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::workspace_config::PlazaYaml;

    fn minimal_valid_yaml() -> &'static str {
        r#"
version: "1"
workspace:
  name: test-project
"#
    }

    #[test]
    fn minimal_config_is_valid() {
        let config = PlazaYaml::parse_yaml(minimal_valid_yaml()).unwrap();
        assert!(validate_plaza_yaml(&config).is_ok());
    }

    #[test]
    fn empty_workspace_name_rejected() {
        let yaml = r#"
version: "1"
workspace:
  name: ""
"#;
        let config = PlazaYaml::parse_yaml(yaml).unwrap();
        assert!(validate_plaza_yaml(&config).is_err());
    }

    #[test]
    fn zero_cpu_cores_rejected() {
        let yaml = r#"
version: "1"
workspace:
  name: test
machine:
  cpu:
    cores: 0
"#;
        let config = PlazaYaml::parse_yaml(yaml).unwrap();
        assert!(validate_plaza_yaml(&config).is_err());
    }

    #[test]
    fn tiny_memory_rejected() {
        let yaml = r#"
version: "1"
workspace:
  name: test
machine:
  memory:
    size: "32MiB"
"#;
        // ByteSize::parse supports "MiB" suffix as "Mi"
        // This may or may not parse depending on suffix support.
        // The validation should catch < 64MiB regardless.
        let config = PlazaYaml::parse_yaml(yaml);
        if let Ok(config) = config {
            let result = validate_plaza_yaml(&config);
            // Should be invalid (too small) or parse error — both are acceptable.
            assert!(result.is_err());
        }
    }

    #[test]
    fn path_traversal_rejected() {
        let yaml = r#"
version: "1"
workspace:
  name: test
capabilities:
  filesystem:
    - path: "../../etc/passwd"
      mode: read-only
"#;
        let config = PlazaYaml::parse_yaml(yaml).unwrap();
        let result = validate_plaza_yaml(&config);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("path traversal"));
    }

    #[test]
    fn absolute_path_rejected() {
        let yaml = r#"
version: "1"
workspace:
  name: test
capabilities:
  filesystem:
    - path: "/etc/passwd"
      mode: read-only
"#;
        let config = PlazaYaml::parse_yaml(yaml).unwrap();
        let result = validate_plaza_yaml(&config);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("absolute path"));
    }

    #[test]
    fn missing_capabilities_means_denied() {
        let yaml = r#"
version: "1"
workspace:
  name: test
"#;
        let config = PlazaYaml::parse_yaml(yaml).unwrap();
        assert!(validate_plaza_yaml(&config).is_ok());
        // Capabilities should default to deny-all.
        let caps = config
            .capabilities
            .as_ref()
            .cloned()
            .unwrap_or_default();
        assert!(!caps.has_filesystem());
        assert!(!caps.has_network());
        assert!(!caps.has_clipboard());
        assert!(!caps.has_any_device());
    }

    #[test]
    fn unknown_backend_rejected() {
        let yaml = r#"
version: "1"
workspace:
  name: test
runtime:
  backend: hyperv
"#;
        let config = PlazaYaml::parse_yaml(yaml).unwrap();
        let result = validate_plaza_yaml(&config);
        assert!(result.is_err());
    }

    #[test]
    fn valid_backend_accepted() {
        for backend in &["auto", "qemu", "v86", "qemu-tcg", "v86-wasm"] {
            let yaml = format!(
                r#"
version: "1"
workspace:
  name: test
runtime:
  backend: {}
"#,
                backend
            );
            let config = PlazaYaml::parse_yaml(&yaml).unwrap();
            assert!(
                validate_plaza_yaml(&config).is_ok(),
                "backend '{}' should be valid",
                backend
            );
        }
    }
}
