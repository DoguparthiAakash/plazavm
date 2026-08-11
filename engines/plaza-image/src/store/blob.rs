use crate::model::{ContentHash, HashAlgorithm};
use plaza_foundation::core::{PlazaError, PlazaResult};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[async_trait::async_trait]
pub trait BlobStore: Send + Sync {
    /// Read a stream and store it atomically. Returns the computed ContentHash.
    async fn put_stream(&self, reader: &mut (dyn tokio::io::AsyncRead + Unpin + Send)) -> PlazaResult<ContentHash>;

    /// Check if a blob exists by hash.
    async fn exists(&self, hash: &ContentHash) -> PlazaResult<bool>;

    /// Get the host filesystem path for the blob.
    fn get_path(&self, hash: &ContentHash) -> PlazaResult<PathBuf>;

    /// Remove a blob.
    async fn remove(&self, hash: &ContentHash) -> PlazaResult<()>;
}

/// A BlobStore backed by the local filesystem.
pub struct LocalBlobStore {
    base_dir: PathBuf,
}

impl LocalBlobStore {
    pub async fn new(base_dir: impl Into<PathBuf>) -> PlazaResult<Self> {
        let base_dir = base_dir.into();
        fs::create_dir_all(&base_dir).await.map_err(PlazaError::Io)?;
        let temp_dir = base_dir.join("tmp");
        fs::create_dir_all(&temp_dir).await.map_err(PlazaError::Io)?;
        let sha256_dir = base_dir.join("sha256");
        fs::create_dir_all(&sha256_dir).await.map_err(PlazaError::Io)?;

        Ok(Self { base_dir })
    }

    fn temp_dir(&self) -> PathBuf {
        self.base_dir.join("tmp")
    }

    fn sha256_dir(&self) -> PathBuf {
        self.base_dir.join("sha256")
    }
}

#[async_trait::async_trait]
impl BlobStore for LocalBlobStore {
    async fn put_stream(&self, reader: &mut (dyn tokio::io::AsyncRead + Unpin + Send)) -> PlazaResult<ContentHash> {
        let temp_dir = self.temp_dir();
        // create temp file
        let temp_id = uuid::Uuid::new_v4().to_string();
        let temp_path = temp_dir.join(&temp_id);

        let mut file = fs::File::create(&temp_path).await.map_err(PlazaError::Io)?;
        
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 8192];
        
        loop {
            let n = reader.read(&mut buffer).await.map_err(PlazaError::Io)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
            file.write_all(&buffer[..n]).await.map_err(PlazaError::Io)?;
        }
        file.flush().await.map_err(PlazaError::Io)?;
        drop(file);

        let digest = hex::encode(hasher.finalize());
        let content_hash = ContentHash::new_sha256(&digest).map_err(|e| PlazaError::storage(e))?;
        let target_path = self.get_path(&content_hash)?;

        // If it already exists, we can discard the temp file.
        if target_path.exists() {
            let _ = fs::remove_file(temp_path).await;
            return Ok(content_hash);
        }

        // Atomic rename
        if let Err(e) = fs::rename(&temp_path, &target_path).await {
            let _ = fs::remove_file(temp_path).await;
            return Err(PlazaError::Io(e));
        }

        Ok(content_hash)
    }

    async fn exists(&self, hash: &ContentHash) -> PlazaResult<bool> {
        let path = self.get_path(hash)?;
        Ok(path.exists())
    }

    fn get_path(&self, hash: &ContentHash) -> PlazaResult<PathBuf> {
        match hash.algorithm {
            HashAlgorithm::Sha256 => Ok(self.sha256_dir().join(&hash.digest)),
        }
    }

    async fn remove(&self, hash: &ContentHash) -> PlazaResult<()> {
        let path = self.get_path(hash)?;
        if path.exists() {
            fs::remove_file(path).await.map_err(PlazaError::Io)?;
        }
        Ok(())
    }
}
