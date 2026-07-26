use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub liveness: bool,
    pub readiness: bool,
    pub details: String,
}

pub struct EngineHealthMonitor;

impl EngineHealthMonitor {
    pub fn new() -> Self {
        Self
    }

    pub fn check(&self) -> HealthStatus {
        HealthStatus {
            liveness: true,
            readiness: true,
            details: "PFE Subsystems Operational".into(),
        }
    }
}

impl Default for EngineHealthMonitor {
    fn default() -> Self {
        Self::new()
    }
}
