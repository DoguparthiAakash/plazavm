use crate::model::WorkspaceImageSpec;
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum DistributionError {
    #[error("Unsupported engine distribution: {0}")]
    UnsupportedDistribution(String),
    #[error("Image building capability is unavailable for this engine. Pre-built images are required.")]
    ImageBuildUnavailable,
}

/// Represents the operations required to build a workspace image from a spec.
/// Note: In Phase 16, PlazaVM does NOT execute these operations on the host.
/// It only produces this plan.
#[derive(Debug, Clone)]
pub struct WorkspaceImageBuildPlan {
    pub base_image: String,
    pub install_commands: Vec<String>,
}

#[async_trait]
pub trait EngineDistribution: Send + Sync {
    /// Validate the spec and produce a logical build plan.
    /// Returns `ImageBuildUnavailable` instead of actually executing host commands.
    async fn resolve_build_plan(&self, spec: &WorkspaceImageSpec) -> Result<WorkspaceImageBuildPlan, DistributionError>;
}

pub struct AlpineEngine;

#[async_trait]
impl EngineDistribution for AlpineEngine {
    async fn resolve_build_plan(&self, spec: &WorkspaceImageSpec) -> Result<WorkspaceImageBuildPlan, DistributionError> {
        let base = spec.base_image.clone().unwrap_or_else(|| "alpine:latest".to_string());
        let mut commands = Vec::new();
        
        if !spec.packages.is_empty() || !spec.tools.is_empty() {
            let mut pkgs = spec.packages.clone();
            pkgs.extend(spec.tools.clone());
            commands.push(format!("apk add --no-cache {}", pkgs.join(" ")));
        }

        // Return a build plan, but actual image construction is deferred/unsupported in Phase 16.
        Ok(WorkspaceImageBuildPlan {
            base_image: base,
            install_commands: commands,
        })
    }
}

pub struct FedoraEngine;

#[async_trait]
impl EngineDistribution for FedoraEngine {
    async fn resolve_build_plan(&self, _spec: &WorkspaceImageSpec) -> Result<WorkspaceImageBuildPlan, DistributionError> {
        Err(DistributionError::UnsupportedDistribution("fedora".to_string()))
    }
}

pub struct ArchEngine;

#[async_trait]
impl EngineDistribution for ArchEngine {
    async fn resolve_build_plan(&self, _spec: &WorkspaceImageSpec) -> Result<WorkspaceImageBuildPlan, DistributionError> {
        Err(DistributionError::UnsupportedDistribution("arch".to_string()))
    }
}

/// Factory for getting the correct distribution engine.
pub fn get_engine(distribution: &str) -> Result<Box<dyn EngineDistribution>, DistributionError> {
    if distribution.starts_with("alpine") {
        Ok(Box::new(AlpineEngine))
    } else if distribution.starts_with("fedora") {
        Ok(Box::new(FedoraEngine))
    } else if distribution.starts_with("arch") {
        Ok(Box::new(ArchEngine))
    } else {
        Err(DistributionError::UnsupportedDistribution(distribution.to_string()))
    }
}
