use crate::models::{CommandContext, CommandResponse};
use anyhow::Result;
use async_trait::async_trait;

/// Middleware for intercepting command execution.
#[async_trait]
pub trait Middleware: Send + Sync {
    /// Execute logic before the command runs. 
    /// If an error is returned, execution halts.
    async fn before_execute(&self, ctx: &CommandContext) -> Result<()>;

    /// Execute logic after the command runs (whether successful or not).
    async fn after_execute(
        &self,
        ctx: &CommandContext,
        response: Option<&CommandResponse>,
        error: Option<&anyhow::Error>,
    ) -> Result<()>;
}

/// The command execution pipeline runner.
pub struct CommandPipeline {
    middlewares: Vec<Box<dyn Middleware>>,
}

impl CommandPipeline {
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }

    pub fn add_middleware(&mut self, middleware: Box<dyn Middleware>) {
        self.middlewares.push(middleware);
    }

    /// Run the "before" phase of all middlewares sequentially.
    pub async fn run_before(&self, ctx: &CommandContext) -> Result<()> {
        for m in &self.middlewares {
            m.before_execute(ctx).await?;
        }
        Ok(())
    }

    /// Run the "after" phase of all middlewares in reverse order.
    pub async fn run_after(
        &self,
        ctx: &CommandContext,
        response: Option<&CommandResponse>,
        error: Option<&anyhow::Error>,
    ) -> Result<()> {
        for m in self.middlewares.iter().rev() {
            m.after_execute(ctx, response, error).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, Ordering};
    use std::sync::Arc;

    struct TestMiddleware {
        state: Arc<AtomicI32>,
        increment: i32,
    }

    #[async_trait]
    impl Middleware for TestMiddleware {
        async fn before_execute(&self, _ctx: &CommandContext) -> Result<()> {
            self.state.fetch_add(self.increment, Ordering::SeqCst);
            Ok(())
        }

        async fn after_execute(
            &self,
            _ctx: &CommandContext,
            _response: Option<&CommandResponse>,
            _error: Option<&anyhow::Error>,
        ) -> Result<()> {
            self.state.fetch_sub(self.increment, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_pipeline_execution_order() {
        let state = Arc::new(AtomicI32::new(0));
        let mut pipeline = CommandPipeline::new();

        pipeline.add_middleware(Box::new(TestMiddleware {
            state: state.clone(),
            increment: 5,
        }));

        pipeline.add_middleware(Box::new(TestMiddleware {
            state: state.clone(),
            increment: 10,
        }));

        let ctx = CommandContext {
            request: crate::models::CommandRequest {
                command_id: "test".to_string(),
                command_name: "test".to_string(),
                arguments: std::collections::HashMap::new(),
                workspace_id: None,
                runtime_id: None,
                user: "test".to_string(),
                permissions: vec![],
                execution_mode: crate::models::ExecutionMode::Normal,
                output_format: "text".to_string(),
                metadata: std::collections::HashMap::new(),
            },
        };

        // Run before
        pipeline.run_before(&ctx).await.unwrap();
        assert_eq!(state.load(Ordering::SeqCst), 15);

        // Run after
        pipeline.run_after(&ctx, None, None).await.unwrap();
        assert_eq!(state.load(Ordering::SeqCst), 0);
    }
}

