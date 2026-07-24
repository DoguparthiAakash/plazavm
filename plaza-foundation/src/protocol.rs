//! Internal Foundation Protocol definitions.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FoundationCommand {
    BuildWorkspace { workspace_id: String },
    StartWorkspace { workspace_id: String },
    StopWorkspace { workspace_id: String },
    SnapshotWorkspace { workspace_id: String, name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FoundationQuery {
    GetWorkspaceStatus { workspace_id: String },
    ListProviders,
    GetPlatformCaps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FoundationResponse {
    Ok { message: String },
    Status { state: String, health: String },
    Error { code: String, message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolEnvelope {
    pub correlation_id: String,
    pub timestamp: plaza_core::types::Timestamp,
    pub payload: FoundationCommand,
}
