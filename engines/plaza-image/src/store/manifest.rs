use crate::model::ImageManifest;
use plaza_foundation::core::{PlazaError, PlazaResult};
use std::path::PathBuf;
use tokio::fs;

#[async_trait::async_trait]
pub trait ManifestStore: Send + Sync {
    /// Save an image manifest.
    async fn put_manifest(&self, manifest: &ImageManifest) -> PlazaResult<()>;

    /// Retrieve an image manifest by name and version/tag.
    async fn get_manifest(&self, name: &str, tag: &str) -> PlazaResult<Option<ImageManifest>>;

    /// Delete an image manifest.
    async fn remove_manifest(&self, name: &str, tag: &str) -> PlazaResult<()>;
}

pub struct LocalManifestStore {
    base_dir: PathBuf,
}

impl LocalManifestStore {
    pub async fn new(base_dir: impl Into<PathBuf>) -> PlazaResult<Self> {
        let base_dir = base_dir.into();
        fs::create_dir_all(&base_dir)
            .await
            .map_err(PlazaError::Io)?;
        Ok(Self { base_dir })
    }

    fn manifest_path(&self, name: &str, tag: &str) -> PathBuf {
        self.base_dir.join(format!("{}_{}.json", name, tag))
    }
}

#[async_trait::async_trait]
impl ManifestStore for LocalManifestStore {
    async fn put_manifest(&self, manifest: &ImageManifest) -> PlazaResult<()> {
        let path = self.manifest_path(&manifest.name, &manifest.image_version);
        let data = serde_json::to_string_pretty(manifest).map_err(PlazaError::serialization)?;

        // Write to temp file and rename atomically
        let temp_id = uuid::Uuid::new_v4().to_string();
        let temp_path = self.base_dir.join(format!("tmp_{}.json", temp_id));

        fs::write(&temp_path, data).await.map_err(PlazaError::Io)?;
        fs::rename(&temp_path, &path).await.map_err(|e| {
            // Clean up temp file on failure, ignore cleanup errors
            let _ = std::fs::remove_file(&temp_path);
            PlazaError::Io(e)
        })?;

        Ok(())
    }

    async fn get_manifest(&self, name: &str, tag: &str) -> PlazaResult<Option<ImageManifest>> {
        let path = self.manifest_path(name, tag);
        if !path.exists() {
            return Ok(None);
        }

        let data = fs::read_to_string(&path).await.map_err(PlazaError::Io)?;
        let manifest: ImageManifest =
            serde_json::from_str(&data).map_err(PlazaError::serialization)?;
        Ok(Some(manifest))
    }

    async fn remove_manifest(&self, name: &str, tag: &str) -> PlazaResult<()> {
        let path = self.manifest_path(name, tag);
        if path.exists() {
            fs::remove_file(&path).await.map_err(PlazaError::Io)?;
        }
        Ok(())
    }
}
