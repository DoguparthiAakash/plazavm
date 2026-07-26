use crate::models::{CommandContext, CommandResponse, CommandStatus, ExecutionMode, ExecutionPlan};
use crate::traits::ExecutableCommand;
use anyhow::{anyhow, Result};
use tracing::{error, info, warn};

/// Manages transactional execution of commands with automatic rollback.
pub struct TransactionManager;

impl TransactionManager {
    /// Executes a command transactionally.
    pub async fn execute_transaction(
        cmd: &(dyn ExecutableCommand + Send + Sync),
        ctx: &mut CommandContext,
    ) -> Result<CommandResponse> {
        let cmd_name = cmd.metadata().name;
        info!("Executing transaction for command: {}", cmd_name);

        // 1. Prepare
        if let Err(e) = cmd.prepare(ctx).await {
            error!("Command {} failed during preparation: {}", cmd_name, e);
            let _ = cmd.cleanup(ctx).await;
            return Err(anyhow!("Command preparation failed: {}", e));
        }

        // 2. Validate
        if let Err(e) = cmd.validate(ctx).await {
            error!("Command {} failed validation: {}", cmd_name, e);
            let _ = cmd.cleanup(ctx).await;
            return Err(anyhow!("Command validation failed: {}", e));
        }

        // 3. Plan
        let plan = match cmd.plan(ctx).await {
            Ok(p) => p,
            Err(e) => {
                error!("Command {} failed during planning: {}", cmd_name, e);
                let _ = cmd.cleanup(ctx).await;
                return Err(anyhow!("Command planning failed: {}", e));
            }
        };

        // 4. Dry Run Check
        if ctx.request.execution_mode == ExecutionMode::DryRun {
            let payload = serde_json::to_string_pretty(&plan)?;
            let response = CommandResponse {
                status: CommandStatus::Success,
                exit_code: 0,
                duration_ms: 0,
                diagnostics: vec!["Dry run completed successfully".to_string()],
                metrics: Default::default(),
                warnings: Vec::new(),
                events_emitted: Vec::new(),
                artifacts_created: Vec::new(),
                payload: Some(payload),
            };
            let _ = cmd.cleanup(ctx).await;
            return Ok(response);
        }

        // 5. Execute
        let execution_result = cmd.execute(ctx).await;

        match execution_result {
            Ok(response) => {
                // 6. Commit
                if let Err(commit_err) = cmd.commit(ctx).await {
                    error!("Command {} failed during commit: {}", cmd_name, commit_err);
                    // Depending on policy, we might rollback here. For now we rollback.
                    if cmd.metadata().supports_rollback {
                        if let Err(rollback_err) = cmd.rollback(ctx).await {
                            error!("CRITICAL: Rollback after failed commit failed for {}: {}", cmd_name, rollback_err);
                        } else {
                            warn!("Rollback completed successfully after failed commit for {}", cmd_name);
                        }
                    }
                    let _ = cmd.cleanup(ctx).await;
                    return Err(anyhow!("Command commit failed: {}", commit_err));
                }

                info!("Command {} executed and committed successfully", cmd_name);
                let _ = cmd.cleanup(ctx).await;
                Ok(response)
            }
            Err(execution_err) => {
                error!("Command {} failed during execution: {}. Initiating rollback...", cmd_name, execution_err);

                // 7. Rollback
                if cmd.metadata().supports_rollback {
                    if let Err(rollback_err) = cmd.rollback(ctx).await {
                        error!("CRITICAL: Rollback failed for command {}: {}", cmd_name, rollback_err);
                        let _ = cmd.cleanup(ctx).await;
                        return Err(anyhow!("Command execution failed ({}), AND rollback failed ({})", execution_err, rollback_err));
                    }
                    warn!("Rollback completed successfully for command {}", cmd_name);
                } else {
                    warn!("Command {} does not support rollback. State may be inconsistent.", cmd_name);
                }

                // 8. Cleanup
                let _ = cmd.cleanup(ctx).await;

                // Bubble up the original execution error
                Err(execution_err)
            }
        }
    }
}
