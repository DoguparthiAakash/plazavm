use plaza_foundation::core::PlazaResult;

pub struct FirstBootSequence;

impl Default for FirstBootSequence {
    fn default() -> Self {
        Self::new()
    }
}

impl FirstBootSequence {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(&self) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
