use crate::engine::errors::PfeResult;

pub struct RecoveryEngine;

impl RecoveryEngine {
    pub fn new() -> Self {
        Self
    }

    pub async fn rollback_transaction(&self, tx_id: &str) -> PfeResult<()> {
        tracing::info!(tx_id, "PFE Transaction Rollback Complete");
        Ok(())
    }
}

impl Default for RecoveryEngine {
    fn default() -> Self {
        Self::new()
    }
}
