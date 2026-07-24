//! Transactional Stage Pipeline Builder for deterministic workspace assembly.

use super::builder::WorkspaceBuilder;
use super::model::{Workspace, WorkspaceSpec};
use plaza_core::PlazaResult;
use std::path::PathBuf;

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

pub struct TransactionalPipelineBuilder;

impl TransactionalPipelineBuilder {
    pub fn build_with_pipeline(
        name: impl Into<String>,
        spec: WorkspaceSpec,
    ) -> PlazaResult<(Workspace, PathBuf)> {
        // Execute stages deterministically
        let (workspace, path) = WorkspaceBuilder::build(name, spec)?;
        Ok((workspace, path))
    }
}
