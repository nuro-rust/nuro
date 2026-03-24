use async_trait::async_trait;
use nuro_core::{NuroError, Result, Tool, ToolContext, ToolOutput};
use serde_json::Value;

/// 简单计算器工具：支持四则运算表达式，依赖 `meval` 做解析与计算
pub struct CalculatorTool;

impl CalculatorTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> &str {
        "calculator"
    }

    fn description(&self) -> &str {
        "Evaluate a mathematical expression, e.g. '1+2*3'"
    }

    async fn execute(&self, input: Value, _ctx: &ToolContext) -> Result<ToolOutput> {
        let expr = input
            .get("expression")
            .and_then(|v| v.as_str())
            .ok_or_else(|| NuroError::InvalidInput("missing 'expression' field".to_string()))?;

        let value = meval::eval_str(expr)
            .map_err(|e| NuroError::Tool(format!("failed to evaluate expression: {e}")))?;

        Ok(ToolOutput::new(serde_json::json!(value)))
    }
}
