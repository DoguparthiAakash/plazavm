use crate::models::{CommandContext, CommandResponse};
use crate::pipeline::CommandPipeline;
use crate::registry::CommandRegistry;
use crate::transaction::TransactionManager;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, instrument};

/// The primary entry point for all command execution.
pub struct CommandDispatcher {
    pub registry: Arc<RwLock<CommandRegistry>>,
    pub pipeline: Arc<CommandPipeline>,
}

impl CommandDispatcher {
    pub fn new(registry: Arc<RwLock<CommandRegistry>>, pipeline: Arc<CommandPipeline>) -> Self {
        Self { registry, pipeline }
    }

    /// Dispatches a CommandRequest through the full lifecycle:
    /// Middleware (Before) -> Validation -> Planning -> Transaction Execution -> Middleware (After)
    #[instrument(skip(self, ctx), fields(command_id = %ctx.request.command_id))]
    pub async fn dispatch(&self, ctx: &mut CommandContext) -> Result<CommandResponse> {
        info!("Dispatching command request: {}", ctx.request.command_id);

        // 1. Resolve Command
        let registry = self.registry.read().await;
        let cmd = registry.resolve(&ctx.request.command_id)?;
        drop(registry); // Release lock

        // 2. Run Before Middleware
        self.pipeline.run_before(ctx).await?;

        // 3. Dry-Run Check handling is now in TransactionManager
        // 4. Transactional Execution
        let exec_result = TransactionManager::execute_transaction(&*cmd, ctx).await;

        match exec_result {
            Ok(response) => {
                let _ = self.pipeline.run_after(ctx, Some(&response), None).await;
                Ok(response)
            }
            Err(e) => {
                let _ = self.pipeline.run_after(ctx, None, Some(&e)).await;
                Err(e)
            }
        }
    }
}

