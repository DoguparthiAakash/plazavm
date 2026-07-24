//! Runtime scoring algorithms.

use plaza_platform::HostCapabilities;
use plaza_runtime::RuntimeCapabilities;
use plaza_workspace::model::WorkspaceSpec;
use serde::{Deserialize, Serialize};

/// Scored candidate runtime plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredCandidate {
    pub backend_id: String,
    pub score: f64,
    pub reasoning: Vec<String>,
}

pub struct RuntimeScorer;

impl RuntimeScorer {
    /// Score a candidate runtime plugin against a workspace spec and host capabilities.
    pub fn score_candidate(
        backend_id: &str,
        caps: &RuntimeCapabilities,
        spec: &WorkspaceSpec,
        host: &HostCapabilities,
    ) -> ScoredCandidate {
        let mut score: f64 = 0.10;
        let mut reasoning = Vec::new();

        // OS support check
        if caps.supports_os(&spec.runtime.os) {
            score += 0.25;
            reasoning.push(format!("Supports guest OS '{:?}'", spec.runtime.os));
        } else {
            score -= 0.40;
            reasoning.push(format!("Does not support guest OS '{:?}'", spec.runtime.os));
        }

        // Arch support check
        if caps.supports_arch(&spec.runtime.arch) {
            score += 0.15;
            reasoning.push(format!("Supports architecture '{:?}'", spec.runtime.arch));
        }

        // Docker preference for container workloads
        if spec.runtime.kind == plaza_workspace::model::RuntimeKind::Container
            && backend_id == "docker"
        {
            score += 0.30;
            reasoning.push("Docker is native for container workloads".into());
        }

        // VirtualBox/QEMU preference for full VMs
        if spec.runtime.kind == plaza_workspace::model::RuntimeKind::VirtualMachine {
            if backend_id == "virtualbox" {
                score += 0.30; // Preferred desktop VM manager
                reasoning.push("VirtualBox desktop VM manager preference".into());
            } else if backend_id == "qemu" {
                score += 0.20;
                reasoning.push("QEMU hypervisor match".into());
            }
        }

        // GPU requirement check
        if spec.resources.gpu_enabled {
            if caps.can_gpu_passthrough {
                score += 0.15;
                reasoning.push("Supports GPU passthrough".into());
            } else {
                score -= 0.3;
                reasoning.push("Lacks GPU passthrough support".into());
            }
        }

        let _ = host; // Used for profile fit check in Phase 2

        ScoredCandidate {
            backend_id: backend_id.to_string(),
            score: score.clamp(0.0, 1.0),
            reasoning,
        }
    }
}
