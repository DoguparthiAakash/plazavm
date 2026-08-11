//! Core models for the fragmented workspace image system.

use plaza_foundation::core::types::Timestamp;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// An error that occurs when parsing a ContentHash.
#[derive(Debug, Error)]
pub enum ContentHashError {
    #[error("Invalid hash format. Expected alg:digest")]
    InvalidFormat,
    #[error("Unsupported hash algorithm: {0}")]
    UnsupportedAlgorithm(String),
    #[error("Invalid digest length or characters")]
    InvalidDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HashAlgorithm {
    Sha256,
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashAlgorithm::Sha256 => write!(f, "sha256"),
        }
    }
}

/// A cryptographic hash representing the content of a blob or layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContentHash {
    pub algorithm: HashAlgorithm,
    pub digest: String,
}

impl ContentHash {
    pub fn new_sha256(digest: impl Into<String>) -> Result<Self, ContentHashError> {
        let digest_str = digest.into();
        if digest_str.len() != 64 || !digest_str.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ContentHashError::InvalidDigest);
        }
        Ok(Self {
            algorithm: HashAlgorithm::Sha256,
            digest: digest_str,
        })
    }
}

impl fmt::Display for ContentHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.algorithm, self.digest)
    }
}

impl FromStr for ContentHash {
    type Err = ContentHashError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(ContentHashError::InvalidFormat);
        }
        
        let alg = match parts[0] {
            "sha256" => HashAlgorithm::Sha256,
            other => return Err(ContentHashError::UnsupportedAlgorithm(other.to_string())),
        };

        match alg {
            HashAlgorithm::Sha256 => Self::new_sha256(parts[1]),
        }
    }
}

/// A parsed reference to an image (e.g. "alpine-dev", "alpine-dev:1.0" or "alpine-dev@sha256:abc...").
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageRef {
    pub name: String,
    pub tag: Option<String>,
    pub digest: Option<ContentHash>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageManifest {
    pub version: u32,
    pub name: String,
    pub image_version: String,
    pub architecture: String,
    pub layers: Vec<ImageLayer>,
    pub kernel: Option<ContentHash>,
    pub initrd: Option<ContentHash>,
    pub boot: BootMetadata,
    pub metadata: ImageMetadata,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BootMetadata {
    pub cmdline: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LayerMediaType {
    RawBlock,
    DiskImage,
    Filesystem,
    Kernel,
    Initrd,
    Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageLayer {
    /// The unique content address of the layer blob.
    pub digest: ContentHash,
    pub size: u64,
    pub media_type: LayerMediaType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub created_at: Timestamp,
    pub author: Option<String>,
    pub labels: std::collections::HashMap<String, String>,
}
