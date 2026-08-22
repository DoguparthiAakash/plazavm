use crate::model::ImageManifest;
use crate::resolver::parse_image_ref;
use crate::store::{BlobStore, ManifestStore};
use crate::gc::{GarbageCollector, GcReport};
use plaza_foundation::core::{PlazaError, PlazaResult};
use std::collections::HashSet;
use std::sync::Arc;

pub struct ImageManager {
    blob_store: Arc<dyn BlobStore>,
    manifest_store: Arc<dyn ManifestStore>,
    gc: Arc<dyn GarbageCollector>,
}

impl ImageManager {
    pub fn new(
        blob_store: Arc<dyn BlobStore>,
        manifest_store: Arc<dyn ManifestStore>,
        gc: Arc<dyn GarbageCollector>,
    ) -> Self {
        Self {
            blob_store,
            manifest_store,
            gc,
        }
    }

    /// Resolve an image reference to a specific ImageManifest.
    pub async fn resolve_image(&self, reference: &str) -> PlazaResult<ImageManifest> {
        let img_ref = parse_image_ref(reference)?;
        
        let manifest = if let Some(ref tag) = img_ref.tag {
            self.manifest_store.get_manifest(&img_ref.name, tag).await?
        } else {
            // Default to 'latest' if no tag is provided but digest isn't handled directly via manifest store name matching right now.
            // A more complex resolution would look up by digest directly.
            self.manifest_store.get_manifest(&img_ref.name, "latest").await?
        };

        let manifest = manifest.ok_or_else(|| PlazaError::ImageNotFound {
            name: reference.to_string(),
        })?;

        // If a digest was specified, ensure the manifest has a matching digest or its layers match it.
        // For Phase 11, we just return the manifest based on name:tag.
        Ok(manifest)
    }

    /// Fetch/Inspect an image.
    pub async fn inspect_image(&self, reference: &str) -> PlazaResult<ImageManifest> {
        self.resolve_image(reference).await
    }

    /// Get the physical path to a blob by its content hash.
    pub fn get_blob_path(&self, hash: &crate::model::ContentHash) -> PlazaResult<std::path::PathBuf> {
        self.blob_store.get_path(hash)
    }

    /// Import a raw file as a single-layer RawBlock image.
    pub async fn import_raw(&self, name: &str, tag: &str, file_path: &std::path::Path) -> PlazaResult<()> {
        let mut file = tokio::fs::File::open(file_path).await.map_err(PlazaError::Io)?;
        let digest = self.blob_store.put_stream(&mut file).await?;
        
        let meta = tokio::fs::metadata(file_path).await.map_err(PlazaError::Io)?;
        let size = meta.len();

        let manifest = ImageManifest {
            version: 1,
            name: name.to_string(),
            image_version: tag.to_string(),
            architecture: "x86_64".to_string(),
            layers: vec![crate::model::ImageLayer {
                digest,
                size,
                media_type: crate::model::LayerMediaType::RawBlock,
            }],
            kernel: None,
            initrd: None,
            boot: crate::model::BootMetadata::default(),
            metadata: crate::model::ImageMetadata {
                created_at: plaza_foundation::core::types::Timestamp::now(),
                author: None,
                labels: std::collections::HashMap::new(),
            },
        };

        self.manifest_store.put_manifest(&manifest).await?;
        Ok(())
    }

    /// Remove an image reference (does not delete blobs, wait for GC).
    pub async fn remove_image(&self, reference: &str) -> PlazaResult<()> {
        let img_ref = parse_image_ref(reference)?;
        let tag = img_ref.tag.unwrap_or_else(|| "latest".to_string());
        self.manifest_store.remove_manifest(&img_ref.name, &tag).await?;
        Ok(())
    }

    /// Run Garbage Collection.
    pub async fn gc(&self, dry_run: bool) -> PlazaResult<GcReport> {
        // In a complete implementation, this would query ALL manifests and workspaces to build the reachable set.
        // For this Phase 11 baseline, we assume the reachable set is provided by the upper layers or built here.
        let reachable = HashSet::new(); // Stub: would be populated by scanning manifests
        self.gc.run_gc(&reachable, dry_run).await
    }
}
