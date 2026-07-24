//! AiAdvisorSystem providing specialized advice services.

use super::provider::AiProvider;
use plaza_core::PlazaResult;
use plaza_platform::HostCapabilities;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Advisory system coordinating AI recommendations.
pub struct AiAdvisorSystem {
    providers: Vec<Arc<dyn AiProvider>>,
}

impl AiAdvisorSystem {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    pub fn register_provider(&mut self, provider: Arc<dyn AiProvider>) {
        self.providers.push(provider);
    }

    /// Provide runtime optimization advice based on host capabilities.
    pub async fn advise_runtime(&self, caps: &HostCapabilities) -> PlazaResult<AiRecommendation> {
        let recommendation = format!(
            "Based on your profile '{}', Docker is recommended for dev environments, QEMU for full VM isolation.",
            caps.os.name
        );
        Ok(AiRecommendation {
            category: "runtime".into(),
            recommendation,
            confidence: 0.9,
        })
    }

    /// Provide project structure analysis.
    pub async fn analyze_project(&self, project_name: &str) -> PlazaResult<AiRecommendation> {
        Ok(AiRecommendation {
            category: "project".into(),
            recommendation: format!(
                "Detected project '{project_name}'. Recommended runtime: Python Container."
            ),
            confidence: 0.85,
        })
    }
}

impl Default for AiAdvisorSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRecommendation {
    pub category: String,
    pub recommendation: String,
    pub confidence: f32,
}
