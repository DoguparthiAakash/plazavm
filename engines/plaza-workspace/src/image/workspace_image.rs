//! Workspace Image Builder (Optimized)
//!
//! Creates ext4 disk images from a source directory.
//! Designed for minimal memory and disk usage:
//! - Auto-sizes images based on source size (not hardcoded 512MB/1GB)
//! - Streaming SHA256 (no full-file read into memory)
//! - Bounded memory per file read
//! - Skips build artifacts automatically

use plaza_foundation::core::{PlazaError, PlazaResult};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

/// Max single file read size (10MB) to prevent memory spikes
const MAX_FILE_READ: u64 = 10 * 1024 * 1024;
/// Max total image size cap (4GB)
const MAX_IMAGE_MB: u64 = 4096;
/// Overhead multiplier on source size (ext4 metadata, inode tables, etc.)
const OVERHEAD_MULTIPLIER: f64 = 1.3;
/// Minimum image size in MB
const MIN_IMAGE_MB: u64 = 64;

/// Configuration for building a workspace image.
#[derive(Debug, Clone)]
pub struct WorkspaceImageConfig {
    pub source_dir: PathBuf,
    pub output_path: PathBuf,
    pub size_mb: Option<u64>, // None = auto-size from source
    pub workspace_name: String,
}

impl WorkspaceImageConfig {
    pub fn new(
        source_dir: impl Into<PathBuf>,
        output_path: impl Into<PathBuf>,
        workspace_name: impl Into<String>,
    ) -> Self {
        Self {
            source_dir: source_dir.into(),
            output_path: output_path.into(),
            size_mb: None, // auto-size by default
            workspace_name: workspace_name.into(),
        }
    }

    pub fn with_size_mb(mut self, size_mb: u64) -> Self {
        self.size_mb = Some(size_mb);
        self
    }
}

/// Build result with metadata.
#[derive(Debug)]
pub struct WorkspaceImageResult {
    pub image_path: PathBuf,
    pub image_size_bytes: u64,
    pub source_size_bytes: u64,
    pub file_count: usize,
    pub sha256: String,
}

/// Auto-calculate optimal image size from source size.
fn auto_size_mb(source_bytes: u64) -> u64 {
    let with_overhead = (source_bytes as f64 * OVERHEAD_MULTIPLIER) as u64;
    let mb = (with_overhead / (1024 * 1024)) + 1; // +1 for rounding
    let mb = mb.max(MIN_IMAGE_MB);
    let mb = mb.min(MAX_IMAGE_MB);
    mb
}

/// Build an ext4 workspace image from a source directory.
pub async fn build_workspace_image(config: WorkspaceImageConfig) -> PlazaResult<WorkspaceImageResult> {
    if !config.source_dir.exists() {
        return Err(PlazaError::storage(format!(
            "Source directory does not exist: {:?}",
            config.source_dir
        )));
    }

    if let Some(parent) = config.output_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            PlazaError::storage(format!("Failed to create output directory: {}", e))
        })?;
    }

    let source_size = calculate_dir_size(&config.source_dir)?;
    let file_count = count_files(&config.source_dir)?;

    // Auto-size: source_size * 1.3 + ext4 overhead, clamped to [64MB, 4096MB]
    let size_mb = config.size_mb.unwrap_or_else(|| auto_size_mb(source_size));
    let image_size_bytes = size_mb * 1024 * 1024;

    info!(
        "Building image '{}' : {} files, {} bytes source -> {} MB image",
        config.workspace_name, file_count, source_size, size_mb
    );

    let image_path = config.output_path.clone();
    let image_path_clone = image_path.clone();
    let source_dir_clone = config.source_dir.clone();
    let workspace_name = config.workspace_name.clone();

    tokio::task::spawn_blocking(move || -> PlazaResult<()> {
        // Step 1: Format ext4 image
        info!("Creating {} MB ext4 image...", size_mb);
        let mut fmt = arcbox_ext4::Formatter::with_options(
            &image_path_clone,
            arcbox_ext4::FormatOptions::new(image_size_bytes)
                .label(&workspace_name),
        )
        .map_err(|e| PlazaError::storage(format!("ext4 format failed: {}", e)))?;

        // Step 2: Create /workspace directory
        fmt.create(
            "/workspace",
            0o40755,
            None, None, None,
            Some(0), Some(0), None,
        )
        .map_err(|e| PlazaError::storage(format!("mkdir /workspace: {}", e)))?;

        // Step 3: Copy source files
        let mut files_written = 0u32;
        copy_directory_into_ext4(&mut fmt, &source_dir_clone, "/workspace", &mut files_written)?;
        info!("Written {} files", files_written);

        // Step 4: Finalize
        fmt.close()
            .map_err(|e| PlazaError::storage(format!("ext4 close: {}", e)))?;

        Ok(())
    })
    .await
    .map_err(|e| PlazaError::process(format!("Build task failed: {}", e)))??;

    // Streaming SHA256 - read in 64KB chunks, not full file
    let sha256 = compute_file_sha256_streaming(&image_path)?;
    let final_size = std::fs::metadata(&image_path)
        .map(|m| m.len())
        .unwrap_or(0);

    info!("Image built: {} bytes, SHA256: {}", final_size, &sha256[..16]);

    Ok(WorkspaceImageResult {
        image_path,
        image_size_bytes: final_size,
        source_size_bytes: source_size,
        file_count,
        sha256,
    })
}

/// Recursively copy a directory into an ext4 image.
/// Memory bounded: reads files in chunks, skips artifacts.
fn copy_directory_into_ext4(
    fmt: &mut arcbox_ext4::Formatter,
    source: &Path,
    target: &str,
    count: &mut u32,
) -> PlazaResult<()> {
    let entries = std::fs::read_dir(source)
        .map_err(|e| PlazaError::storage(format!("read_dir {:?}: {}", source, e)))?;

    for entry in entries {
        let entry = entry.map_err(|e| PlazaError::storage(e.to_string()))?;
        let metadata = entry
            .metadata()
            .map_err(|e| PlazaError::storage(e.to_string()))?;

        let file_name = entry.file_name().to_string_lossy().to_string();
        let entry_path = format!("{}/{}", target, file_name);

        // Skip hidden files
        if file_name.starts_with('.') {
            continue;
        }

        // Skip heavy build artifacts
        match file_name.as_str() {
            "target" | "node_modules" | ".git" | "__pycache__" | ".cache" | "dist" => {
                debug!("skip: {}", entry_path);
                continue;
            }
            _ => {}
        }

        if metadata.is_dir() {
            fmt.create(&entry_path, 0o40755, None, None, None, Some(0), Some(0), None)
                .map_err(|e| PlazaError::storage(format!("mkdir {}: {}", entry_path, e)))?;
            copy_directory_into_ext4(fmt, &entry.path(), &entry_path, count)?;
        } else if metadata.is_file() {
            // Skip files over 10MB (binaries, disk images, etc.)
            let file_size = metadata.len();
            if file_size > MAX_FILE_READ {
                debug!("skip large: {} ({} bytes)", entry_path, file_size);
                continue;
            }

            // Read file - bounded memory
            let mut content = Vec::with_capacity(file_size.min(1024 * 1024) as usize);
            let mut file = std::fs::File::open(entry.path())
                .map_err(|e| PlazaError::storage(format!("open {:?}: {}", entry.path(), e)))?;
            file.read_to_end(&mut content)
                .map_err(|e| PlazaError::storage(format!("read {:?}: {}", entry.path(), e)))?;

            let mode = if is_executable(metadata.permissions()) {
                0o100755
            } else {
                0o100644
            };

            fmt.create(&entry_path, mode, None, None,
                Some(&mut content.as_slice()), Some(0), Some(0), None)
                .map_err(|e| PlazaError::storage(format!("write {}: {}", entry_path, e)))?;

            *count += 1;
            if *count % 200 == 0 {
                debug!("  {} files...", count);
            }
        } else if metadata.file_type().is_symlink() {
            if let Ok(link_target) = std::fs::read_link(entry.path()) {
                fmt.create(&entry_path, 0o120777,
                    Some(&link_target.to_string_lossy()), None, None, Some(0), Some(0), None)
                    .ok();
            }
        }
    }
    Ok(())
}

#[cfg(unix)]
fn is_executable(perms: std::fs::Permissions) -> bool {
    use std::os::unix::fs::PermissionsExt;
    perms.mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn is_executable(_perms: std::fs::Permissions) -> bool {
    false
}

/// Calculate total size of a directory (excluding build artifacts).
fn calculate_dir_size(path: &Path) -> PlazaResult<u64> {
    let mut total = 0u64;
    if path.is_dir() {
        for entry in std::fs::read_dir(path).map_err(|e| PlazaError::storage(e.to_string()))? {
            let entry = entry.map_err(|e| PlazaError::storage(e.to_string()))?;
            let metadata = entry.metadata().map_err(|e| PlazaError::storage(e.to_string()))?;
            if metadata.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                match name.as_str() {
                    "target" | "node_modules" | ".git" | "__pycache__" | ".cache" | "dist" => continue,
                    _ => {}
                }
                total += calculate_dir_size(&entry.path())?;
            } else {
                total += metadata.len();
            }
        }
    }
    Ok(total)
}

/// Count files recursively (excluding build artifacts).
fn count_files(path: &Path) -> PlazaResult<usize> {
    let mut count = 0;
    if path.is_dir() {
        for entry in std::fs::read_dir(path).map_err(|e| PlazaError::storage(e.to_string()))? {
            let entry = entry.map_err(|e| PlazaError::storage(e.to_string()))?;
            let metadata = entry.metadata().map_err(|e| PlazaError::storage(e.to_string()))?;
            if metadata.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                match name.as_str() {
                    "target" | "node_modules" | ".git" | "__pycache__" | ".cache" | "dist" => continue,
                    _ => {}
                }
                count += count_files(&entry.path())?;
            } else {
                count += 1;
            }
        }
    }
    Ok(count)
}

/// Streaming SHA256 - reads file in 64KB chunks to avoid loading entire file into memory.
fn compute_file_sha256_streaming(path: &Path) -> PlazaResult<String> {
    use std::io::BufReader;
    let file = std::fs::File::open(path)
        .map_err(|e| PlazaError::storage(format!("open for SHA256: {}", e)))?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 65536]; // 64KB buffer
    loop {
        let n = reader.read(&mut buf)
            .map_err(|e| PlazaError::storage(format!("read for SHA256: {}", e)))?;
        if n == 0 { break; }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn auto_size_scales_with_source() {
        // 10MB source should produce ~64MB (min)
        assert_eq!(auto_size_mb(10 * 1024 * 1024), 64);
        // 100MB source should produce ~130MB
        let mb = auto_size_mb(100 * 1024 * 1024);
        assert!(mb >= 100 && mb <= 200);
        // 1GB source should produce ~1.3GB
        let mb = auto_size_mb(1024 * 1024 * 1024);
        assert!(mb >= 1000 && mb <= 2000);
    }

    #[test]
    fn calculate_dir_size_works() {
        let temp_dir = std::env::temp_dir().join("plaza-ws-img-test");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(temp_dir.join("sub")).unwrap();
        fs::write(temp_dir.join("file.txt"), "hello").unwrap();
        fs::write(temp_dir.join("sub/file2.txt"), "world!").unwrap();
        let size = calculate_dir_size(&temp_dir).unwrap();
        assert!(size > 0);
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn count_files_works() {
        let temp_dir = std::env::temp_dir().join("plaza-ws-count-test");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();
        fs::write(temp_dir.join("a.txt"), "a").unwrap();
        fs::write(temp_dir.join("b.txt"), "b").unwrap();
        let count = count_files(&temp_dir).unwrap();
        assert_eq!(count, 2);
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn streaming_sha256_matches_regular() {
        let temp_file = std::env::temp_dir().join("plaza-sha256-stream-test.txt");
        fs::write(&temp_file, "hello world test data").unwrap();
        let hash = compute_file_sha256_streaming(&temp_file).unwrap();
        assert_eq!(hash.len(), 64);
        // Should match direct SHA256
        let content = fs::read(&temp_file).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let expected: String = hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(hash, expected);
        let _ = fs::remove_file(&temp_file);
    }
}
