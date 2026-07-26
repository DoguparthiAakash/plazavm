use crate::model::ImageManifest;
use plaza_foundation::core::{PlazaError, PlazaResult};

pub struct ImageManager;

impl ImageManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn fetch_image(&self, reference: &str) -> PlazaResult<ImageManifest> {
        // DP1 stub
        Err(PlazaError::storage(format!("Image fetching not implemented for DP1: {}", reference)))
    }

    pub async fn inspect_image(&self, _reference: &str) -> PlazaResult<ImageManifest> {
        // DP1 stub
        Err(PlazaError::storage("Image inspection not implemented for DP1"))
    }
}
