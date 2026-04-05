use crate::{Result, message::Message};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum AgentInput {
    Text(String),
    Messages(Vec<Message>),
}

impl AgentInput {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(text.into())
    }

    pub fn messages(messages: Vec<Message>) -> Self {
        Self::Messages(messages)
    }
}

#[derive(Debug, Clone)]
pub struct AgentOutput {
    pub messages: Vec<Message>,
}

impl AgentOutput {
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }

    pub fn last_message(&self) -> Option<&Message> {
        self.messages.last()
    }

    /// 一个便捷方法：优先返回最后一条消息的文本内容；若没有，则回退到最近的 ToolResult
    pub fn text(&self) -> Option<String> {
        if let Some(last) = self.last_message() {
            if let Some(text) = last.text_content() {
                return Some(text);
            }
        }

        // 回退：找最近的非错误 ToolResult
        for msg in self.messages.iter().rev() {
            for block in &msg.content {
                if let crate::message::ContentBlock::ToolResult {
                    content, is_error, ..
                } = block
                {
                    if !is_error {
                        return Some(content.to_string());
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct AgentContext {
    pub session: Option<SessionContext>,
    pub metadata: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub session_id: String,
    pub run_id: Option<String>,
    pub resume_token: Option<String>,
    pub metadata: HashMap<String, Value>,
}

impl SessionContext {
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            run_id: None,
            resume_token: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_run_id(mut self, run_id: impl Into<String>) -> Self {
        self.run_id = Some(run_id.into());
        self
    }

    pub fn with_resume_token(mut self, token: impl Into<String>) -> Self {
        self.resume_token = Some(token.into());
        self
    }
}

impl AgentContext {
    pub fn new() -> Self {
        Self {
            session: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_session(mut self, session: SessionContext) -> Self {
        self.session = Some(session);
        self
    }
}

#[async_trait]
pub trait Agent: Send + Sync {
    async fn invoke(&self, input: AgentInput, ctx: &mut AgentContext) -> Result<AgentOutput>;
}
