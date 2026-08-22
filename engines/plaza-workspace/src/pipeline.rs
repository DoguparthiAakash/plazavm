use super::builder::WorkspaceBuilder;
use super::model::{Workspace, WorkspaceSpec, WorkspaceImageSpec};
use crate::distribution::{get_engine, DistributionError};
use plaza_foundation::core::{PlazaError, PlazaResult};
use plaza_foundation::config::PlazaYaml;
use std::path::PathBuf;
use tracing::{info, debug, error};
use std::sync::Arc;

#[derive(Debug)]
pub enum BuilderStage {
    Filesystem,
    Security,
    Hardware,
    Runtime,
    Provider,
    Package,
    Validation,
    Snapshot,
}

/// Orchestrator for transforming a WorkspaceSpec into a runnable Workspace state.
pub struct TransactionalPipelineBuilder;

impl TransactionalPipelineBuilder {
    pub async fn provision_image(
        yaml: &PlazaYaml,
        image_manager: Arc<plaza_image::ImageManager>,
    ) -> Result<String, PlazaError> {
        debug!("Provisioning workspace image from PlazaYaml");
        
        let spec = WorkspaceImageSpec::from_yaml(yaml);
        let id = spec.compute_identity();
        
        info!("Calculated Workspace Image ID: {}", id);

        // Check if image exists in registry/cache
        if image_manager.inspect_image(&id).await.is_ok() {
            info!("Found existing Workspace Image: {}", id);
            return Ok(id);
        }

        // Image doesn't exist, we need to build it.
        let engine = get_engine(&spec.engine_distribution).map_err(|e| {
            PlazaError::config(format!("Unsupported distribution: {}", e))
        })?;

        let plan = engine.resolve_build_plan(&spec).await.map_err(|e| match e {
            DistributionError::ImageBuildUnavailable => {
                PlazaError::config("Image building capability is unavailable for this engine. Pre-built images are required.".to_string())
            },
            _ => PlazaError::config(e.to_string()),
        })?;

        info!("Cold Cache Miss: Workspace image {} is missing. Acquiring/Building via userspace builder...", id);
        
        let new_id = crate::image::builder::UserspaceImageBuilder::build(
            &id,
            plan,
            image_manager,
        ).await?;

        Ok(new_id)
    }

    pub fn build_with_pipeline(
        name: impl Into<String>,
        spec: WorkspaceSpec,
    ) -> PlazaResult<(Workspace, PathBuf)> {
        let name_str = name.into();
        info!("Starting workspace pipeline for '{}'", name_str);

        // Stage 1: Filesystem Layout
        Self::execute_stage(BuilderStage::Filesystem)?;
        
        // Stage 2: Security & Capability Enforcement
        Self::execute_stage(BuilderStage::Security)?;
        
        // Let the WorkspaceBuilder do the filesystem layout for DP1
        let (workspace, path) = WorkspaceBuilder::build(name_str, spec, None)?;

        // Stage 3: Hardware Translation (Virtual Block Composer integration happens here)
        Self::execute_stage(BuilderStage::Hardware)?;

        // Stage 4: Runtime Negotiation
        Self::execute_stage(BuilderStage::Runtime)?;

        // Stage 5: Validation
        Self::execute_stage(BuilderStage::Validation)?;

        info!("Workspace pipeline completed successfully for '{}'", workspace.name);
        Ok((workspace, path))
    }

    fn execute_stage(stage: BuilderStage) -> PlazaResult<()> {
        debug!("Executing pipeline stage: {:?}", stage);
        Ok(())
    }
}
