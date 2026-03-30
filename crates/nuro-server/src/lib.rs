//! nuro-server — 基于 axum 的最小 HTTP Server 实现。
//!
//! 暴露的 HTTP 接口：
//! - `GET /health`：健康检查，返回 `{ "status": "ok" }`；
//! - `POST /v1/chat`：一次性聊天接口，请求体 `{ "input": String }`，响应
//!   `{ "output": String }`；
//! - `POST /v1/chat/stream`：SSE 流式接口，请求体同上，响应为一串 `Event`
//!   JSON 文本块，便于前端增量展示。
//!
//! 内部实现：
//! - 使用 `AgentLoop + MockLlmProvider + ToolBox(CalculatorTool)` 处理请求；
//! - 使用 `tracing` 输出基本的结构化日志（未接入 OTEL）。

use std::convert::Infallible;
use std::net::SocketAddr;

use anyhow::Result;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{
        IntoResponse,
        sse::{Event as SseEvent, Sse},
    },
    routing::{get, post},
};
use nuro_core::{AgentContext, AgentInput};
use nuro_llm::MockLlmProvider;
use nuro_runtime::AgentLoop;
use nuro_tools::{CalculatorTool, ToolBox};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;
use tracing::{error, info};

#[derive(Clone)]
struct AppState {
    agent: AgentLoop,
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

/// 启动一个 HTTP Server，监听指定地址并提供 `/health`、`/v1/chat` 与
/// `/v1/chat/stream` 接口。
pub async fn run_server(addr: SocketAddr) -> Result<()> {
    // 构建内部使用的 Agent。
    let toolbox = ToolBox::new().with_tool(CalculatorTool::new());

    let agent = AgentLoop::builder()
        .llm(MockLlmProvider::new())
        .system_prompt("You are a simple HTTP echo bot with a calculator tool.")
        .toolbox(toolbox)
        .build()?;

    let state = AppState { agent };

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/v1/chat", post(chat_handler))
        .route("/v1/chat/stream", post(chat_stream_handler))
        .with_state(state);

    info!(addr = %addr, "starting nuro HTTP server");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn chat_handler(
    State(state): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut ctx = AgentContext::new();
    let result = state.agent.run(AgentInput::Text(req.input), &mut ctx).await;

    match result {
        Ok(output) => {
            let text = output.text().unwrap_or_default();
            Ok(Json(ChatResponse { output: text }))
        }
        Err(err) => {
            error!(error = %err, "chat_handler error");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: err.to_string(),
                }),
            ))
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
