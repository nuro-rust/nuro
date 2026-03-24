use crate::{message::Message, Result};
use async_trait::async_trait;
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
                if let crate::message::ContentBlock::ToolResult { content, is_error, .. } = block
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
    pub metadata: HashMap<String, Value>,
}

impl AgentContext {
    pub fn new() -> Self {
        Self {
            metadata: HashMap::new(),
        }
    }
}

#[async_trait]
pub trait Agent: Send + Sync {
    async fn invoke(&self, input: AgentInput, ctx: &mut AgentContext) -> Result<AgentOutput>;
}
