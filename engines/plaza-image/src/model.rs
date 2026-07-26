use plaza_foundation::core::types::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub format: ImageFormat,
    pub architecture: String,
    pub os: String,
    pub layers: Vec<ImageLayer>,
    pub metadata: ImageMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    Oci,
    Docker,
    Qcow2,
    Raw,
    AppImage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageLayer {
    pub digest: String,
    pub size: u64,
    pub media_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub created_at: Timestamp,
    pub author: Option<String>,
    pub labels: std::collections::HashMap<String, String>,
}
