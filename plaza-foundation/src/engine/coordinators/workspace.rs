use plaza_core::PlazaResult;
use plaza_workspace::builder::WorkspaceBuilder;
use plaza_workspace::model::{Workspace, WorkspaceSpec};
use std::path::PathBuf;

/// Thin coordinator delegating workspace builder operations to `plaza-workspace`.
pub struct WorkspaceCoordinator;

impl WorkspaceCoordinator {
    pub fn new() -> Self {
        Self
    }

    pub fn build_workspace(
        &self,
        name: impl Into<String>,
        spec: WorkspaceSpec,
    ) -> PlazaResult<(Workspace, PathBuf)> {
        WorkspaceBuilder::build(name, spec)
    }
}

impl Default for WorkspaceCoordinator {
    fn default() -> Self {
        Self::new()
    }
}
