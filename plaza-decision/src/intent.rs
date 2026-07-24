//! Intent resolution module (R6: intent-based config).

use plaza_config::IntentConfig;
use plaza_workspace::model::WorkspaceSpec;

/// Intent resolver translating high-level user intent into concrete specifications.
pub struct IntentResolver;

impl IntentResolver {
    /// Apply intent settings to a WorkspaceSpec without overriding explicit choices.
    pub fn resolve(intent: &IntentConfig, spec: &mut WorkspaceSpec) {
        if let Some(purpose) = &intent.purpose {
            if (purpose.to_lowercase().contains("ai") || purpose.to_lowercase().contains("ml"))
                && intent.gpu.as_deref() != Some("none")
            {
                spec.resources.gpu_enabled = true;
            }
        }

        if let Some(perf) = &intent.performance {
            match perf.to_lowercase().as_str() {
                "high" | "maximum" => {
                    if spec.resources.cpu_cores < 4 {
                        spec.resources.cpu_cores = 4;
                    }
                    if spec.resources.memory_mb < 4096 {
                        spec.resources.memory_mb = 4096;
                    }
                }
                "low" => {
                    spec.resources.priority = "low".into();
                }
                _ => {}
            }
        }
    }
}
