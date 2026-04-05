use async_trait::async_trait;
use nuro_core::{AgentContext, Result};

#[async_trait]
pub trait RuntimeExecutable: Send + Sync {
    type Input: Send;
    type Output: Send;

    async fn execute(&self, input: Self::Input, ctx: &mut AgentContext) -> Result<Self::Output>;
}

#[async_trait]
pub trait RuntimeMiddleware<I, O>: Send + Sync {
    async fn before_execute(&self, _input: &I, _ctx: &mut AgentContext) -> Result<()> {
        Ok(())
    }

    async fn after_execute(&self, _output: &O, _ctx: &mut AgentContext) -> Result<()> {
        Ok(())
    }

    async fn on_error(&self, _err: &nuro_core::NuroError, _ctx: &mut AgentContext) -> Result<()> {
        Ok(())
    }
}
