use plaza_foundation::core::PlazaResult;

pub struct ClipboardSync;

impl Default for ClipboardSync {
    fn default() -> Self {
        Self::new()
    }
}

impl ClipboardSync {
    pub fn new() -> Self {
        Self
    }

    pub async fn sync(&self) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
