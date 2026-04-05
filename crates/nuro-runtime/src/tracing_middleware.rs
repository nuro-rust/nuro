use async_trait::async_trait;
use nuro_core::{AgentContext, AgentInput, AgentOutput, NuroError, Result};
use tracing::{error, info};

use crate::RuntimeMiddleware;

pub struct TracingMiddleware;

impl TracingMiddleware {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl RuntimeMiddleware<AgentInput, AgentOutput> for TracingMiddleware {
    async fn before_execute(&self, _input: &AgentInput, ctx: &mut AgentContext) -> Result<()> {
        let session_id = ctx
            .session
            .as_ref()
            .map(|s| s.session_id.as_str())
            .unwrap_or("-");
        let run_id = ctx
            .session
            .as_ref()
            .and_then(|s| s.run_id.as_deref())
            .unwrap_or("-");
        info!(session_id, run_id, "agent loop invoke started");
        Ok(())
    }

    async fn after_execute(&self, output: &AgentOutput, _ctx: &mut AgentContext) -> Result<()> {
        info!(
            message_count = output.messages.len(),
            "agent loop invoke finished"
        );
        Ok(())
    }

    async fn on_error(&self, err: &NuroError, _ctx: &mut AgentContext) -> Result<()> {
        error!(error = %err, "agent loop invoke failed");
        Ok(())
    }
}
