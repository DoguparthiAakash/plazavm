use crate::models::{CommandContext, CommandResponse};
use crate::pipeline::Middleware;
use anyhow::Result;
use async_trait::async_trait;
use plaza_foundation::events::{EventBus, PlazaEvent};
use tracing::{error, info};

/// Middleware that publishes command lifecycle events to the PlazaVM event bus.
pub struct EventMiddleware {
    bus: EventBus,
}

impl EventMiddleware {
    pub fn new(bus: EventBus) -> Self {
        Self { bus }
    }
}

#[async_trait]
impl Middleware for EventMiddleware {
    async fn before_execute(&self, ctx: &CommandContext) -> Result<()> {
        self.bus
            .publish(PlazaEvent::CommandReceived {
                command: ctx.request.command_name.clone(),
            })
            .await;

        self.bus
            .publish(PlazaEvent::CommandExecutionStarted {
                command: ctx.request.command_name.clone(),
                target: ctx
                    .request
                    .workspace_id
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "system".to_string()),
            })
            .await;

        Ok(())
    }

    async fn after_execute(
        &self,
        ctx: &CommandContext,
        _response: Option<&CommandResponse>,
        error: Option<&anyhow::Error>,
    ) -> Result<()> {
        let target = ctx
            .request
            .workspace_id
            .as_ref()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "system".to_string());

        if let Some(err) = error {
            self.bus
                .publish(PlazaEvent::CommandExecutionFailed {
                    command: ctx.request.command_name.clone(),
                    target,
                    error: err.to_string(),
                })
                .await;
        } else {
            self.bus
                .publish(PlazaEvent::CommandExecutionCompleted {
                    command: ctx.request.command_name.clone(),
                    target,
                    duration_ms: 0, // In a complete implementation, this would track actual time
                })
                .await;
        }

        Ok(())
    }
}

/// Middleware that tracks detailed execution metrics and tracing.
pub struct ObservabilityMiddleware;

impl ObservabilityMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ObservabilityMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Middleware for ObservabilityMiddleware {
    async fn before_execute(&self, ctx: &CommandContext) -> Result<()> {
        info!(
            "Command execution started: {} (User: {})",
            ctx.request.command_name, ctx.request.user
        );
        Ok(())
    }

    async fn after_execute(
        &self,
        ctx: &CommandContext,
        response: Option<&CommandResponse>,
        error: Option<&anyhow::Error>,
    ) -> Result<()> {
        if let Some(err) = error {
            error!(
                "Command execution failed: {} - {}",
                ctx.request.command_name, err
            );
        } else if let Some(res) = response {
            info!(
                "Command execution completed: {} -> Status: {:?}",
                ctx.request.command_name, res.status
            );
        }
        Ok(())
    }
}

