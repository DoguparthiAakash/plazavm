//! Workspace templates registry (R2: references runtime images).

use plaza_workspace::model::WorkspaceSpec;
use serde::{Deserialize, Serialize};

/// Pre-built workspace template referencing a runtime image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub runtime_image_id: String,
    pub default_spec: WorkspaceSpec,
}

pub struct WorkspaceTemplateRegistry {
    templates: Vec<WorkspaceTemplate>,
}

impl WorkspaceTemplateRegistry {
    pub fn new() -> Self {
        let default_templates = vec![
            WorkspaceTemplate {
                id: "python-dev".into(),
                name: "Python 3.12 Development".into(),
                description: "Python development environment with uv, pytest, and ruff".into(),
                category: "Development".into(),
                runtime_image_id: "ubuntu-24.04".into(),
                default_spec: WorkspaceSpec::default(),
            },
            WorkspaceTemplate {
                id: "rust-dev".into(),
                name: "Rust Development Environment".into(),
                description: "Rust stable toolchain with cargo, clippy, and rust-analyzer".into(),
                category: "Development".into(),
                runtime_image_id: "ubuntu-24.04".into(),
                default_spec: WorkspaceSpec::default(),
            },
            WorkspaceTemplate {
                id: "ai-research".into(),
                name: "AI & Deep Learning Lab".into(),
                description: "PyTorch, CUDA 12, JupyterLab, and Transformers environment".into(),
                category: "AI & ML".into(),
                runtime_image_id: "cuda-12".into(),
                default_spec: WorkspaceSpec::default(),
            },
        ];

        Self {
            templates: default_templates,
        }
    }

    pub fn list(&self) -> &[WorkspaceTemplate] {
        &self.templates
    }

    pub fn get(&self, id: &str) -> Option<&WorkspaceTemplate> {
        self.templates.iter().find(|t| t.id == id)
    }
}

impl Default for WorkspaceTemplateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

