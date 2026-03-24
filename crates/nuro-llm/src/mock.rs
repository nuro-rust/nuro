use async_trait::async_trait;
use nuro_core::{
    message::Message,
    ContentBlock, LlmProvider, LlmRequest, LlmResponse, Result, Role,
};

/// 一个简单的 Mock LLM Provider：
/// - 普通文本输入返回 "Echo: <输入>" 的 Assistant 文本消息
/// - 如果最后一条用户消息以 "calc:" 开头，则返回一个包含 ToolUse 的消息，
///   name 固定为 "calculator"，input = { "expression": "<expr>" }
pub struct MockLlmProvider;

impl MockLlmProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl LlmProvider for MockLlmProvider {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse> {
        // 找到最后一条用户消息
        let user_text = request
            .messages
            .iter()
            .rev()
            .find(|m| matches!(m.role, Role::User))
            .and_then(|m| m.text_content());

        let reply = if let Some(text) = user_text {
            if let Some(expr) = text.strip_prefix("calc:") {
                let expr = expr.trim();
                let tool_input = serde_json::json!({ "expression": expr });
                Message::new(
                    Role::Assistant,
                    vec![ContentBlock::ToolUse {
                        id: "tool_call_1".to_string(),
                        name: "calculator".to_string(),
                        input: tool_input,
                    }],
                )
            } else {
                Message::assistant(format!("Echo: {}", text))
            }
        } else {
            Message::assistant("Echo: (no user input)")
        };

        Ok(LlmResponse { message: reply })
    }
}
