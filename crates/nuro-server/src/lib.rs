//! nuro-server — Runtime gateway built on top of axum.
//!
//! The server keeps the original demo chat endpoints and adds a minimal
//! platform-oriented API:
//! - `POST /v1/sessions`
//! - `GET /v1/sessions/:id`
//! - `POST /v1/tasks`
//! - `GET /v1/tasks/:id`
//! - `POST /v1/tasks/:id/approve`
//! - `POST /v1/tasks/:id/interrupt`
//! - `POST /v1/tasks/:id/resume`
//! - `GET /v1/replay/sessions/:id`
//! - `GET /metrics`
//! - `GET /playground`
//! - `GET /trace`

mod models;
mod policy;
mod service;
mod ui;

use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::{HeaderValue, StatusCode, header},
    response::{
        Html, IntoResponse,
        sse::{Event as SseEvent, Sse},
    },
    routing::{get, post},
};
use nuro_core::{AgentContext, AgentInput};
use nuro_llm::MockLlmProvider;
use nuro_runtime::{
    AgentLoop, CheckpointStore, EventStore, ReplayMode, SqliteCheckpointStore, SqliteEventStore,
};
use nuro_tools::{CalculatorTool, ToolBox};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;
use tracing::error;

pub use models::{
    ActionRequest, MetricsSnapshot, ReplayResponse, RuntimeSession, RuntimeTask,
    RuntimeTaskRequest, SessionCreateRequest, TaskStatus, TaskTarget, TokenUsage,
};
pub use policy::{PolicyAction, PolicyDecision, PolicyEngine, PolicyRule};
pub use service::{RuntimeService, ServerError};

#[derive(Clone)]
struct AppState {
    agent: AgentLoop,
    service: Arc<RuntimeService>,
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub input: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub output: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Debug, Deserialize, Default)]
struct ReplayQuery {
    run_id: Option<String>,
    mode: Option<String>,
}

pub struct RuntimeServer {
    app: Router,
}

pub struct RuntimeServerBuilder {
    agent: Option<AgentLoop>,
    sqlite_path: Option<String>,
    event_store: Option<Arc<dyn EventStore>>,
    checkpoint_store: Option<Arc<dyn CheckpointStore>>,
    policy_engine: PolicyEngine,
}

impl RuntimeServer {
    pub fn builder() -> RuntimeServerBuilder {
        RuntimeServerBuilder {
            agent: None,
            sqlite_path: None,
            event_store: None,
            checkpoint_store: None,
            policy_engine: PolicyEngine::default(),
        }
    }

    pub fn into_router(self) -> Router {
        self.app
    }

    pub async fn serve(self, addr: SocketAddr) -> Result<()> {
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, self.app).await?;
        Ok(())
    }
}

impl RuntimeServerBuilder {
    pub fn agent(mut self, agent: AgentLoop) -> Self {
        self.agent = Some(agent);
        self
    }

    pub fn sqlite_path(mut self, path: impl Into<String>) -> Self {
        self.sqlite_path = Some(path.into());
        self
    }

    pub fn event_store(mut self, store: Arc<dyn EventStore>) -> Self {
        self.event_store = Some(store);
        self
    }

    pub fn checkpoint_store(mut self, store: Arc<dyn CheckpointStore>) -> Self {
        self.checkpoint_store = Some(store);
        self
    }

    pub fn policy_engine(mut self, engine: PolicyEngine) -> Self {
        self.policy_engine = engine;
        self
    }

    pub fn policy_rule(mut self, rule: PolicyRule) -> Self {
        self.policy_engine = self.policy_engine.with_rule(rule);
        self
    }

    pub fn build(self) -> Result<RuntimeServer> {
        let sqlite_path = self.sqlite_path.unwrap_or_else(default_sqlite_path);
        let event_store = match self.event_store {
            Some(store) => store,
            None => Arc::new(SqliteEventStore::new(sqlite_path.clone())?),
        };
        let checkpoint_store = match self.checkpoint_store {
            Some(store) => store,
            None => Arc::new(SqliteCheckpointStore::new(sqlite_path)?),
        };

        let agent = match self.agent {
            Some(agent) => agent,
            None => default_agent()?,
        };
        let service = Arc::new(RuntimeService::new(
            agent.clone(),
            event_store,
            checkpoint_store,
            self.policy_engine,
        ));

        Ok(RuntimeServer {
            app: build_app(agent, service),
        })
    }
}

pub async fn run_server(addr: SocketAddr) -> Result<()> {
    RuntimeServer::builder().build()?.serve(addr).await
}

fn build_app(agent: AgentLoop, service: Arc<RuntimeService>) -> Router {
    let state = AppState { agent, service };
    Router::new()
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler))
        .route("/playground", get(playground_handler))
        .route("/trace", get(trace_viewer_handler))
        .route("/v1/chat", post(chat_handler))
        .route("/v1/chat/stream", post(chat_stream_handler))
        .route("/v1/sessions", post(create_session_handler))
        .route("/v1/sessions/:id", get(get_session_handler))
        .route("/v1/tasks", post(create_task_handler))
        .route("/v1/tasks/:id", get(get_task_handler))
        .route("/v1/tasks/:id/approve", post(approve_task_handler))
        .route("/v1/tasks/:id/interrupt", post(interrupt_task_handler))
        .route("/v1/tasks/:id/resume", post(resume_task_handler))
        .route("/v1/replay/sessions/:id", get(replay_session_handler))
        .with_state(state)
}

fn default_agent() -> Result<AgentLoop> {
    let toolbox = ToolBox::new().with_tool(CalculatorTool::new());
    AgentLoop::builder()
        .llm(MockLlmProvider::new())
        .system_prompt("You are a runtime gateway bot with calculator and replay support.")
        .toolbox(toolbox)
        .build()
        .map_err(anyhow::Error::from)
}

fn default_sqlite_path() -> String {
    std::env::var("NURO_SERVER_DB").unwrap_or_else(|_| {
        let mut path = std::env::temp_dir();
        path.push("nuro-runtime-server.db");
        path.to_string_lossy().to_string()
    })
}

async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn metrics_handler(State(state): State<AppState>) -> impl IntoResponse {
    (
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/plain; version=0.0.4"),
        )],
        state.service.metrics_prometheus(),
    )
}

async fn playground_handler() -> Html<String> {
    Html(ui::playground_html())
}

async fn trace_viewer_handler() -> Html<String> {
    Html(ui::trace_viewer_html())
}

async fn chat_handler(
    State(state): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut ctx = AgentContext::new();
    let result = state.agent.run(AgentInput::Text(req.input), &mut ctx).await;

    match result {
        Ok(output) => Ok(Json(ChatResponse {
            output: output.text().unwrap_or_default(),
        })),
        Err(err) => {
            error!(error = %err, "chat_handler error");
            Err(api_error(ServerError::from(err)))
        }
    }
}

async fn chat_stream_handler(
    State(state): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> impl IntoResponse {
    let ctx = AgentContext::new();
    let event_stream = state.agent.stream(AgentInput::Text(req.input), ctx).map(
        |item| -> Result<SseEvent, Infallible> {
            match item {
                Ok(event) => {
                    let data = serde_json::to_string(&event).unwrap_or_else(|_| "{}".to_string());
                    Ok(SseEvent::default().data(data))
                }
                Err(err) => {
                    let data = serde_json::json!({ "error": err.to_string() }).to_string();
                    Ok(SseEvent::default().data(data))
                }
            }
        },
    );

    Sse::new(event_stream)
}

async fn create_session_handler(
    State(state): State<AppState>,
    Json(req): Json<SessionCreateRequest>,
) -> Result<(StatusCode, Json<RuntimeSession>), (StatusCode, Json<ErrorResponse>)> {
    let session = state
        .service
        .create_session(req.session_id, req.metadata)
        .map_err(api_error)?;
    Ok((StatusCode::CREATED, Json(session)))
}

async fn get_session_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<RuntimeSession>, (StatusCode, Json<ErrorResponse>)> {
    let session = state.service.get_session(&id).ok_or_else(|| {
        api_error(ServerError::not_found(format!(
            "session '{}' was not found",
            id
        )))
    })?;
    Ok(Json(session))
}

async fn create_task_handler(
    State(state): State<AppState>,
    Json(req): Json<RuntimeTaskRequest>,
) -> Result<(StatusCode, Json<RuntimeTask>), (StatusCode, Json<ErrorResponse>)> {
    let task = state.service.submit_task(req).await.map_err(api_error)?;
    let status = match task.status {
        TaskStatus::PendingApproval | TaskStatus::Interrupted => StatusCode::ACCEPTED,
        TaskStatus::Running | TaskStatus::Completed | TaskStatus::Failed => StatusCode::OK,
    };
    Ok((status, Json(task)))
}

async fn get_task_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<RuntimeTask>, (StatusCode, Json<ErrorResponse>)> {
    let task = state.service.get_task(&id).ok_or_else(|| {
        api_error(ServerError::not_found(format!(
            "task '{}' was not found",
            id
        )))
    })?;
    Ok(Json(task))
}

async fn approve_task_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ActionRequest>,
) -> Result<Json<RuntimeTask>, (StatusCode, Json<ErrorResponse>)> {
    let task = state
        .service
        .approve_task(&id, req)
        .await
        .map_err(api_error)?;
    Ok(Json(task))
}

async fn interrupt_task_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ActionRequest>,
) -> Result<Json<RuntimeTask>, (StatusCode, Json<ErrorResponse>)> {
    let task = state
        .service
        .interrupt_task(&id, req)
        .await
        .map_err(api_error)?;
    Ok(Json(task))
}

async fn resume_task_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ActionRequest>,
) -> Result<Json<RuntimeTask>, (StatusCode, Json<ErrorResponse>)> {
    let task = state
        .service
        .resume_task(&id, req)
        .await
        .map_err(api_error)?;
    Ok(Json(task))
}

async fn replay_session_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<ReplayQuery>,
) -> Result<Json<ReplayResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mode = parse_replay_mode(query.mode.as_deref());
    let replay = state
        .service
        .replay_session(&id, query.run_id.as_deref(), mode)
        .map_err(api_error)?;
    Ok(Json(replay))
}

fn parse_replay_mode(mode: Option<&str>) -> ReplayMode {
    match mode.unwrap_or("strict").to_ascii_lowercase().as_str() {
        "lenient" => ReplayMode::Lenient,
        _ => ReplayMode::Strict,
    }
}

fn api_error(error: ServerError) -> (StatusCode, Json<ErrorResponse>) {
    let status = StatusCode::from_u16(error.status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (
        status,
        Json(ErrorResponse {
            error: error.message().to_string(),
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::body::{Body, to_bytes};
    use axum::http::Request;
    use nuro_a2a::A2aServer;
    use serde::de::DeserializeOwned;
    use serde_json::{Value, json};
    use tower::util::ServiceExt;

    fn temp_db_path(name: &str) -> String {
        let mut path = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        path.push(format!("nuro-server-{name}-{nanos}.db"));
        path.to_string_lossy().to_string()
    }

    fn test_router(policy_engine: PolicyEngine) -> Router {
        let db_path = temp_db_path("router");
        RuntimeServer::builder()
            .sqlite_path(db_path)
            .policy_engine(policy_engine)
            .build()
            .expect("runtime server")
            .into_router()
    }

    async fn read_json<T>(response: axum::response::Response) -> T
    where
        T: DeserializeOwned,
    {
        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body bytes");
        serde_json::from_slice(&bytes).expect("json body")
    }

    async fn read_text(response: axum::response::Response) -> String {
        let bytes = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("body bytes");
        String::from_utf8(bytes.to_vec()).expect("utf8 body")
    }

    fn post_json(uri: &str, value: Value) -> Request<Body> {
        Request::builder()
            .method("POST")
            .uri(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(value.to_string()))
            .expect("request")
    }

    #[tokio::test]
    async fn agent_tasks_are_completed_and_replayable() {
        let app = test_router(PolicyEngine::default());

        let response = app
            .clone()
            .oneshot(post_json("/v1/tasks", json!({ "input": "calc: 2 + 2" })))
            .await
            .expect("task response");
        assert_eq!(response.status(), StatusCode::OK);
        let task: RuntimeTask = read_json(response).await;
        assert_eq!(task.status, TaskStatus::Completed, "{task:?}");
        assert!(task.output.as_deref().unwrap_or_default().contains('4'));
        let session_id = task.session_id.clone();

        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/replay/sessions/{}", session_id))
                    .body(Body::empty())
                    .expect("replay request"),
            )
            .await
            .expect("replay response");
        assert_eq!(response.status(), StatusCode::OK);
        let replay: ReplayResponse = read_json(response).await;
        assert!(replay.events.len() >= 3);
    }

    #[tokio::test]
    async fn approval_flow_runs_after_manual_approval() {
        let app = test_router(PolicyEngine::default());

        let response = app
            .clone()
            .oneshot(post_json(
                "/v1/tasks",
                json!({ "input": "hello approval", "requires_approval": true }),
            ))
            .await
            .expect("task response");
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        let task: RuntimeTask = read_json(response).await;
        assert_eq!(task.status, TaskStatus::PendingApproval);

        let response = app
            .oneshot(post_json(
                &format!("/v1/tasks/{}/approve", task.task_id),
                json!({ "actor": "reviewer", "comment": "approved for execution" }),
            ))
            .await
            .expect("approval response");
        assert_eq!(response.status(), StatusCode::OK);
        let approved: RuntimeTask = read_json(response).await;
        assert_eq!(approved.status, TaskStatus::Completed, "{approved:?}");
        assert!(
            approved
                .output
                .unwrap_or_default()
                .contains("Echo: hello approval")
        );
    }

    #[tokio::test]
    async fn policy_rules_can_reject_requests_and_update_metrics() {
        let app = test_router(
            PolicyEngine::default().with_rule(PolicyRule::deny_contains("deny-secret", "secret")),
        );

        let response = app
            .clone()
            .oneshot(post_json("/v1/tasks", json!({ "input": "secret draft" })))
            .await
            .expect("task response");
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/metrics")
                    .body(Body::empty())
                    .expect("metrics request"),
            )
            .await
            .expect("metrics response");
        let metrics = read_text(response).await;
        assert!(metrics.contains("nuro_policy_rejections_total 1"));
    }

    #[tokio::test]
    async fn mcp_tasks_are_executed_through_the_gateway() {
        let app = test_router(PolicyEngine::default());

        let response = app
            .clone()
            .oneshot(post_json(
                "/v1/tasks",
                json!({
                    "input": "call calculator through mcp",
                    "target": {
                        "protocol": "mcp",
                        "tool_name": "calculator",
                        "arguments": { "expression": "6 * 7" }
                    }
                }),
            ))
            .await
            .expect("task response");
        assert_eq!(response.status(), StatusCode::OK);
        let task: RuntimeTask = read_json(response).await;
        assert_eq!(task.status, TaskStatus::Completed);
        assert!(task.output.as_deref().unwrap_or_default().contains("42"));
        let session_id = task.session_id.clone();

        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/replay/sessions/{}", session_id))
                    .body(Body::empty())
                    .expect("replay request"),
            )
            .await
            .expect("replay response");
        let replay: ReplayResponse = read_json(response).await;
        let payload = serde_json::to_string(&replay).expect("replay json");
        assert!(payload.contains("mcp::calculator"));
    }

    #[tokio::test]
    async fn a2a_tasks_are_recorded_as_runtime_events() {
        let agent = default_agent().expect("default agent");
        let server = A2aServer::builder().agent(agent).build();
        let Ok(listener) = tokio::net::TcpListener::bind("127.0.0.1:0").await else {
            return;
        };
        let addr = listener.local_addr().expect("local addr");
        drop(listener);
        let handle = tokio::spawn(async move {
            let _ = server.serve(addr).await;
        });

        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let app = test_router(PolicyEngine::default());
        let response = app
            .clone()
            .oneshot(post_json(
                "/v1/tasks",
                json!({
                    "input": "remote hello",
                    "target": {
                        "protocol": "a2a",
                        "url": "http://127.0.0.1:4107"
                    }
                }),
            ))
            .await
            .expect("task response");
        assert_eq!(response.status(), StatusCode::OK);
        let task: RuntimeTask = read_json(response).await;
        assert_eq!(task.status, TaskStatus::Completed);
        assert!(
            task.output
                .as_deref()
                .unwrap_or_default()
                .contains("Echo: remote hello")
        );
        let session_id = task.session_id.clone();

        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/v1/replay/sessions/{}", session_id))
                    .body(Body::empty())
                    .expect("replay request"),
            )
            .await
            .expect("replay response");
        let replay: ReplayResponse = read_json(response).await;
        let payload = serde_json::to_string(&replay).expect("replay json");
        assert!(payload.contains(&format!("a2a::http://{}", addr)));

        handle.abort();
    }
}
