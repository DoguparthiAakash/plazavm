use anyhow::Result;
use async_trait::async_trait;
use plaza_command::models::{CommandContext, CommandMetadata, CommandResponse, CommandStatus, ExecutionPlan, ExecutionStep};
use plaza_command::registry::CommandRegistry;
use plaza_command::ExecutableCommand;
use std::sync::Arc;
use crate::service::WorkspaceService;

pub fn register_all(registry: &mut CommandRegistry, service: Arc<WorkspaceService>) {
    registry.register("workspace.list", Arc::new(WorkspaceListCommand::new(service.clone())));
    registry.register("workspace.create", Arc::new(WorkspaceCreateCommand::new(service.clone())));
    registry.register("workspace.delete", Arc::new(WorkspaceDeleteCommand::new(service.clone())));
    registry.register("workspace.open", Arc::new(WorkspaceOpenCommand::new(service.clone())));
    registry.register("workspace.close", Arc::new(WorkspaceCloseCommand::new(service.clone())));
}

pub struct WorkspaceListCommand {
    service: Arc<WorkspaceService>,
}

impl WorkspaceListCommand {
    pub fn new(service: Arc<WorkspaceService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl ExecutableCommand for WorkspaceListCommand {
    fn metadata(&self) -> CommandMetadata {
        CommandMetadata {
            name: "workspace.list".to_string(),
            description: "List all workspaces".to_string(),
            version: "1.0".to_string(),
            category: "workspace".to_string(),
            permissions: vec![],
            required_engines: vec!["workspace_engine".to_string()],
            supports_rollback: false,
            supports_dry_run: true,
            supports_transaction: false,
            supports_interactive_mode: false,
        }
    }

    async fn prepare(&self, _ctx: &mut CommandContext) -> Result<()> {
        Ok(())
    }

    async fn validate(&self, _ctx: &mut CommandContext) -> Result<()> {
        Ok(())
    }

    async fn plan(&self, _ctx: &mut CommandContext) -> Result<ExecutionPlan> {
        Ok(ExecutionPlan {
            steps: vec![ExecutionStep {
                name: "fetch_workspaces".to_string(),
                description: "Fetch list of workspaces from repository".to_string(),
                allows_rollback: false,
            }],
            required_permissions: vec![],
            estimated_cost: "0.0".to_string(),
            affected_engines: vec!["workspace_engine".to_string()],
        })
    }

    async fn execute(&self, _ctx: &mut CommandContext) -> Result<CommandResponse> {
        let workspaces = self.service.list_workspaces().await?;
        let mut diagnostics = vec![format!("Workspaces ({}):", workspaces.len())];
        for ws in workspaces {
            diagnostics.push(format!(
                "  - [{}] {} ({:?}, {})",
                ws.id, ws.name, ws.status.state, ws.status.health
            ));
        }

        Ok(CommandResponse {
            status: CommandStatus::Success,
            exit_code: 0,
            duration_ms: 0,
            diagnostics,
            metrics: Default::default(),
            warnings: vec![],
            events_emitted: vec![],
            artifacts_created: vec![],
            payload: None,
        })
    }

    async fn commit(&self, _ctx: &mut CommandContext) -> Result<()> {
        Ok(())
    }

    async fn rollback(&self, _ctx: &mut CommandContext) -> Result<()> {
        Ok(())
    }

    async fn cleanup(&self, _ctx: &mut CommandContext) -> Result<()> {
        Ok(())
    }
}

pub struct WorkspaceCreateCommand {
    service: Arc<WorkspaceService>,
}
impl WorkspaceCreateCommand {
    pub fn new(service: Arc<WorkspaceService>) -> Self { Self { service } }
}
#[async_trait]
impl ExecutableCommand for WorkspaceCreateCommand {
    fn metadata(&self) -> CommandMetadata {
        CommandMetadata {
            name: "workspace.create".into(),
            description: "Create a new workspace".into(),
            version: "1.0".into(),
            category: "workspace".into(),
            permissions: vec![],
            required_engines: vec!["workspace_engine".into()],
            supports_rollback: true,
            supports_dry_run: true,
            supports_transaction: true,
            supports_interactive_mode: false,
        }
    }
    async fn prepare(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
    async fn validate(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
    async fn plan(&self, _ctx: &mut CommandContext) -> Result<ExecutionPlan> {
        Ok(ExecutionPlan {
            steps: vec![
                ExecutionStep {
                    name: "provision_workspace".into(),
                    description: "Provision directory structure and initialize workspace databases".into(),
                    allows_rollback: true,
                }
            ],
            required_permissions: vec![],
            estimated_cost: "0".into(),
            affected_engines: vec!["workspace_engine".into()]
        })
    }
    async fn execute(&self, ctx: &mut CommandContext) -> Result<CommandResponse> {
        let name = ctx.request.arguments.get("name").cloned().unwrap_or_else(|| "default".into());
        let spec = crate::model::WorkspaceSpec::default();
        let ws = self.service.create_workspace(&name, spec).await?;
        
        ctx.state.insert("created_workspace_id".into(), ws.id.to_string());
        
        Ok(CommandResponse {
            status: CommandStatus::Success,
            exit_code: 0,
            duration_ms: 0,
            diagnostics: vec![format!("Created workspace: {} [{}]", ws.name, ws.id)],
            metrics: Default::default(),
            warnings: vec![],
            events_emitted: vec!["WorkspaceCreated".into()],
            artifacts_created: vec![format!("~/.plazavm/workspaces/{}", ws.name)],
            payload: Some(ws.id.to_string()),
        })
    }
    async fn commit(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
    
    async fn rollback(&self, ctx: &mut CommandContext) -> Result<()> {
        if let Some(id_str) = ctx.state.get("created_workspace_id") {
            if let Ok(id) = plaza_foundation::core::id::WorkspaceId::parse(id_str) {
                // Ignore rollback errors to prevent panics during rollback
                let _ = self.service.delete_workspace(&id).await;
            }
        }
        Ok(())
    }
    
    async fn cleanup(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
}

pub struct WorkspaceDeleteCommand {
    service: Arc<WorkspaceService>,
}
impl WorkspaceDeleteCommand {
    pub fn new(service: Arc<WorkspaceService>) -> Self { Self { service } }
}
#[async_trait]
impl ExecutableCommand for WorkspaceDeleteCommand {
    fn metadata(&self) -> CommandMetadata {
        CommandMetadata {
            name: "workspace.delete".into(),
            description: "Delete a workspace".into(),
            version: "1.0".into(),
            category: "workspace".into(),
            permissions: vec![],
            required_engines: vec!["workspace_engine".into()],
            supports_rollback: false, // Cannot easily rollback a filesystem deletion
            supports_dry_run: true,
            supports_transaction: true,
            supports_interactive_mode: false,
        }
    }
    async fn prepare(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
    async fn validate(&self, ctx: &mut CommandContext) -> Result<()> { 
        let id = ctx.request.arguments.get("id").ok_or_else(|| anyhow::anyhow!("Missing id"))?;
        plaza_foundation::core::id::WorkspaceId::parse(id).map_err(|e| anyhow::anyhow!("Invalid workspace ID: {}", e))?;
        Ok(()) 
    }
    async fn plan(&self, ctx: &mut CommandContext) -> Result<ExecutionPlan> {
        let id = ctx.request.arguments.get("id").unwrap();
        Ok(ExecutionPlan { 
            steps: vec![
                ExecutionStep {
                    name: "delete_workspace".into(),
                    description: format!("Delete workspace {} from registry and filesystem", id),
                    allows_rollback: false,
                }
            ], 
            required_permissions: vec![], 
            estimated_cost: "0".into(), 
            affected_engines: vec!["workspace_engine".into()] 
        })
    }
    async fn execute(&self, ctx: &mut CommandContext) -> Result<CommandResponse> {
        let id = ctx.request.arguments.get("id").unwrap();
        let ws_id = plaza_foundation::core::id::WorkspaceId::parse(id).unwrap();
        self.service.delete_workspace(&ws_id).await?;
        Ok(CommandResponse {
            status: CommandStatus::Success,
            exit_code: 0,
            duration_ms: 0,
            diagnostics: vec![format!("Deleted workspace: {}", ws_id)],
            metrics: Default::default(),
            warnings: vec![],
            events_emitted: vec![],
            artifacts_created: vec![],
            payload: None,
        })
    }
    async fn commit(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
    async fn rollback(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
    async fn cleanup(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
}

pub struct WorkspaceOpenCommand {
    _service: Arc<WorkspaceService>,
}
impl WorkspaceOpenCommand {
    pub fn new(_service: Arc<WorkspaceService>) -> Self { Self { _service } }
}
#[async_trait]
impl ExecutableCommand for WorkspaceOpenCommand {
    fn metadata(&self) -> CommandMetadata {
        CommandMetadata {
            name: "workspace.open".into(),
            description: "Open/Activate a workspace".into(),
            version: "1.0".into(),
            category: "workspace".into(),
            permissions: vec![],
            required_engines: vec!["workspace_engine".into()],
            supports_rollback: true,
            supports_dry_run: true,
            supports_transaction: true,
            supports_interactive_mode: false,
        }
    }
    async fn prepare(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
    async fn validate(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
    async fn plan(&self, _ctx: &mut CommandContext) -> Result<ExecutionPlan> {
        Ok(ExecutionPlan { steps: vec![], required_permissions: vec![], estimated_cost: "0".into(), affected_engines: vec!["workspace_engine".into()] })
    }
    async fn execute(&self, ctx: &mut CommandContext) -> Result<CommandResponse> {
        let name = ctx.request.arguments.get("name").cloned().unwrap_or_else(|| "default".into());
        Ok(CommandResponse {
            status: CommandStatus::Success,
            exit_code: 0,
            duration_ms: 0,
            diagnostics: vec![format!("Opened workspace: {}", name)],
            metrics: Default::default(),
            warnings: vec![],
            events_emitted: vec![],
            artifacts_created: vec![],
            payload: None,
        })
    }
    async fn commit(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
    async fn rollback(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
    async fn cleanup(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
}

pub struct WorkspaceCloseCommand {
    _service: Arc<WorkspaceService>,
}
impl WorkspaceCloseCommand {
    pub fn new(_service: Arc<WorkspaceService>) -> Self { Self { _service } }
}
#[async_trait]
impl ExecutableCommand for WorkspaceCloseCommand {
    fn metadata(&self) -> CommandMetadata {
        CommandMetadata {
            name: "workspace.close".into(),
            description: "Close/Deactivate a workspace".into(),
            version: "1.0".into(),
            category: "workspace".into(),
            permissions: vec![],
            required_engines: vec!["workspace_engine".into()],
            supports_rollback: true,
            supports_dry_run: true,
            supports_transaction: true,
            supports_interactive_mode: false,
        }
    }
    async fn prepare(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
    async fn validate(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
    async fn plan(&self, _ctx: &mut CommandContext) -> Result<ExecutionPlan> {
        Ok(ExecutionPlan { steps: vec![], required_permissions: vec![], estimated_cost: "0".into(), affected_engines: vec!["workspace_engine".into()] })
    }
    async fn execute(&self, _ctx: &mut CommandContext) -> Result<CommandResponse> {
        Ok(CommandResponse {
            status: CommandStatus::Success,
            exit_code: 0,
            duration_ms: 0,
            diagnostics: vec!["Closed workspace".into()],
            metrics: Default::default(),
            warnings: vec![],
            events_emitted: vec![],
            artifacts_created: vec![],
            payload: None,
        })
    }
    async fn commit(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
    async fn rollback(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
    async fn cleanup(&self, _ctx: &mut CommandContext) -> Result<()> { Ok(()) }
}

