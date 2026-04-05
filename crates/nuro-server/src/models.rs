use nuro_core::Event;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

fn default_metadata() -> Value {
    json!({})
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCreateRequest {
    pub session_id: Option<String>,
    #[serde(default = "default_metadata")]
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSession {
    pub session_id: String,
    pub metadata: Value,
    pub task_ids: Vec<String>,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(tag = "protocol", rename_all = "snake_case")]
pub enum TaskTarget {
    #[default]
    Agent,
    A2a {
        url: String,
    },
    Mcp {
        tool_name: String,
        arguments: Value,
    },
}

impl TaskTarget {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Agent => "agent",
            Self::A2a { .. } => "a2a",
            Self::Mcp { .. } => "mcp",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeTaskRequest {
    pub session_id: Option<String>,
    pub input: String,
    #[serde(default)]
    pub target: TaskTarget,
    #[serde(default)]
    pub requires_approval: bool,
    #[serde(default)]
    pub start_paused: bool,
    #[serde(default = "default_metadata")]
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    PendingApproval,
    Interrupted,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    pub timestamp_ms: u64,
    pub action: String,
    pub actor: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeTask {
    pub task_id: String,
    pub session_id: String,
    pub run_id: String,
    pub input: String,
    pub target: TaskTarget,
    pub status: TaskStatus,
    pub requires_approval: bool,
    pub approval_reason: Option<String>,
    pub output: Option<String>,
    pub error: Option<String>,
    pub metadata: Value,
    pub token_usage: TokenUsage,
    pub audit_trail: Vec<AuditRecord>,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActionRequest {
    pub actor: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayResponse {
    pub session_id: String,
    pub run_id: Option<String>,
    pub mode: String,
    pub events: Vec<Event>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricsSnapshot {
    pub sessions_total: u64,
    pub tasks_total: u64,
    pub tasks_completed_total: u64,
    pub tasks_failed_total: u64,
    pub tasks_interrupted_current: u64,
    pub tasks_pending_approval_current: u64,
    pub policy_rejections_total: u64,
    pub input_tokens_total: u64,
    pub output_tokens_total: u64,
}
