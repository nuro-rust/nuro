use crate::{message::Message, Result};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct LlmRequest {
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub message: Message,
}

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse>;
}
