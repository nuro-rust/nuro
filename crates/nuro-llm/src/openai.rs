//! OpenAI LLM Provider 实现（在 `openai` feature 下启用）。
//!
//! 这是一个非常精简的封装，仅调用 `/v1/chat/completions` 接口：
//! - 仅支持纯文本对话，不处理函数调用 / tool call；
//! - 使用环境变量 `OPENAI_API_KEY` 作为鉴权；
//! - 可通过 `OPENAI_BASE_URL` 覆盖默认地址 `https://api.openai.com/v1`；
//! - 默认模型为 `gpt-4o-mini`，也可在 `new_with_model` 中自定义。
//!
//! 该实现的目标是提供一个“可用但最小”的真实 LLM Provider，
//! 保持 API 形状简单，后续可以在不破坏现有调用的前提下扩展配置项
//! 与流式调用能力。

#[cfg(feature = "openai")]
use async_trait::async_trait;
#[cfg(feature = "openai")]
use nuro_core::{
    message::{ContentBlock, Message, Role},
    LlmProvider, LlmRequest, LlmResponse, NuroError, Result,
};

#[cfg(feature = "openai")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "openai")]
use reqwest::Client;

/// 一个最小可用的 OpenAI LLM Provider 实现。
///
/// - 仅实现 `LlmProvider::generate`；
/// - 使用 `chat/completions` 接口；
/// - 不支持流式增量返回（但上层可以基于最终结果做简单“伪流式”拆分）。
#[cfg(feature = "openai")]
#[derive(Clone)]
pub struct OpenAiLlmProvider {
    client: Client,
    api_key: String,
    base_url: String,
    model: String,
}

#[cfg(feature = "openai")]
impl OpenAiLlmProvider {
    /// 默认使用 `gpt-4o-mini` 模型构造 Provider。
    pub fn new() -> Result<Self> {
        Self::new_with_model("gpt-4o-mini")
    }

    /// 使用自定义模型构造 Provider。
    pub fn new_with_model(model: impl Into<String>) -> Result<Self> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| NuroError::Llm("OPENAI_API_KEY is not set".to_string()))?;

        let base_url = std::env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

        Ok(Self {
            client: Client::new(),
            api_key,
            base_url,
            model: model.into(),
        })
    }
}

#[cfg(feature = "openai")]
#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[cfg(feature = "openai")]
#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[cfg(feature = "openai")]
#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatCompletionChoice>,
}

#[cfg(feature = "openai")]
#[derive(Debug, Deserialize)]
struct ChatCompletionChoice {
    message: ChatCompletionMessage,
}

#[cfg(feature = "openai")]
#[derive(Debug, Deserialize)]
struct ChatCompletionMessage {
    role: String,
    content: String,
}

#[cfg(feature = "openai")]
fn to_openai_role(role: &Role) -> &'static str {
    match role {
        Role::System => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    }
}

#[cfg(feature = "openai")]
fn flatten_message_content(msg: &Message) -> String {
    // 仅拼接 Text 内容块，其余类型忽略。
    msg.content
        .iter()
        .filter_map(|c| match c {
            ContentBlock::Text(t) => Some(t.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(feature = "openai")]
#[async_trait]
impl LlmProvider for OpenAiLlmProvider {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse> {
        let messages: Vec<ChatMessage> = request
            .messages
            .iter()
            .map(|m| ChatMessage {
                role: to_openai_role(&m.role).to_string(),
                content: flatten_message_content(m),
            })
            .collect();

        if messages.is_empty() {
            return Err(NuroError::Llm(
                "OpenAI provider received empty message list".to_string(),
            ));
        }

        let body = ChatCompletionRequest {
            model: self.model.clone(),
            messages,
        };

        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));

        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| NuroError::Llm(format!("OpenAI HTTP error: {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp
                .text()
                .await
                .unwrap_or_else(|_| "<failed to read error body>".to_string());
            return Err(NuroError::Llm(format!(
                "OpenAI API error: status = {status}, body = {text}"
            )));
        }

        let parsed: ChatCompletionResponse = resp
            .json()
            .await
            .map_err(|e| NuroError::Llm(format!("failed to parse OpenAI response: {e}")))?;

        let choice = parsed
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| NuroError::Llm("OpenAI response has no choices".to_string()))?;

        let reply = Message::assistant(choice.message.content);

        Ok(LlmResponse { message: reply })
    }
}
