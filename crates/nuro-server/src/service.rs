use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU64, Ordering},
};

use nuro_a2a::A2aClient;
use nuro_core::{AgentContext, AgentInput, Event, EventKind, NuroError, SessionContext};
use nuro_mcp::{McpClient, McpServer};
use nuro_runtime::{AgentLoop, CheckpointStore, EventStore, ReplayEngine, ReplayMode};
use nuro_tools::CalculatorTool;
use serde_json::{Value, json};
use tokio::io::{BufReader, duplex};
use tokio_stream::StreamExt;
use tracing::{error, info, warn};

use crate::{
    models::{
        ActionRequest, AuditRecord, MetricsSnapshot, ReplayResponse, RuntimeSession, RuntimeTask,
        RuntimeTaskRequest, TaskStatus, TaskTarget, TokenUsage,
    },
    policy::{PolicyDecision, PolicyEngine},
};

pub type ServiceResult<T> = std::result::Result<T, ServerError>;

#[derive(Debug, Clone)]
pub struct ServerError {
    status: u16,
    message: String,
}

impl ServerError {
    pub fn new(status: u16, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(400, message)
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(403, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(404, message)
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(409, message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(500, message)
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<NuroError> for ServerError {
    fn from(value: NuroError) -> Self {
        match value {
            NuroError::InvalidInput(message) => Self::bad_request(message),
            NuroError::Storage(message) => Self::internal(message),
            NuroError::Tool(message)
            | NuroError::Llm(message)
            | NuroError::ToolNotFound(message) => Self::internal(message),
        }
    }
}

pub struct RuntimeService {
    agent: AgentLoop,
    event_store: Arc<dyn EventStore>,
    checkpoint_store: Arc<dyn CheckpointStore>,
    policy_engine: PolicyEngine,
    sessions: Mutex<std::collections::HashMap<String, RuntimeSession>>,
    tasks: Mutex<std::collections::HashMap<String, RuntimeTask>>,
    next_session_id: AtomicU64,
    next_task_id: AtomicU64,
    next_run_id: AtomicU64,
    policy_rejections: AtomicU64,
}

struct TaskExecutionOutcome {
    output: String,
    output_tokens: u64,
}

impl RuntimeService {
    pub fn new(
        agent: AgentLoop,
        event_store: Arc<dyn EventStore>,
        checkpoint_store: Arc<dyn CheckpointStore>,
        policy_engine: PolicyEngine,
    ) -> Self {
        Self {
            agent,
            event_store,
            checkpoint_store,
            policy_engine,
            sessions: Mutex::new(std::collections::HashMap::new()),
            tasks: Mutex::new(std::collections::HashMap::new()),
            next_session_id: AtomicU64::new(1),
            next_task_id: AtomicU64::new(1),
            next_run_id: AtomicU64::new(1),
            policy_rejections: AtomicU64::new(0),
        }
    }

    pub fn create_session(
        &self,
        session_id: Option<String>,
        metadata: Value,
    ) -> ServiceResult<RuntimeSession> {
        let now = now_ms();
        let session_id = session_id.unwrap_or_else(|| {
            format!(
                "session-{}",
                self.next_session_id.fetch_add(1, Ordering::Relaxed)
            )
        });

        let mut sessions = self.sessions.lock().expect("sessions lock");
        let entry = sessions
            .entry(session_id.clone())
            .or_insert_with(|| RuntimeSession {
                session_id: session_id.clone(),
                metadata: metadata.clone(),
                task_ids: Vec::new(),
                created_at_ms: now,
                updated_at_ms: now,
            });

        if has_metadata(&metadata) {
            entry.metadata = metadata;
        }
        entry.updated_at_ms = now;
        Ok(entry.clone())
    }

    pub fn get_session(&self, session_id: &str) -> Option<RuntimeSession> {
        self.sessions
            .lock()
            .expect("sessions lock")
            .get(session_id)
            .cloned()
    }

    pub fn get_task(&self, task_id: &str) -> Option<RuntimeTask> {
        self.tasks.lock().expect("tasks lock").get(task_id).cloned()
    }

    pub async fn submit_task(&self, request: RuntimeTaskRequest) -> ServiceResult<RuntimeTask> {
        self.validate_request(&request)?;

        let decision = self.policy_engine.evaluate(&request.input);
        if let PolicyDecision::Deny { reason, .. } = &decision {
            self.policy_rejections.fetch_add(1, Ordering::Relaxed);
            warn!(reason = %reason, "runtime task rejected by policy");
            return Err(ServerError::forbidden(reason.clone()));
        }

        let session = self.create_session(request.session_id.clone(), request.metadata.clone())?;
        let now = now_ms();
        let task_id = format!("task-{}", self.next_task_id.fetch_add(1, Ordering::Relaxed));
        let run_id = format!("run-{}", self.next_run_id.fetch_add(1, Ordering::Relaxed));
        let mut task = RuntimeTask {
            task_id: task_id.clone(),
            session_id: session.session_id.clone(),
            run_id,
            input: request.input.clone(),
            target: request.target.clone(),
            status: TaskStatus::Running,
            requires_approval: request.requires_approval,
            approval_reason: None,
            output: None,
            error: None,
            metadata: request.metadata.clone(),
            token_usage: TokenUsage {
                input_tokens: estimate_tokens(&request.input),
                output_tokens: 0,
            },
            audit_trail: Vec::new(),
            created_at_ms: now,
            updated_at_ms: now,
        };

        let target_label = task.target.label().to_string();
        append_audit(
            &mut task,
            "task.created",
            "system",
            format!("protocol={target_label}"),
        );

        if let PolicyDecision::RequireApproval { reason, .. } = decision {
            task.requires_approval = true;
            task.approval_reason = Some(reason.clone());
            task.status = TaskStatus::PendingApproval;
            append_audit(&mut task, "task.awaiting_approval", "policy", reason);
            return self.store_task(task);
        }

        if request.requires_approval {
            task.status = TaskStatus::PendingApproval;
            task.approval_reason = Some("manual approval requested".to_string());
            append_audit(
                &mut task,
                "task.awaiting_approval",
                "user",
                "request marked as approval-gated",
            );
            return self.store_task(task);
        }

        if request.start_paused {
            task.status = TaskStatus::Interrupted;
            append_audit(
                &mut task,
                "task.interrupted",
                "system",
                "task created in paused state",
            );
            return self.store_task(task);
        }

        self.store_task(task)?;
        self.execute_task(&task_id, "system", "task.executed", None)
            .await
    }

    pub async fn interrupt_task(
        &self,
        task_id: &str,
        request: ActionRequest,
    ) -> ServiceResult<RuntimeTask> {
        let mut task = self.require_task(task_id)?;
        if matches!(task.status, TaskStatus::Completed | TaskStatus::Failed) {
            return Err(ServerError::conflict("cannot interrupt a completed task"));
        }
        if matches!(task.status, TaskStatus::Running) {
            return Err(ServerError::conflict(
                "running tasks are synchronous; create them paused or approval-gated to resume later",
            ));
        }

        task.status = TaskStatus::Interrupted;
        task.updated_at_ms = now_ms();
        append_audit(
            &mut task,
            "task.interrupted",
            &actor_or_system(&request.actor),
            request
                .comment
                .unwrap_or_else(|| "task interrupted before execution".to_string()),
        );
        self.store_task(task)
    }

    pub async fn approve_task(
        &self,
        task_id: &str,
        request: ActionRequest,
    ) -> ServiceResult<RuntimeTask> {
        let task = self.require_task(task_id)?;
        if task.status != TaskStatus::PendingApproval {
            return Err(ServerError::conflict("task is not awaiting approval"));
        }

        self.execute_task(
            task_id,
            &actor_or_system(&request.actor),
            "task.approved",
            request.comment,
        )
        .await
    }

    pub async fn resume_task(
        &self,
        task_id: &str,
        request: ActionRequest,
    ) -> ServiceResult<RuntimeTask> {
        let task = self.require_task(task_id)?;
        if task.status != TaskStatus::Interrupted {
            return Err(ServerError::conflict("task is not interrupted"));
        }

        self.execute_task(
            task_id,
            &actor_or_system(&request.actor),
            "task.resumed",
            request.comment,
        )
        .await
    }

    pub fn replay_session(
        &self,
        session_id: &str,
        run_id: Option<&str>,
        mode: ReplayMode,
    ) -> ServiceResult<ReplayResponse> {
        let engine = ReplayEngine::new(self.event_store.clone());
        let result = match run_id {
            Some(run_id) => engine.replay_session_run(session_id, run_id, mode.clone()),
            None => engine.replay_session(session_id, mode.clone()),
        }
        .map_err(ServerError::from)?;

        if result.events.is_empty() && self.get_session(session_id).is_none() {
            return Err(ServerError::not_found(format!(
                "session '{}' was not found",
                session_id
            )));
        }

        Ok(ReplayResponse {
            session_id: session_id.to_string(),
            run_id: run_id.map(str::to_string),
            mode: replay_mode_name(&mode).to_string(),
            events: result.events,
            warnings: result.warnings,
        })
    }

    pub fn metrics_snapshot(&self) -> MetricsSnapshot {
        let sessions = self.sessions.lock().expect("sessions lock");
        let tasks = self.tasks.lock().expect("tasks lock");

        let mut snapshot = MetricsSnapshot {
            sessions_total: sessions.len() as u64,
            tasks_total: tasks.len() as u64,
            policy_rejections_total: self.policy_rejections.load(Ordering::Relaxed),
            ..MetricsSnapshot::default()
        };

        for task in tasks.values() {
            snapshot.input_tokens_total += task.token_usage.input_tokens;
            snapshot.output_tokens_total += task.token_usage.output_tokens;
            match task.status {
                TaskStatus::Completed => snapshot.tasks_completed_total += 1,
                TaskStatus::Failed => snapshot.tasks_failed_total += 1,
                TaskStatus::Interrupted => snapshot.tasks_interrupted_current += 1,
                TaskStatus::PendingApproval => snapshot.tasks_pending_approval_current += 1,
                TaskStatus::Running => {}
            }
        }

        snapshot
    }

    pub fn metrics_prometheus(&self) -> String {
        let metrics = self.metrics_snapshot();
        format!(
            concat!(
                "# TYPE nuro_sessions_total gauge\n",
                "nuro_sessions_total {}\n",
                "# TYPE nuro_tasks_total gauge\n",
                "nuro_tasks_total {}\n",
                "# TYPE nuro_tasks_completed_total counter\n",
                "nuro_tasks_completed_total {}\n",
                "# TYPE nuro_tasks_failed_total counter\n",
                "nuro_tasks_failed_total {}\n",
                "# TYPE nuro_tasks_interrupted_current gauge\n",
                "nuro_tasks_interrupted_current {}\n",
                "# TYPE nuro_tasks_pending_approval_current gauge\n",
                "nuro_tasks_pending_approval_current {}\n",
                "# TYPE nuro_policy_rejections_total counter\n",
                "nuro_policy_rejections_total {}\n",
                "# TYPE nuro_input_tokens_total counter\n",
                "nuro_input_tokens_total {}\n",
                "# TYPE nuro_output_tokens_total counter\n",
                "nuro_output_tokens_total {}\n"
            ),
            metrics.sessions_total,
            metrics.tasks_total,
            metrics.tasks_completed_total,
            metrics.tasks_failed_total,
            metrics.tasks_interrupted_current,
            metrics.tasks_pending_approval_current,
            metrics.policy_rejections_total,
            metrics.input_tokens_total,
            metrics.output_tokens_total,
        )
    }

    async fn execute_task(
        &self,
        task_id: &str,
        actor: &str,
        action: &str,
        detail: Option<String>,
    ) -> ServiceResult<RuntimeTask> {
        let mut task = self.require_task(task_id)?;
        task.status = TaskStatus::Running;
        task.error = None;
        task.updated_at_ms = now_ms();
        append_audit(
            &mut task,
            action,
            actor,
            detail.unwrap_or_else(|| "task execution started".to_string()),
        );
        self.store_task(task.clone())?;

        info!(
            session_id = %task.session_id,
            task_id = %task.task_id,
            run_id = %task.run_id,
            protocol = task.target.label(),
            input_tokens = task.token_usage.input_tokens,
            "runtime task started",
        );

        match self.run_target(&task).await {
            Ok(outcome) => {
                let mut finished = self.require_task(task_id)?;
                finished.status = TaskStatus::Completed;
                finished.output = Some(outcome.output.clone());
                finished.error = None;
                finished.token_usage.output_tokens = outcome.output_tokens;
                finished.updated_at_ms = now_ms();
                append_audit(
                    &mut finished,
                    "task.completed",
                    "runtime",
                    format!("output_tokens={}", outcome.output_tokens),
                );
                info!(
                    session_id = %finished.session_id,
                    task_id = %finished.task_id,
                    run_id = %finished.run_id,
                    protocol = finished.target.label(),
                    output_tokens = finished.token_usage.output_tokens,
                    "runtime task completed",
                );
                self.store_task(finished)
            }
            Err(err) => {
                let mut failed = self.require_task(task_id)?;
                failed.status = TaskStatus::Failed;
                failed.error = Some(err.message().to_string());
                failed.updated_at_ms = now_ms();
                append_audit(
                    &mut failed,
                    "task.failed",
                    "runtime",
                    err.message().to_string(),
                );
                error!(
                    session_id = %failed.session_id,
                    task_id = %failed.task_id,
                    run_id = %failed.run_id,
                    protocol = failed.target.label(),
                    error = %err.message(),
                    "runtime task failed",
                );
                self.store_task(failed)
            }
        }
    }

    async fn run_target(&self, task: &RuntimeTask) -> ServiceResult<TaskExecutionOutcome> {
        match &task.target {
            TaskTarget::Agent => self.execute_agent_task(task).await,
            TaskTarget::Mcp {
                tool_name,
                arguments,
            } => self.execute_mcp_task(task, tool_name, arguments).await,
            TaskTarget::A2a { url } => self.execute_a2a_task(task, url).await,
        }
    }

    async fn execute_agent_task(&self, task: &RuntimeTask) -> ServiceResult<TaskExecutionOutcome> {
        let session = SessionContext::new(task.session_id.clone()).with_run_id(task.run_id.clone());
        let ctx = AgentContext::new().with_session(session);
        let mut stream = self.agent.stream(AgentInput::Text(task.input.clone()), ctx);
        let mut output = String::new();
        let mut last_tool_output: Option<Value> = None;

        while let Some(item) = stream.next().await {
            let event = item.map_err(ServerError::from)?;
            self.event_store.append(&event).map_err(ServerError::from)?;
            match &event.kind {
                EventKind::LlmResponse { message } => {
                    if let Some(text) = message.text_content() {
                        output.push_str(&text);
                    }
                }
                EventKind::ToolCallEnd {
                    output: tool_output,
                    ..
                } => {
                    last_tool_output = Some(tool_output.clone());
                }
                EventKind::LlmRequest { .. } | EventKind::ToolCallStart { .. } => {}
            }
        }

        if output.trim().is_empty() {
            output = last_tool_output
                .map(|value| value.to_string())
                .unwrap_or_default();
        }

        Ok(TaskExecutionOutcome {
            output_tokens: estimate_tokens(&output),
            output,
        })
    }

    async fn execute_mcp_task(
        &self,
        task: &RuntimeTask,
        tool_name: &str,
        arguments: &Value,
    ) -> ServiceResult<TaskExecutionOutcome> {
        let start = protocol_event(
            &task.session_id,
            &task.run_id,
            EventKind::ToolCallStart {
                tool_name: format!("mcp::{tool_name}"),
                input: arguments.clone(),
            },
        );
        self.event_store.append(&start).map_err(ServerError::from)?;

        let (client_writer, server_reader) = duplex(4096);
        let (server_writer, client_reader) = duplex(4096);
        let server = McpServer::builder("nuro-runtime-server", "0.1.0")
            .tool(CalculatorTool::new())
            .build();

        let server_handle = tokio::spawn(async move {
            let reader = BufReader::new(server_reader);
            let _ = server.serve_io(reader, server_writer).await;
        });

        let reader = BufReader::new(client_reader);
        let mut client = McpClient::new(reader, client_writer);
        let result = client
            .call_tool(tool_name, arguments.clone())
            .await
            .map_err(|err| ServerError::internal(err.to_string()));
        drop(client);
        let _ = server_handle.await;
        let response = result?;

        let end = protocol_event(
            &task.session_id,
            &task.run_id,
            EventKind::ToolCallEnd {
                tool_name: format!("mcp::{tool_name}"),
                output: response.clone(),
            },
        );
        self.event_store.append(&end).map_err(ServerError::from)?;

        let output = response
            .get("content")
            .cloned()
            .unwrap_or(response)
            .to_string();

        Ok(TaskExecutionOutcome {
            output_tokens: estimate_tokens(&output),
            output,
        })
    }

    async fn execute_a2a_task(
        &self,
        task: &RuntimeTask,
        url: &str,
    ) -> ServiceResult<TaskExecutionOutcome> {
        let start = protocol_event(
            &task.session_id,
            &task.run_id,
            EventKind::ToolCallStart {
                tool_name: format!("a2a::{url}"),
                input: json!({ "input": task.input }),
            },
        );
        self.event_store.append(&start).map_err(ServerError::from)?;

        let card = A2aClient::discover(url).await.ok();
        let client = match &card {
            Some(card) => A2aClient::from_card(card),
            None => A2aClient::new(url.to_string()),
        };
        let (remote_task_id, output) = client
            .send_task(&task.input)
            .await
            .map_err(|err| ServerError::internal(err.to_string()))?;
        let chunks = client
            .subscribe_task(&remote_task_id)
            .await
            .unwrap_or_default();

        let end = protocol_event(
            &task.session_id,
            &task.run_id,
            EventKind::ToolCallEnd {
                tool_name: format!("a2a::{url}"),
                output: json!({
                    "remote_task_id": remote_task_id,
                    "output": output,
                    "stream_chunks": chunks,
                    "card": card,
                }),
            },
        );
        self.event_store.append(&end).map_err(ServerError::from)?;

        Ok(TaskExecutionOutcome {
            output_tokens: estimate_tokens(&output),
            output,
        })
    }

    fn validate_request(&self, request: &RuntimeTaskRequest) -> ServiceResult<()> {
        match &request.target {
            TaskTarget::Agent | TaskTarget::A2a { .. } if request.input.trim().is_empty() => {
                Err(ServerError::bad_request("input must not be empty"))
            }
            TaskTarget::A2a { url } if url.trim().is_empty() => {
                Err(ServerError::bad_request("a2a url must not be empty"))
            }
            TaskTarget::Mcp { tool_name, .. } if tool_name.trim().is_empty() => {
                Err(ServerError::bad_request("mcp tool name must not be empty"))
            }
            _ => Ok(()),
        }
    }

    fn require_task(&self, task_id: &str) -> ServiceResult<RuntimeTask> {
        self.get_task(task_id)
            .ok_or_else(|| ServerError::not_found(format!("task '{}' was not found", task_id)))
    }

    fn store_task(&self, task: RuntimeTask) -> ServiceResult<RuntimeTask> {
        let checkpoint =
            serde_json::to_value(&task).map_err(|err| ServerError::internal(err.to_string()))?;
        self.checkpoint_store
            .save_checkpoint(&task.session_id, &task.task_id, &checkpoint)
            .map_err(ServerError::from)?;

        {
            let mut sessions = self.sessions.lock().expect("sessions lock");
            let session = sessions.get_mut(&task.session_id).ok_or_else(|| {
                ServerError::not_found(format!("session '{}' was not found", task.session_id))
            })?;
            if !session.task_ids.iter().any(|id| id == &task.task_id) {
                session.task_ids.push(task.task_id.clone());
            }
            session.updated_at_ms = task.updated_at_ms;
        }

        self.tasks
            .lock()
            .expect("tasks lock")
            .insert(task.task_id.clone(), task.clone());
        Ok(task)
    }
}

fn protocol_event(session_id: &str, run_id: &str, kind: EventKind) -> Event {
    let mut event = Event::new(kind);
    event.session_id = Some(session_id.to_string());
    event.run_id = Some(run_id.to_string());
    event
}

fn append_audit(task: &mut RuntimeTask, action: &str, actor: &str, detail: impl Into<String>) {
    task.audit_trail.push(AuditRecord {
        timestamp_ms: now_ms(),
        action: action.to_string(),
        actor: actor.to_string(),
        detail: detail.into(),
    });
}

fn actor_or_system(actor: &Option<String>) -> String {
    actor
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("system")
        .to_string()
}

fn has_metadata(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Object(map) => !map.is_empty(),
        _ => true,
    }
}

fn replay_mode_name(mode: &ReplayMode) -> &'static str {
    match mode {
        ReplayMode::Strict => "strict",
        ReplayMode::Lenient => "lenient",
    }
}

fn estimate_tokens(text: &str) -> u64 {
    let chars = text.chars().count() as u64;
    if chars == 0 {
        return 0;
    }

    let whitespace = text.split_whitespace().count() as u64;
    whitespace.max(chars.div_ceil(4))
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0)
}
