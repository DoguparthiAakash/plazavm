use plaza_foundation::core::PlazaResult;

pub struct LayerManager;

impl LayerManager {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn mount_layers(&self, _layers: &[crate::model::ImageLayer]) -> PlazaResult<()> {
        Err(plaza_foundation::core::PlazaError::storage("Layer mounting not implemented for DP1"))
    }
}
