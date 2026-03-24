use crate::Result;
use async_trait::async_trait;
use serde_json::Value;

#[derive(Debug, Default)]
pub struct ToolContext {}

impl ToolContext {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
pub struct ToolOutput {
    pub content: Value,
}

impl ToolOutput {
    pub fn new(content: Value) -> Self {
        Self { content }
    }
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;

    async fn execute(&self, input: Value, ctx: &ToolContext) -> Result<ToolOutput>;
}
