use plaza_command::models::{CommandContext, CommandMetadata, CommandResponse, CommandStatus, ExecutionPlan, ExecutionStep};
use plaza_command::ExecutableCommand;
use anyhow::Result;
use async_trait::async_trait;
use crate::engine::manager::EngineManager;
use std::sync::Arc;

pub struct EngineStartCommand {
    manager: Arc<EngineManager>,
}

impl EngineStartCommand {
    pub fn new(manager: Arc<EngineManager>) -> Self {
        Self { manager }
    }
}

#[async_trait]
impl ExecutableCommand for EngineStartCommand {
    fn metadata(&self) -> CommandMetadata {
        CommandMetadata {
            name: "Engine Start".to_string(),
            description: "Starts all registered engines in dependency order".to_string(),
            version: "1.0.0".to_string(),
            category: "Core".to_string(),
            permissions: vec!["system.admin".to_string()],
            required_engines: vec![],
            supports_rollback: false, // Cannot easily rollback engine start
            supports_dry_run: true,
            supports_transaction: false,
            supports_interactive_mode: false,
        }
    }

    async fn prepare(&self, _ctx: &mut CommandContext) -> Result<()> {
        Ok(())
    }

    async fn validate(&self, _ctx: &mut CommandContext) -> Result<()> {
        // Here we'd check ctx permissions
        Ok(())
    }

    async fn plan(&self, _ctx: &mut CommandContext) -> Result<ExecutionPlan> {
        Ok(ExecutionPlan {
            steps: vec![ExecutionStep {
                name: "Start All Engines".to_string(),
                description: "Invokes initialize() and start() on all engines".to_string(),
                allows_rollback: false,
            }],
            required_permissions: vec!["system.admin".to_string()],
            estimated_cost: "High (IO/Network)".to_string(),
            affected_engines: vec!["*".to_string()],
        })
    }

    async fn execute(&self, _ctx: &mut CommandContext) -> Result<CommandResponse> {
        self.manager.start_all().await?;

        Ok(CommandResponse {
            status: CommandStatus::Success,
            exit_code: 0,
            duration_ms: 0,
            diagnostics: vec!["All engines successfully started".to_string()],
            metrics: Default::default(),
            warnings: vec![],
            events_emitted: vec!["EnginesStarted".to_string()],
            artifacts_created: vec![],
            payload: None,
        })
    }

    async fn commit(&self, _ctx: &mut CommandContext) -> Result<()> {
        Ok(())
    }

    async fn rollback(&self, _ctx: &mut CommandContext) -> Result<()> {
        // Rollback of start is conceptually stop
        self.manager.stop_all().await?;
        Ok(())
    }

    async fn cleanup(&self, _ctx: &mut CommandContext) -> Result<()> {
        Ok(())
    }
}

pub struct EngineStopCommand {
    manager: Arc<EngineManager>,
}

impl EngineStopCommand {
    pub fn new(manager: Arc<EngineManager>) -> Self {
        Self { manager }
    }
}

#[async_trait]
impl ExecutableCommand for EngineStopCommand {
    fn metadata(&self) -> CommandMetadata {
        CommandMetadata {
            name: "Engine Stop".to_string(),
            description: "Stops all registered engines".to_string(),
            version: "1.0.0".to_string(),
            category: "Core".to_string(),
            permissions: vec!["system.admin".to_string()],
            required_engines: vec![],
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
                name: "Stop All Engines".to_string(),
                description: "Invokes stop() on all engines".to_string(),
                allows_rollback: false,
            }],
            required_permissions: vec!["system.admin".to_string()],
            estimated_cost: "Medium".to_string(),
            affected_engines: vec!["*".to_string()],
        })
    }

    async fn execute(&self, _ctx: &mut CommandContext) -> Result<CommandResponse> {
        self.manager.stop_all().await?;

        Ok(CommandResponse {
            status: CommandStatus::Success,
            exit_code: 0,
            duration_ms: 0,
            diagnostics: vec!["All engines successfully stopped".to_string()],
            metrics: Default::default(),
            warnings: vec![],
            events_emitted: vec!["EnginesStopped".to_string()],
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
