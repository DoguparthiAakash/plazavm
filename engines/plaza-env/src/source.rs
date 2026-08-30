//! Source tree resolution for environment builds.
//!
//! Resolves vendored OS source trees for compilation inside Inferno workspaces.
//! The user can override the default path via `plaza.yaml` (`env.source_path`).

use plaza_foundation::core::{PlazaError, PlazaResult};
use plaza_runtime::runtime::GuestRuntimeKind;
use std::path::{Path, PathBuf};

/// Represents a validated OS source tree ready for injection into a build workspace.
#[derive(Debug, Clone)]
pub struct SourceTree {
    /// The runtime this source tree builds.
    pub target: GuestRuntimeKind,
    /// Absolute path to the root of the source tree on the host.
    pub path: PathBuf,
    /// Whether the path was user-provided (overrides default vendored path).
    pub is_user_provided: bool,
}

impl SourceTree {
    /// Resolve the source tree for a given runtime, using an optional user override.
    ///
    /// Resolution order:
    /// 1. `user_override` if provided and exists
    /// 2. Default vendored path from `GuestRuntimeKind::default_source_dir()`, relative to `workspace_root`
    ///
    /// Returns an error if neither path exists or if the runtime has no vendored source
    /// (e.g. Inferno uses its own native path and does not go through this builder).
    pub fn resolve(
        target: &GuestRuntimeKind,
        workspace_root: &Path,
        user_override: Option<&Path>,
    ) -> PlazaResult<Self> {
        // Inferno is built natively — no external source tree needed.
        if *target == GuestRuntimeKind::Inferno {
            return Err(PlazaError::config(
                "The Inferno runtime does not use an external source tree. \
                 It is built from `inferno-os/` natively.",
            ));
        }

        // User-provided path takes priority.
        if let Some(override_path) = user_override {
            let abs = if override_path.is_absolute() {
                override_path.to_path_buf()
            } else {
                workspace_root.join(override_path)
            };
            if abs.exists() {
                tracing::info!(
                    target = %target,
                    path = %abs.display(),
                    "Using user-provided source tree override"
                );
                return Ok(SourceTree {
                    target: target.clone(),
                    path: abs,
                    is_user_provided: true,
                });
            } else {
                tracing::warn!(
                    path = %abs.display(),
                    "User-provided source_path does not exist, falling back to vendored default"
                );
            }
        }

        // Default vendored path.
        let default_rel = target.default_source_dir().ok_or_else(|| {
            PlazaError::config(format!(
                "Runtime '{}' has no vendored source tree in this project.",
                target
            ))
        })?;

        let default_abs = workspace_root.join(&default_rel);

        if default_abs.exists() {
            tracing::info!(
                target = %target,
                path = %default_abs.display(),
                "Using vendored source tree"
            );
            Ok(SourceTree {
                target: target.clone(),
                path: default_abs,
                is_user_provided: false,
            })
        } else {
            Err(PlazaError::config(format!(
                "Vendored source tree for '{}' not found at '{}'. \
                 Make sure the submodule is initialized (`git submodule update --init`) \
                 or provide a custom path via `env.source_path` in plaza.yaml.",
                target,
                default_abs.display()
            )))
        }
    }

    /// Returns the approximate size of the source tree in bytes by walking it.
    /// Used for progress reporting during workspace injection.
    pub fn approximate_size_bytes(&self) -> u64 {
        walkdir::WalkDir::new(&self.path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter_map(|e| e.metadata().ok())
            .map(|m| m.len())
            .sum()
    }

    /// Describes the source tree location for human-readable output.
    pub fn description(&self) -> String {
        if self.is_user_provided {
            format!(
                "{} (custom path: {})",
                self.target.display_name(),
                self.path.display()
            )
        } else {
            format!(
                "{} (vendored: {})",
                self.target.display_name(),
                self.path.display()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_fake_source(tmp: &TempDir, rel: &str) -> PathBuf {
        let path = tmp.path().join(rel);
        std::fs::create_dir_all(&path).unwrap();
        // Create a dummy file so the directory is not empty.
        std::fs::write(path.join("Makefile"), "all:\n\techo build\n").unwrap();
        path
    }

    #[test]
    fn inferno_returns_error() {
        let tmp = TempDir::new().unwrap();
        let result = SourceTree::resolve(&GuestRuntimeKind::Inferno, tmp.path(), None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Inferno"));
    }

    #[test]
    fn linux_resolves_from_vendored_default() {
        let tmp = TempDir::new().unwrap();
        setup_fake_source(&tmp, "vendors/linux");
        let tree = SourceTree::resolve(&GuestRuntimeKind::Linux, tmp.path(), None).unwrap();
        assert_eq!(tree.target, GuestRuntimeKind::Linux);
        assert!(!tree.is_user_provided);
        assert!(tree.path.exists());
    }

    #[test]
    fn freebsd_resolves_from_vendored_default() {
        let tmp = TempDir::new().unwrap();
        setup_fake_source(&tmp, "inferno-os/FreeBSD");
        let tree = SourceTree::resolve(&GuestRuntimeKind::FreeBsd, tmp.path(), None).unwrap();
        assert_eq!(tree.target, GuestRuntimeKind::FreeBsd);
        assert!(!tree.is_user_provided);
    }

    #[test]
    fn user_override_takes_priority() {
        let tmp = TempDir::new().unwrap();
        // Do NOT create the default vendored path.
        let custom = setup_fake_source(&tmp, "my-custom-linux-src");
        let tree = SourceTree::resolve(
            &GuestRuntimeKind::Linux,
            tmp.path(),
            Some(&custom),
        ).unwrap();
        assert!(tree.is_user_provided);
        assert_eq!(tree.path, custom);
    }

    #[test]
    fn missing_source_returns_error() {
        let tmp = TempDir::new().unwrap();
        // Nothing created — neither vendored nor custom path.
        let result = SourceTree::resolve(&GuestRuntimeKind::FreeBsd, tmp.path(), None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("submodule"));
    }
}
