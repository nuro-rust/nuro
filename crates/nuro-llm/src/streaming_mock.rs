use async_trait::async_trait;
use nuro_core::{
    ContentBlock, LlmProvider, LlmRequest, LlmResponse, Result, Role, message::Message,
};

use crate::provider_adapter::{
    LlmProviderAdapter, ProviderCapability, ProviderDescriptor, ProviderStreamEvent,
};

pub struct StreamingMockLlmProvider {
    chunk_size: usize,
}

impl StreamingMockLlmProvider {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            chunk_size: chunk_size.max(1),
        }
    }
}

#[async_trait]
impl LlmProvider for StreamingMockLlmProvider {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse> {
        let user_text = request
            .messages
            .iter()
            .rev()
            .find(|m| matches!(m.role, Role::User))
            .and_then(|m| m.text_content())
            .unwrap_or_else(|| "(no user input)".to_string());

        Ok(LlmResponse {
            message: Message::assistant(format!("StreamEcho: {user_text}")),
        })
    }
}

#[async_trait]
impl LlmProviderAdapter for StreamingMockLlmProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor {
            name: "streaming-mock".to_string(),
            capabilities: vec![
                ProviderCapability::TextGeneration,
                ProviderCapability::Streaming,
                ProviderCapability::ToolCall,
            ],
        }
    }

    async fn stream_generate(&self, request: LlmRequest) -> Result<Vec<ProviderStreamEvent>> {
        let response = self.generate(request).await?;
        let mut events = Vec::new();

        for block in &response.message.content {
            if let ContentBlock::Text(text) = block {
                let chars: Vec<char> = text.chars().collect();
                let mut start = 0usize;
                while start < chars.len() {
                    let end = (start + self.chunk_size).min(chars.len());
                    let chunk: String = chars[start..end].iter().collect();
                    events.push(ProviderStreamEvent::TextDelta(chunk));
                    start = end;
                }
            }
        }

        events.push(ProviderStreamEvent::Done);
        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use nuro_core::{LlmRequest, Message};

    use crate::{LlmProviderAdapter, StreamingMockLlmProvider};

    #[tokio::test]
    async fn streaming_mock_emits_multiple_deltas() {
        let provider = StreamingMockLlmProvider::new(3);
        let req = LlmRequest {
            messages: vec![Message::user("hello streaming")],
        };

        let events = provider.stream_generate(req).await.expect("stream ok");
        assert!(events.len() > 2);
    }
}
