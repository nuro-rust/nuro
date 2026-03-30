use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use anyhow::Result;
use axum::{
    Json, Router,
    extract::{Path, State},
    response::{
        IntoResponse,
        sse::{Event as SseEvent, Sse},
    },
    routing::{get, post},
};
use nuro_core::{Agent, AgentContext, AgentInput};

use crate::types::{AgentCard, TaskCreateRequest, TaskCreateResponse};

/// 全局递增的任务 id 计数器。
static NEXT_TASK_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone)]
struct AppState {
    agent: Arc<dyn Agent>,
    card: AgentCard,
    tasks: Arc<Mutex<HashMap<String, String>>>,
}

/// A2A Server：将本地 Agent 通过 HTTP + SSE 暴露出去。
pub struct A2aServer {
    agent: Arc<dyn Agent>,
    name: String,
    description: String,
    version: String,
}

pub struct A2aServerBuilder {
    agent: Option<Arc<dyn Agent>>,
    name: String,
    description: String,
    version: String,
}

impl A2aServer {
    pub fn builder() -> A2aServerBuilder {
        A2aServerBuilder {
            agent: None,
            name: "nuro-agent".to_string(),
            description: "A minimal Nuro A2A agent".to_string(),
            version: "0.1.0".to_string(),
        }
    }

    /// 启动 A2A HTTP 服务：
    /// - `GET /.well-known/agent.json` 返回 `AgentCard`；
    /// - `POST /tasks` 接收任务文本，调用一次 Agent 并返回 `TaskCreateResponse`；
    /// - `GET /tasks/:id/stream` 以 SSE 形式按若干块流式返回该任务的输出。
    pub async fn serve(self, addr: SocketAddr) -> Result<()> {
        let base_url = format!("http://{}", addr);

        let card = AgentCard {
            name: self.name.clone(),
            description: self.description.clone(),
            url: base_url,
            version: self.version.clone(),
        };

        let state = AppState {
            agent: self.agent.clone(),
            card,
            tasks: Arc::new(Mutex::new(HashMap::new())),
        };

        let app = Router::new()
            .route("/.well-known/agent.json", get(agent_card_handler))
            .route("/tasks", post(create_task_handler))
            .route("/tasks/:id/stream", get(task_stream_handler))
            .with_state(state);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

impl A2aServerBuilder {
    pub fn agent<A>(mut self, agent: A) -> Self
    where
        A: Agent + 'static,
    {
        self.agent = Some(Arc::new(agent));
        self
    }

    /// 自定义 Agent 名称（用于 `AgentCard`）。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 自定义 Agent 描述（用于 `AgentCard`）。
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// 自定义 Agent 版本号（用于 `AgentCard`）。
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    pub fn build(self) -> A2aServer {
        let agent = self
            .agent
            .expect("A2aServer requires an Agent; call builder().agent(...) first");
        A2aServer {
            agent,
            name: self.name,
            description: self.description,
            version: self.version,
        }
    }
}

async fn agent_card_handler(State(state): State<AppState>) -> Json<AgentCard> {
    Json(state.card.clone())
}

async fn create_task_handler(
    State(state): State<AppState>,
    Json(req): Json<TaskCreateRequest>,
) -> Json<TaskCreateResponse> {
    let mut ctx = AgentContext::new();
    let output = state
        .agent
        .invoke(AgentInput::Text(req.input.clone()), &mut ctx)
        .await;

    let text = match output {
        Ok(out) => out.text().unwrap_or_default(),
        Err(err) => format!("A2A task execution error: {err}"),
    };

    let id = format!("task-{}", NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed));

    {
        let mut guard = state.tasks.lock().unwrap();
        guard.insert(id.clone(), text.clone());
    }

    Json(TaskCreateResponse { id, output: text })
}

async fn task_stream_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    use std::convert::Infallible;

    let output = {
        let guard = state.tasks.lock().unwrap();
        guard.get(&id).cloned()
    };

    let chunks: Vec<String> = match output {
        Some(text) => split_into_chunks(&text, 3),
        None => vec![format!("task '{}' not found", id)],
    };

    let stream = tokio_stream::iter(
        chunks
            .into_iter()
            .map(|chunk| Ok(SseEvent::default().data(chunk)) as Result<SseEvent, Infallible>),
    );

    Sse::new(stream)
}

fn split_into_chunks(text: &str, max_chunks: usize) -> Vec<String> {
    if text.is_empty() || max_chunks == 0 {
        return Vec::new();
    }

    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let chunk_count = max_chunks.min(len).max(1);
    let chunk_size = ((len as f32) / (chunk_count as f32)).ceil() as usize;

    let mut chunks = Vec::new();
    let mut start = 0usize;

    while start < len {
        let end = (start + chunk_size).min(len);
        let chunk: String = chars[start..end].iter().collect();
        chunks.push(chunk);
        start = end;
    }

    chunks
}
