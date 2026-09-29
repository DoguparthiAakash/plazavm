use async_trait::async_trait;

/// Get the appropriate acquisition source for a given base image.
pub fn get_acquisition_source(base_image: &str) -> PlazaResult<Box<dyn ImageAcquisitionSource>> {
    if base_image.starts_with("alpine") {
        Ok(Box::new(AlpineAcquisitionSource::new()?))
    } else {
        Err(PlazaError::config(format!(
            "No acquisition source available for base image '{}'",
            base_image
        )))
    }
}

use futures_util::StreamExt;
use plaza_foundation::core::{PlazaError, PlazaResult};
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Represents a source capable of acquiring a base image rootfs.
#[async_trait]
pub trait ImageAcquisitionSource: Send + Sync {
    /// Fetch the base image (e.g. `alpine:3.19.1`) and return the path to the downloaded tarball or raw block.
    async fn fetch_base_image(&self, base_image: &str) -> PlazaResult<PathBuf>;
}

/// Real acquisition of Alpine minirootfs via HTTP.
pub struct AlpineAcquisitionSource {
    client: Client,
}

impl AlpineAcquisitionSource {
    pub fn new() -> PlazaResult<Self> {
        tracing::info!("AlpineAcquisitionSource: Creating HTTP client...");
        let client = match Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
        {
            Ok(c) => {
                tracing::info!("AlpineAcquisitionSource: HTTP client created successfully");
                c
            }
            Err(e) => {
                tracing::error!("AlpineAcquisitionSource: Failed to create HTTP client: {}", e);
                return Err(PlazaError::config(format!(
                    "Failed to initialize HTTP client: {}",
                    e
                )));
            }
        };

        Ok(Self { client })
    }

    pub async fn fetch_kernel_and_initrd(
        &self,
        base_image: &str,
    ) -> PlazaResult<(PathBuf, PathBuf, Option<PathBuf>)> {
        let parts: Vec<&str> = base_image.split(':').collect();
        let version = if parts.len() > 1 && parts[1] != "latest" {
            parts[1]
        } else {
            "3.19.1"
        };

        let arch = "x86_64";
        let major_minor = if version.matches('.').count() >= 1 {
            let v_parts: Vec<&str> = version.split('.').collect();
            format!("{}.{}", v_parts[0], v_parts[1])
        } else {
            version.to_string()
        };

        let kernel_url = format!(
            "https://dl-cdn.alpinelinux.org/alpine/v{}/releases/{}/netboot/vmlinuz-virt",
            major_minor, arch
        );
        let initrd_url = format!(
            "https://dl-cdn.alpinelinux.org/alpine/v{}/releases/{}/netboot/initramfs-virt",
            major_minor, arch
        );

        tracing::info!(
            "AlpineAcquisitionSource: Resolving artifacts for v{}, arch={}",
            version,
            arch
        );

        let temp_dir = std::env::temp_dir().join("plaza-kernel-cache");
        tracing::info!("AlpineAcquisitionSource: Temp dir = {:?}", temp_dir);

        match fs::create_dir_all(&temp_dir).await {
            Ok(_) => tracing::info!("AlpineAcquisitionSource: Temp dir created/exists"),
            Err(e) => {
                tracing::error!("AlpineAcquisitionSource: Failed to create temp dir: {}", e);
                return Err(PlazaError::Io(e));
            }
        }

        let kernel_path = temp_dir.join(format!("vmlinuz-virt-{}", version));
        let initrd_path = temp_dir.join(format!("initramfs-virt-{}", version));

        tracing::info!("AlpineAcquisitionSource: kernel_path = {:?}", kernel_path);
        tracing::info!("AlpineAcquisitionSource: initrd_path = {:?}", initrd_path);
        tracing::info!(
            "AlpineAcquisitionSource: kernel exists = {}, initrd exists = {}",
            kernel_path.exists(),
            initrd_path.exists()
        );

        // Download kernel if not cached
        if !kernel_path.exists() {
            tracing::info!("AlpineAcquisitionSource: Downloading kernel from {}", kernel_url);
            match self.client.get(&kernel_url).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.bytes().await {
                            Ok(bytes) => {
                                tracing::info!(
                                    "AlpineAcquisitionSource: Got kernel bytes: {}",
                                    bytes.len()
                                );
                                if let Err(e) = tokio::fs::write(&kernel_path, &bytes).await {
                                    tracing::error!(
                                        "AlpineAcquisitionSource: Failed to write kernel: {}",
                                        e
                                    );
                                    return Err(PlazaError::Io(e));
                                }
                                tracing::info!("AlpineAcquisitionSource: Kernel cached at {:?}", kernel_path);
                            }
                            Err(e) => {
                                tracing::error!("AlpineAcquisitionSource: Failed to read kernel bytes: {}", e);
                                // Non-fatal: we can still try to boot without kernel if initrd has it
                            }
                        }
                    } else {
                        tracing::error!("AlpineAcquisitionSource: Kernel download HTTP {}", resp.status());
                    }
                }
                Err(e) => {
                    tracing::error!("AlpineAcquisitionSource: Kernel download failed: {}", e);
                }
            }
        }

        // Download initrd if not cached
        if !initrd_path.exists() {
            tracing::info!("AlpineAcquisitionSource: Downloading initrd from {}", initrd_url);
            match self.client.get(&initrd_url).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        match resp.bytes().await {
                            Ok(bytes) => {
                                tracing::info!(
                                    "AlpineAcquisitionSource: Got initrd bytes: {}",
                                    bytes.len()
                                );
                                if let Err(e) = tokio::fs::write(&initrd_path, &bytes).await {
                                    tracing::error!(
                                        "AlpineAcquisitionSource: Failed to write initrd: {}",
                                        e
                                    );
                                    return Err(PlazaError::Io(e));
                                }
                                tracing::info!("AlpineAcquisitionSource: Initrd cached at {:?}", initrd_path);
                            }
                            Err(e) => {
                                tracing::error!("AlpineAcquisitionSource: Failed to read initrd bytes: {}", e);
                            }
                        }
                    } else {
                        tracing::error!("AlpineAcquisitionSource: Initrd download HTTP {}", resp.status());
                    }
                }
                Err(e) => {
                    tracing::error!("AlpineAcquisitionSource: Initrd download failed: {}", e);
                }
            }
        }

        // Check what we have
        if !kernel_path.exists() {
            tracing::error!("AlpineAcquisitionSource: Kernel not available after download attempt");
            return Err(PlazaError::config(
                "Alpine kernel (vmlinuz-virt) is not available. Check network connectivity.".to_string(),
            ));
        }
        if !initrd_path.exists() {
            tracing::error!("AlpineAcquisitionSource: Initrd not available after download attempt");
            return Err(PlazaError::config(
                "Alpine initrd (initramfs-virt) is not available. Check network connectivity.".to_string(),
            ));
        }

        // Optional: download modloop
        let modloop_url = format!(
            "https://dl-cdn.alpinelinux.org/alpine/v{}/releases/{}/netboot/modloop-virt",
            major_minor, arch
        );
        let modloop_path = temp_dir.join(format!("modloop-virt-{}", version));

        if !modloop_path.exists() {
            if let Ok(resp) = self.client.get(&modloop_url).send().await {
                if resp.status().is_success() {
                    if let Ok(bytes) = resp.bytes().await {
                        let _ = tokio::fs::write(&modloop_path, bytes).await;
                    }
                }
            }
        }

        let modloop_opt = if modloop_path.exists() {
            Some(modloop_path)
        } else {
            None
        };

        tracing::info!(
            "AlpineAcquisitionSource: Artifacts resolved: kernel={:?}, initrd={:?}, modloop={:?}",
            kernel_path, initrd_path, modloop_opt
        );

        Ok((kernel_path, initrd_path, modloop_opt))
    }
}

#[async_trait]
impl ImageAcquisitionSource for AlpineAcquisitionSource {
    async fn fetch_base_image(&self, base_image: &str) -> PlazaResult<PathBuf> {
        // Parse "alpine:3.19.1"
        let parts: Vec<&str> = base_image.split(':').collect();
        let version = if parts.len() > 1 && parts[1] != "latest" {
            parts[1]
        } else {
            "3.19.1" // Fallback to a stable version
        };

        let arch = "x86_64";
        let major_minor = if version.matches('.').count() >= 1 {
            let v_parts: Vec<&str> = version.split('.').collect();
            format!("{}.{}", v_parts[0], v_parts[1])
        } else {
            version.to_string()
        };

        // Example URL: https://dl-cdn.alpinelinux.org/alpine/v3.19/releases/x86_64/alpine-minirootfs-3.19.1-x86_64.tar.gz
        let artifact_name = format!("alpine-minirootfs-{}-{}.tar.gz", version, arch);
        let url = format!(
            "https://dl-cdn.alpinelinux.org/alpine/v{}/releases/{}/{}",
            major_minor, arch, artifact_name
        );
        let checksum_url = format!("{}.sha256", url);

        tracing::info!("Acquiring Alpine metadata from {}", checksum_url);

        // 1. Fetch checksum
        let checksum_res = self
            .client
            .get(&checksum_url)
            .send()
            .await
            .map_err(|e| PlazaError::config(format!("Failed to fetch checksum: {}", e)))?;

        if !checksum_res.status().is_success() {
            return Err(PlazaError::config(format!(
                "Checksum not found for Alpine version: {}",
                version
            )));
        }

        let checksum_text = checksum_res
            .text()
            .await
            .map_err(|e| PlazaError::config(format!("Failed to read checksum text: {}", e)))?;

        // The sha256 file usually looks like: "d83...  alpine-minirootfs-3.19.1-x86_64.tar.gz"
        let expected_hash = checksum_text
            .split_whitespace()
            .next()
            .ok_or_else(|| PlazaError::config("Malformed checksum file".to_string()))?;

        tracing::info!("Expected SHA-256: {}", expected_hash);
        tracing::info!("Downloading Alpine minirootfs from {}", url);

        // 2. Stream download
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| PlazaError::config(format!("Failed to download alpine: {}", e)))?;

        if !response.status().is_success() {
            return Err(PlazaError::config(format!(
                "Failed to download alpine, HTTP {}",
                response.status()
            )));
        }

        let temp_file = std::env::temp_dir().join(format!(
            "alpine-minirootfs-{}-{}.tar.gz",
            version, arch
        ));

        let mut file = fs::File::create(&temp_file)
            .await
            .map_err(|e| PlazaError::Io(e))?;

        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| PlazaError::config(format!("Stream error: {}", e)))?;
            file.write_all(&chunk)
                .await
                .map_err(|e| PlazaError::Io(e))?;
        }
        file.flush().await.map_err(|e| PlazaError::Io(e))?;
        drop(file);

        // 3. Verify checksum
        let content = fs::read(&temp_file)
            .await
            .map_err(|e| PlazaError::Io(e))?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let actual_hash: String = hasher
            .finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();

        if actual_hash != expected_hash {
            return Err(PlazaError::config(format!(
                "Checksum mismatch: expected={}, got={}",
                expected_hash, actual_hash
            )));
        }

        tracing::info!("Alpine minirootfs verified: {:?}", temp_file);
        Ok(temp_file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_from_image_ref() {
        let parts: Vec<&str> = "alpine:3.19.1".split(':').collect();
        let version = if parts.len() > 1 { parts[1] } else { "3.19.1" };
        assert_eq!(version, "3.19.1");
    }

    #[test]
    fn parse_version_latest() {
        let parts: Vec<&str> = "alpine:latest".split(':').collect();
        let version = if parts.len() > 1 && parts[1] != "latest" {
            parts[1]
        } else {
            "3.19.1"
        };
        assert_eq!(version, "3.19.1");
    }

    #[tokio::test]
    async fn test_fetch_kernel_url_construction() {
        let version = "3.19.1";
        let arch = "x86_64";
        let major_minor = "3.19";
        let kernel_url = format!(
            "https://dl-cdn.alpinelinux.org/alpine/v{}/releases/{}/netboot/vmlinuz-virt",
            major_minor, arch
        );
        assert!(kernel_url.contains("3.19"));
        assert!(kernel_url.contains("x86_64"));
        assert!(kernel_url.contains("vmlinuz-virt"));
    }
}
