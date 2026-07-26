//! Application Data Transfer Objects (DTOs).

use serde::{Deserialize, Serialize};

/// Request payload for creating a new workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub description: Option<String>,
    pub runtime_kind: Option<String>,
    pub image: Option<String>,
    pub cpu_cores: Option<u32>,
    pub memory_mb: Option<u64>,
}

/// Workspace representation exposed to UI and CLI client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub state: String,
    pub runtime_backend: Option<String>,
    pub health: String,
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub created_at: String,
}

