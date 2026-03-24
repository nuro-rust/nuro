pub mod message;
pub mod event;
pub mod error;
pub mod agent;
pub mod llm;
pub mod tool;

pub use crate::agent::{Agent, AgentContext, AgentInput, AgentOutput};
pub use crate::error::NuroError;
pub use crate::event::{Event, EventKind};
pub use crate::llm::{LlmProvider, LlmRequest, LlmResponse};
pub use crate::message::{ContentBlock, Message, Role, ToolCall};
pub use crate::tool::{Tool, ToolContext, ToolOutput};

pub type Result<T> = std::result::Result<T, NuroError>;

pub mod prelude {
    pub use crate::agent::{Agent, AgentContext, AgentInput, AgentOutput};
    pub use crate::error::NuroError;
    pub use crate::event::{Event, EventKind};
    pub use crate::llm::{LlmProvider, LlmRequest, LlmResponse};
    pub use crate::message::{ContentBlock, Message, Role, ToolCall};
    pub use crate::tool::{Tool, ToolContext, ToolOutput};
    pub use crate::Result;
}
