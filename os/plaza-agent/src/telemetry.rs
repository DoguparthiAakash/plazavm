use plaza_foundation::core::PlazaResult;

pub struct TelemetryCollector;

impl Default for TelemetryCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl TelemetryCollector {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn collect_metrics(&self) -> PlazaResult<()> {
        Ok(()) // DP1 Stub
    }
}
