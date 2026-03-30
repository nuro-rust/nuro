use async_trait::async_trait;
use nuro_core::{
    ContentBlock, Event, EventKind, LlmProvider, LlmRequest, Message, NuroError, Result,
};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderCapability {
    TextGeneration,
    Streaming,
    ToolCall,
}

#[derive(Debug, Clone)]
pub struct ProviderDescriptor {
    pub name: String,
    pub capabilities: Vec<ProviderCapability>,
}

#[derive(Debug, Clone)]
pub enum ProviderStreamEvent {
    TextDelta(String),
    ToolCallStart { name: String, input: Value },
    ToolCallEnd { name: String, output: Value },
    Done,
}

#[async_trait]
pub trait LlmProviderAdapter: LlmProvider + Send + Sync {
    fn descriptor(&self) -> ProviderDescriptor;

    async fn stream_generate(&self, request: LlmRequest) -> Result<Vec<ProviderStreamEvent>>;

    fn stream_events_to_runtime_events(
        &self,
        session_id: Option<String>,
        run_id: Option<String>,
        stream_events: &[ProviderStreamEvent],
    ) -> Result<Vec<Event>> {
        let mut events = Vec::new();

        for item in stream_events {
            let event = match item {
                ProviderStreamEvent::TextDelta(delta) => Event::new(EventKind::LlmResponse {
                    message: Message::new(
                        nuro_core::Role::Assistant,
                        vec![ContentBlock::Text(delta.clone())],
                    ),
                })
                .with_session(session_id.clone(), run_id.clone()),
                ProviderStreamEvent::ToolCallStart { name, input } => {
                    Event::new(EventKind::ToolCallStart {
                        tool_name: name.clone(),
                        input: input.clone(),
                    })
                    .with_session(session_id.clone(), run_id.clone())
                }
                ProviderStreamEvent::ToolCallEnd { name, output } => {
                    Event::new(EventKind::ToolCallEnd {
                        tool_name: name.clone(),
                        output: output.clone(),
                    })
                    .with_session(session_id.clone(), run_id.clone())
                }
                ProviderStreamEvent::Done => continue,
            };
            events.push(event);
        }

        if events.is_empty() {
            return Err(NuroError::InvalidInput(
                "stream_events_to_runtime_events received empty stream events".to_string(),
            ));
        }

        Ok(events)
    }
}
