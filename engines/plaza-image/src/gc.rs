//! Reference-tracked garbage collection.
//!
//! Removes unreferenced layers and blobs from the content-addressable store.

use crate::model::ContentHash;
use plaza_foundation::core::{PlazaError, PlazaResult};
use std::collections::HashSet;

pub struct GcReport {
    pub deleted_blobs: usize,
    pub freed_bytes: u64,
}

#[async_trait::async_trait]
pub trait GarbageCollector: Send + Sync {
    /// Collect all root references across the system (manifests, workspaces, etc).
    /// For Phase 11, the caller will supply the reachable set.
    async fn run_gc(
        &self,
        reachable_hashes: &HashSet<ContentHash>,
        dry_run: bool,
    ) -> PlazaResult<GcReport>;
}

pub struct LocalGarbageCollector {
    blob_store_path: std::path::PathBuf,
}

impl LocalGarbageCollector {
    pub fn new(blob_store_path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            blob_store_path: blob_store_path.into(),
        }
    }
}

#[async_trait::async_trait]
impl GarbageCollector for LocalGarbageCollector {
    async fn run_gc(
        &self,
        reachable_hashes: &HashSet<ContentHash>,
        dry_run: bool,
    ) -> PlazaResult<GcReport> {
        let mut deleted_blobs = 0;
        let mut freed_bytes = 0;

        let sha256_dir = self.blob_store_path.join("sha256");
        if !sha256_dir.exists() {
            return Ok(GcReport {
                deleted_blobs,
                freed_bytes,
            });
        }

        let mut entries = tokio::fs::read_dir(&sha256_dir)
            .await
            .map_err(PlazaError::Io)?;
        while let Some(entry) = entries.next_entry().await.map_err(PlazaError::Io)? {
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                if let Ok(hash) = ContentHash::new_sha256(file_name.as_ref()) {
                    if !reachable_hashes.contains(&hash) {
                        let meta = entry.metadata().await.map_err(PlazaError::Io)?;
                        let size = meta.len();

                        if !dry_run {
                            tokio::fs::remove_file(&path)
                                .await
                                .map_err(PlazaError::Io)?;
                        }

                        deleted_blobs += 1;
                        freed_bytes += size;
                    }
                }
            }
        }

        Ok(GcReport {
            deleted_blobs,
            freed_bytes,
        })
    }
}
