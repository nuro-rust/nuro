use crate::message::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub kind: EventKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventKind {
    LlmRequest { messages: Vec<Message> },
    LlmResponse { message: Message },
    ToolCallStart { tool_name: String, input: Value },
    ToolCallEnd { tool_name: String, output: Value },
}
