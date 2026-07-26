use crate::model::Workspace;
use plaza_foundation::core::{PlazaError, PlazaResult};
use std::path::PathBuf;
use tracing::info;

pub struct CloningManager {
    workspace_dir: PathBuf,
}

impl CloningManager {
    pub fn new(workspace_dir: PathBuf) -> Self {
        Self { workspace_dir }
    }

    /// Clones an existing workspace into a new one
    pub async fn clone_workspace(&self, source: &Workspace, new_name: &str) -> PlazaResult<Workspace> {
        info!("Cloning workspace {} into {}", source.name, new_name);
        
        // Copy the spec and create a new identity
        let cloned_spec = source.spec.clone();
        let cloned_workspace = Workspace::new(new_name, cloned_spec);
        
        // In a real implementation, we would also copy the filesystem and state
        let target_dir = self.workspace_dir.join(new_name);
        if target_dir.exists() {
            return Err(PlazaError::storage(format!("Target workspace directory already exists: {}", target_dir.display())));
        }
        
        std::fs::create_dir_all(&target_dir)
            .map_err(|e| PlazaError::storage(format!("Failed to create clone directory: {}", e)))?;
            
        Ok(cloned_workspace)
    }
}
