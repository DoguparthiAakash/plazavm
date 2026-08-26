use async_trait::async_trait;
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
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(|e| PlazaError::config(format!("Failed to initialize HTTP client: {}", e)))?;

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

        let temp_dir = std::env::temp_dir().join("plaza-kernel-cache");
        tokio::fs::create_dir_all(&temp_dir)
            .await
            .map_err(PlazaError::Io)?;

        let kernel_path = temp_dir.join(format!("vmlinuz-virt-{}", version));
        let initrd_path = temp_dir.join(format!("initramfs-virt-{}", version));

        if !kernel_path.exists() {
            let resp = self
                .client
                .get(&kernel_url)
                .send()
                .await
                .map_err(|e| PlazaError::process(e.to_string()))?;
            let bytes = resp
                .bytes()
                .await
                .map_err(|e| PlazaError::process(e.to_string()))?;
            tokio::fs::write(&kernel_path, bytes)
                .await
                .map_err(PlazaError::Io)?;
        }

        if !initrd_path.exists() {
            let resp = self
                .client
                .get(&initrd_url)
                .send()
                .await
                .map_err(|e| PlazaError::process(e.to_string()))?;
            let bytes = resp
                .bytes()
                .await
                .map_err(|e| PlazaError::process(e.to_string()))?;
            tokio::fs::write(&initrd_path, bytes)
                .await
                .map_err(PlazaError::Io)?;
        }

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

        let temp_dir = std::env::temp_dir().join("plaza-acquisition");
        fs::create_dir_all(&temp_dir)
            .await
            .map_err(PlazaError::Io)?;

        let unique_id = uuid::Uuid::new_v4();
        let temp_file_path = temp_dir.join(format!("temp_{}.tar.gz", unique_id));

        let mut file = fs::File::create(&temp_file_path)
            .await
            .map_err(PlazaError::Io)?;

        let mut hasher = Sha256::new();
        let mut downloaded_size = 0u64;
        let max_size = 50 * 1024 * 1024; // 50MB bound for minirootfs

        let mut stream = response.bytes_stream();
        while let Some(chunk_res) = stream.next().await {
            let chunk = chunk_res.map_err(|e| {
                let _ = std::fs::remove_file(&temp_file_path); // Cleanup on failure
                PlazaError::config(format!("Stream error: {}", e))
            })?;

            downloaded_size += chunk.len() as u64;
            if downloaded_size > max_size {
                let _ = std::fs::remove_file(&temp_file_path);
                return Err(PlazaError::config(
                    "Download exceeded maximum allowed size (50MB)".to_string(),
                ));
            }

            hasher.update(&chunk);
            if let Err(e) = file.write_all(&chunk).await {
                let _ = std::fs::remove_file(&temp_file_path);
                return Err(PlazaError::Io(e));
            }
        }

        file.flush().await.map_err(|e| {
            let _ = std::fs::remove_file(&temp_file_path);
            PlazaError::Io(e)
        })?;

        // 3. Verify integrity
        let actual_hash: String = hasher
            .finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();
        if actual_hash != expected_hash {
            let _ = std::fs::remove_file(&temp_file_path);
            return Err(PlazaError::config(format!(
                "ImageCorrupted: Hash mismatch. Expected {}, got {}",
                expected_hash, actual_hash
            )));
        }

        tracing::info!("Integrity verified. Artifact securely acquired.");

        // 4. Atomic publish (rename)
        let final_path = temp_dir.join(artifact_name);
        fs::rename(&temp_file_path, &final_path)
            .await
            .map_err(|e| {
                let _ = std::fs::remove_file(&temp_file_path);
                PlazaError::Io(e)
            })?;

        Ok(final_path)
    }
}

/// Factory for getting the appropriate acquisition source for a given base image.
pub fn get_acquisition_source(base_image: &str) -> PlazaResult<Box<dyn ImageAcquisitionSource>> {
    if base_image.starts_with("alpine") {
        Ok(Box::new(AlpineAcquisitionSource::new()?))
    } else {
        Err(PlazaError::config(format!(
            "No userspace acquisition source available for base image: {}",
            base_image
        )))
    }
}
