use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Standard Request Object from any Presentation Layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRequest {
    pub command_id: String,
    pub command_name: String,
    pub arguments: HashMap<String, String>,
    pub workspace_id: Option<String>,
    pub runtime_id: Option<String>,
    pub user: String,
    pub permissions: Vec<String>,
    pub execution_mode: ExecutionMode,
    pub output_format: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExecutionMode {
    Normal,
    DryRun,
    Interactive,
}

/// Standard Response Object to any Presentation Layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResponse {
    pub status: CommandStatus,
    pub exit_code: i32,
    pub duration_ms: u64,
    pub diagnostics: Vec<String>,
    pub metrics: HashMap<String, String>,
    pub warnings: Vec<String>,
    pub events_emitted: Vec<String>,
    pub artifacts_created: Vec<String>,
    pub payload: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommandStatus {
    Success,
    Failed,
    RolledBack,
}

/// A plan representing execution and rollback steps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub steps: Vec<ExecutionStep>,
    pub required_permissions: Vec<String>,
    pub estimated_cost: String,
    pub affected_engines: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    pub name: String,
    pub description: String,
    pub allows_rollback: bool,
}

/// Execution Environment passed to Commands
pub struct CommandContext {
    pub request: CommandRequest,
    // Note: To avoid huge dependency injection here initially,
    // this will house an EngineRegistry reference so commands
    // can request the Workspace, Runtime, etc. dynamically.
    // For now we simulate with a dummy or dynamic resolver.
    // pub engine_registry: Arc<EngineRegistry>,
}

/// Command Capability Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub category: String,
    pub permissions: Vec<String>,
    pub required_engines: Vec<String>,
    pub supports_rollback: bool,
    pub supports_dry_run: bool,
    pub supports_transaction: bool,
    pub supports_interactive_mode: bool,
}
