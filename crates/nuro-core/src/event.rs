use crate::message::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static EVENT_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_id: String,
    pub schema_version: String,
    pub timestamp_ms: u64,
    pub session_id: Option<String>,
    pub run_id: Option<String>,
    pub correlation_id: Option<String>,
    pub causation_id: Option<String>,
    pub metadata: Value,
    pub kind: EventKind,
}

impl Event {
    pub fn new(kind: EventKind) -> Self {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        let seq = EVENT_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            event_id: format!("evt-{ts}-{seq}"),
            schema_version: "v1".to_string(),
            timestamp_ms: ts,
            session_id: None,
            run_id: None,
            correlation_id: None,
            causation_id: None,
            metadata: Value::Object(Default::default()),
            kind,
        }
    }

    pub fn with_session(mut self, session_id: Option<String>, run_id: Option<String>) -> Self {
        self.session_id = session_id;
        self.run_id = run_id;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventKind {
    LlmRequest { messages: Vec<Message> },
    LlmResponse { message: Message },
    ToolCallStart { tool_name: String, input: Value },
    ToolCallEnd { tool_name: String, output: Value },
}
