use crate::models::{CommandContext, CommandMetadata, CommandResponse, ExecutionPlan};
use anyhow::Result;
use async_trait::async_trait;

/// The central trait for all executable operations across PlazaVM.
#[async_trait]
pub trait ExecutableCommand: Send + Sync {
    /// Get static metadata about this command.
    fn metadata(&self) -> CommandMetadata;

    /// Prepare the command before execution. This can involve fetching resources or allocating state.
    async fn prepare(&self, ctx: &mut CommandContext) -> Result<()>;

    /// Validate the command against the context, ensuring required permissions
    /// and correct arguments before planning or executing.
    async fn validate(&self, ctx: &mut CommandContext) -> Result<()>;

    /// Plan the execution steps, estimating cost and generating an artifact map.
    /// In a dry run, execution stops here and returns the plan.
    async fn plan(&self, ctx: &mut CommandContext) -> Result<ExecutionPlan>;

    /// Execute the actual command operation. This must produce observable work.
    async fn execute(&self, ctx: &mut CommandContext) -> Result<CommandResponse>;

    /// Commit the execution, finalizing state (e.g., confirming transactions in DB).
    async fn commit(&self, ctx: &mut CommandContext) -> Result<()>;

    /// Revert any changes made by execute().
    async fn rollback(&self, ctx: &mut CommandContext) -> Result<()>;

    /// Cleanup any temporary resources regardless of success or failure.
    async fn cleanup(&self, ctx: &mut CommandContext) -> Result<()>;
}
