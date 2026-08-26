use plaza_foundation::core::PlazaResult;

pub struct AbUpdater;

impl Default for AbUpdater {
    fn default() -> Self {
        Self::new()
    }
}

impl AbUpdater {
    pub fn new() -> Self {
        Self
    }

    pub async fn apply_update(&self, _image_path: &str) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
