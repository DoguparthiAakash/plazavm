//! Inferno OS image acquisition source.
//!
//! Handles locating the minimal Inferno kernel for booting under QEMU.
//!
//! CRITICAL: This module does NOT create placeholder kernels.
//! If a real Inferno kernel is not available, an explicit error is returned.
//!
//! To obtain a real kernel:
//! - Linux/Mac: ./scripts/build-inferno-kernel.sh
//! - Windows: .\scripts\build-inferno-kernel.ps1
//! - Manual: Clone https://github.com/inferno-os/inferno-os and build the
//!   native kernel for 386/x86

use async_trait::async_trait;
use plaza_foundation::core::{PlazaError, PlazaResult};
use std::path::PathBuf;
use tracing::{debug, info};

use super::acquisition::ImageAcquisitionSource;

/// The Inferno OS source repository URL.
pub const INFERNO_SOURCE_URL: &str = "https://github.com/inferno-os/inferno-os.git";

/// Default Inferno version/tag to use.
const INFERNO_DEFAULT_VERSION: &str = "master";

/// Inferno kernel binary names (platform-specific).
const INFERNO_KERNEL_386: &str = "386";

/// Acquisition source for Inferno OS artifacts.
///
/// This source locates a pre-built Inferno kernel in the
/// PlazaVM cache directory or staging directory.
/// If not found, it returns an explicit error.
pub struct InfernoAcquisitionSource {
    cache_dir: PathBuf,
}

impl InfernoAcquisitionSource {
    /// Create a new Inferno acquisition source.
    ///
    /// Uses `$TEMP/plaza-inferno-cache` as the default cache directory.
    pub fn new() -> PlazaResult<Self> {
        let cache_dir = std::env::temp_dir().join("plaza-inferno-cache");
        std::fs::create_dir_all(&cache_dir).map_err(PlazaError::Io)?;
        Ok(Self { cache_dir })
    }

    /// Create with a custom cache directory.
    pub fn with_cache_dir(cache_dir: PathBuf) -> PlazaResult<Self> {
        std::fs::create_dir_all(&cache_dir).map_err(PlazaError::Io)?;
        Ok(Self { cache_dir })
    }

    /// Resolve the target architecture string for Inferno.
    fn target_arch() -> &'static str {
        match std::env::consts::ARCH {
            "x86_64" => INFERNO_KERNEL_386,
            "x86" => INFERNO_KERNEL_386,
            "aarch64" => "arm",
            "arm" => "arm",
            _ => INFERNO_KERNEL_386,
        }
    }

    /// Look for a pre-built Inferno kernel in the cache.
    fn find_cached_kernel(&self) -> Option<PathBuf> {
        let arch = Self::target_arch();
        let kernel_name = format!("inferno-{}", arch);
        let kernel_path = self.cache_dir.join(&kernel_name);
        if kernel_path.exists() {
            info!("Found cached Inferno kernel at {:?}", kernel_path);
            Some(kernel_path)
        } else {
            debug!("No cached Inferno kernel at {:?}", kernel_path);
            None
        }
    }

    /// Look for an Inferno kernel in the staging directory.
    fn find_staging_kernel(&self) -> Option<PathBuf> {
        let arch = Self::target_arch();
        let staging_dir = std::env::current_dir()
            .unwrap_or_default()
            .join("staging");
        let kernel_candidates = [
            staging_dir
                .join("inferno")
                .join(format!("inferno.{}", arch)),
            staging_dir
                .join("inferno-kernel")
                .join(format!("inferno.{}", arch)),
            staging_dir.join(format!("inferno-{}", arch)),
            staging_dir.join(format!("inferno.{}", arch)),
        ];

        for kernel_path in &kernel_candidates {
            if kernel_path.exists() {
                info!("Found Inferno kernel in staging at {:?}", kernel_path);
                return Some(kernel_path.clone());
            }
        }
        debug!("No Inferno kernel found in staging directory");
        None
    }

    /// Look for an Inferno kernel in a local source tree.
    fn find_source_kernel(&self) -> Option<PathBuf> {
        let home_inferno = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .ok()
            .map(|h| PathBuf::from(h).join("inferno"))
            .unwrap_or_default();

        let candidates = [
            PathBuf::from("/usr/inferno"),
            PathBuf::from("/opt/inferno"),
            home_inferno,
            PathBuf::from("C:\\inferno"),
            PathBuf::from("e:\\plazavm\\inferno-os"),
        ];

        let arch = Self::target_arch();

        for candidate in &candidates {
            let kernel_paths = [
                candidate.join(format!("os/pc/obj/inferno.{}", arch)),
                candidate.join(format!("os/pc/inferno.{}", arch)),
                candidate.join(format!("os/pc/pc/inferno.{}", arch)),
            ];

            for kernel_path in &kernel_paths {
                if kernel_path.exists() {
                    info!(
                        "Found Inferno kernel in source tree at {:?}",
                        kernel_path
                    );
                    return Some(kernel_path.clone());
                }
            }
        }
        debug!("No Inferno kernel found in source tree");
        None
    }

    /// Find the Inferno source tree.
    fn find_source_tree(&self) -> Option<PathBuf> {
        let home_inferno = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .ok()
            .map(|h| PathBuf::from(h).join("inferno"))
            .unwrap_or_default();

        let candidates = [
            PathBuf::from("/usr/inferno"),
            PathBuf::from("/opt/inferno"),
            home_inferno,
            PathBuf::from("C:\\inferno"),
            PathBuf::from("e:\\plazavm\\inferno-os"),
        ];

        for candidate in &candidates {
            if candidate.join("mkfile").exists() || candidate.join("makemk.sh").exists() {
                return Some(candidate.clone());
            }
        }
        None
    }

    /// Try to build a minimal Inferno kernel from source.
    ///
    /// This is a best-effort operation that requires the Inferno toolchain.
    /// Returns None if building is not possible.
    async fn try_build_kernel(&self) -> Option<PathBuf> {
        let inferno_root = self.find_source_tree()?;

        info!(
            "Attempting to build Inferno kernel from source at {:?}",
            inferno_root
        );

        let objtype = Self::target_arch();
        let build_dir = inferno_root.join(format!("os/pc/obj/{}", objtype));

        let kernel_name = format!("inferno.{}", objtype);
        let built_kernel = build_dir.join(&kernel_name);
        if built_kernel.exists() {
            info!("Found pre-built Inferno kernel at {:?}", built_kernel);
            return Some(built_kernel);
        }

        let status = std::process::Command::new("mk")
            .arg("install")
            .current_dir(&inferno_root)
            .env("OBJTYPE", objtype)
            .status();

        match status {
            Ok(s) if s.success() => {
                if built_kernel.exists() {
                    info!("Successfully built Inferno kernel at {:?}", built_kernel);
                    Some(built_kernel)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Default for InfernoAcquisitionSource {
    fn default() -> Self {
        Self::new().expect("Failed to create InfernoAcquisitionSource")
    }
}

#[async_trait]
impl ImageAcquisitionSource for InfernoAcquisitionSource {
    async fn fetch_base_image(&self, _base_image: &str) -> PlazaResult<PathBuf> {
        // For Inferno, the "base image" is the kernel itself.
        // Search all known locations for a real kernel.

        // 1. Check cache
        if let Some(kernel) = self.find_cached_kernel() {
            return Ok(kernel);
        }

        // 2. Check staging directory
        if let Some(kernel) = self.find_staging_kernel() {
            let arch = Self::target_arch();
            let cache_path = self.cache_dir.join(format!("inferno-{}", arch));
            tokio::fs::copy(&kernel, &cache_path)
                .await
                .map_err(PlazaError::Io)?;
            return Ok(cache_path);
        }

        // 3. Check source tree
        if let Some(kernel) = self.find_source_kernel() {
            let arch = Self::target_arch();
            let cache_path = self.cache_dir.join(format!("inferno-{}", arch));
            tokio::fs::copy(&kernel, &cache_path)
                .await
                .map_err(PlazaError::Io)?;
            return Ok(cache_path);
        }

        // 4. Try building from source
        if let Some(kernel) = self.try_build_kernel().await {
            let arch = Self::target_arch();
            let cache_path = self.cache_dir.join(format!("inferno-{}", arch));
            tokio::fs::copy(&kernel, &cache_path)
                .await
                .map_err(PlazaError::Io)?;
            return Ok(cache_path);
        }

        // 5. NO REAL KERNEL FOUND — return explicit error
        Err(PlazaError::InfernoKernelUnavailable {
            reason: format!(
                "No real Inferno kernel found. Searched:\n  \
                 - Cache: {:?}\n  \
                 - Staging: staging/inferno/\n  \
                 - Source: /usr/inferno, /opt/inferno, ~/inferno\n\n\
                 To build a real Inferno kernel:\n  \
                 Linux/Mac: ./scripts/build-inferno-kernel.sh\n  \
                 Windows:   .\\scripts\\build-inferno-kernel.ps1\n\n\
                 Or place the kernel manually at: {:?}/inferno-386",
                self.cache_dir,
                self.cache_dir
            ),
        })
    }
}

impl InfernoAcquisitionSource {
    /// Fetch the Inferno kernel and return its path.
    ///
    /// Unlike Alpine's fetch_kernel_and_initrd, Inferno uses a single
    /// kernel binary that contains both kernel and init (no separate initrd).
    pub async fn fetch_kernel(&self) -> PlazaResult<PathBuf> {
        self.fetch_base_image("inferno:latest").await
    }

    /// Get the Inferno source repository URL.
    pub fn source_url() -> &'static str {
        INFERNO_SOURCE_URL
    }

    /// Get the default Inferno version.
    pub fn default_version() -> &'static str {
        INFERNO_DEFAULT_VERSION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inferno_acquisition_source_creation() {
        let source = InfernoAcquisitionSource::new();
        assert!(source.is_ok());
    }

    #[test]
    fn inferno_target_arch() {
        let arch = InfernoAcquisitionSource::target_arch();
        assert!(!arch.is_empty());
    }

    #[test]
    fn inferno_source_url() {
        assert!(InfernoAcquisitionSource::source_url().contains("github.com"));
    }

    #[test]
    fn inferno_kernel_unavailable_error() {
        // Verify the error type exists and formats correctly
        let err = PlazaError::InfernoKernelUnavailable {
            reason: "test reason".to_string(),
        };
        assert!(err.to_string().contains("Inferno kernel unavailable"));
        assert!(err.to_string().contains("test reason"));
    }

    #[test]
    fn inferno_rootfs_unavailable_error() {
        let err = PlazaError::InfernoRootFilesystemUnavailable {
            reason: "test reason".to_string(),
        };
        assert!(err.to_string().contains("Inferno root filesystem unavailable"));
    }

    #[test]
    fn inferno_runtime_unavailable_error() {
        let err = PlazaError::InfernoRuntimeUnavailable {
            reason: "test reason".to_string(),
        };
        assert!(err.to_string().contains("Inferno runtime unavailable"));
    }
}
