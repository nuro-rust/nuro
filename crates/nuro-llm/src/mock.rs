use async_trait::async_trait;
use nuro_core::{
    ContentBlock, LlmProvider, LlmRequest, LlmResponse, Result, Role, message::Message,
};

use crate::provider_adapter::{
    LlmProviderAdapter, ProviderCapability, ProviderDescriptor, ProviderStreamEvent,
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

#[async_trait]
impl LlmProviderAdapter for MockLlmProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            name: "mock".to_string(),
            capabilities: vec![
                ProviderCapability::TextGeneration,
                ProviderCapability::Streaming,
                ProviderCapability::ToolCall,
            ],
        }
    }

    async fn stream_generate(&self, request: LlmRequest) -> Result<Vec<ProviderStreamEvent>> {
        let response = self.generate(request).await?;
        let mut out = Vec::new();

        for block in &response.message.content {
            match block {
                ContentBlock::Text(text) => {
                    for part in text.split_whitespace() {
                        out.push(ProviderStreamEvent::TextDelta(format!("{} ", part)));
                    }
                }
                ContentBlock::ToolUse { name, input, .. } => {
                    out.push(ProviderStreamEvent::ToolCallStart {
                        name: name.clone(),
                        input: input.clone(),
                    });
                    out.push(ProviderStreamEvent::ToolCallEnd {
                        name: name.clone(),
                        output: serde_json::json!({"status":"emitted_by_mock"}),
                    });
                }
                _ => {}
            }
        }

        out.push(ProviderStreamEvent::Done);
        Ok(out)
    }
}
