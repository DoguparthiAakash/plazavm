//! DecisionEngine — runtime selection and allocation decision making.

use super::intent::IntentResolver;
use super::scoring::RuntimeScorer;
use plaza_core::{PlazaError, PlazaResult};
use plaza_platform::PlatformDetector;
use plaza_plugin::PluginHost;
use plaza_resource::ResourcePlan;
use plaza_workspace::model::{RuntimeBackendPreference, WorkspaceSpec};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

/// Output decision for a workspace scheduling request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceDecision {
    pub selected_backend: SelectedBackend,
    pub resource_plan: ResourcePlan,
    pub ai_recommendation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedBackend {
    pub backend_id: String,
    pub reason: String,
}

/// Central DecisionEngine.
pub struct DecisionEngine {
    platform: Arc<PlatformDetector>,
    plugin_host: Arc<PluginHost>,
}

impl DecisionEngine {
    pub fn new(platform: Arc<PlatformDetector>, plugin_host: Arc<PluginHost>) -> Self {
        Self {
            platform,
            plugin_host,
        }
    }

    /// Make a complete scheduling decision for a workspace.
    pub async fn decide(&self, mut spec: WorkspaceSpec) -> PlazaResult<WorkspaceDecision> {
        // 1. Resolve intent if present
        if let Some(intent) = spec.intent.clone() {
            IntentResolver::resolve(&intent, &mut spec);
        }

        // 2. Detect host capabilities
        let caps = self.platform.capabilities().await?;

        // 3. Select backend
        let selected_backend = self.select_backend(&spec, &caps).await?;

        // 4. Plan resource allocation
        let resource_plan = ResourcePlan {
            workspace_id: plaza_core::id::WorkspaceId::new(), // Overridden by controller
            cpu_cores: spec.resources.cpu_cores,
            memory_mb: spec.resources.memory_mb,
            gpu_enabled: spec.resources.gpu_enabled,
            priority: match spec.resources.priority.to_lowercase().as_str() {
                "critical" => plaza_resource::WorkspacePriority::Critical,
                "high" => plaza_resource::WorkspacePriority::High,
                "low" => plaza_resource::WorkspacePriority::Low,
                "background" => plaza_resource::WorkspacePriority::Background,
                _ => plaza_resource::WorkspacePriority::Normal,
            },
        };

        info!(
            backend = %selected_backend.backend_id,
            reason = %selected_backend.reason,
            "decision engine selected runtime backend"
        );

        Ok(WorkspaceDecision {
            selected_backend,
            resource_plan,
            ai_recommendation: None,
        })
    }

    async fn select_backend(
        &self,
        spec: &WorkspaceSpec,
        host: &plaza_platform::HostCapabilities,
    ) -> PlazaResult<SelectedBackend> {
        match &spec.runtime.backend {
            RuntimeBackendPreference::Pinned(backend_id) => Ok(SelectedBackend {
                backend_id: backend_id.clone(),
                reason: format!("User explicitly pinned backend '{backend_id}'"),
            }),

            RuntimeBackendPreference::Preferred(backend_id) => {
                if self
                    .plugin_host
                    .get_runtime_plugin(backend_id)
                    .await
                    .is_some()
                {
                    Ok(SelectedBackend {
                        backend_id: backend_id.clone(),
                        reason: format!("User preferred backend '{backend_id}' is available"),
                    })
                } else {
                    self.auto_select(spec, host).await
                }
            }

            RuntimeBackendPreference::Auto => self.auto_select(spec, host).await,
        }
    }

    async fn auto_select(
        &self,
        spec: &WorkspaceSpec,
        host: &plaza_platform::HostCapabilities,
    ) -> PlazaResult<SelectedBackend> {
        let plugins = self.plugin_host.available_runtime_plugins().await;

        if plugins.is_empty() {
            // Fallback default backend for Phase 1 compilation and testing
            let fallback = match spec.runtime.kind {
                plaza_workspace::model::RuntimeKind::Container => "docker",
                plaza_workspace::model::RuntimeKind::VirtualMachine => "qemu",
                _ => "docker",
            };
            return Ok(SelectedBackend {
                backend_id: fallback.to_string(),
                reason: format!("Default auto-selection for {:?}", spec.runtime.kind),
            });
        }

        let mut candidates = Vec::new();
        for plugin in plugins {
            let backend_id = plugin.id();
            let caps = plugin.capabilities();
            let scored = RuntimeScorer::score_candidate(backend_id, &caps, spec, host);
            candidates.push(scored);
        }

        candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        if let Some(best) = candidates.first() {
            Ok(SelectedBackend {
                backend_id: best.backend_id.clone(),
                reason: format!(
                    "Auto-selected with score {:.2} ({})",
                    best.score,
                    best.reasoning.join("; ")
                ),
            })
        } else {
            Err(PlazaError::NoSuitableRuntime {
                reason: "No plugin candidates matched workspace requirements".into(),
            })
        }
    }
}
