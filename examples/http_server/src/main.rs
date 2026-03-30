use std::net::SocketAddr;

use anyhow::Result;
use nuro::prelude::*;

/// 使用 `nuro-server` 提供的 `run_server` 启动一个最小 HTTP 服务：
///
/// - 默认监听 `127.0.0.1:3000`（可通过环境变量 `NURO_HTTP_ADDR` 覆盖）；
/// - 提供接口：
///   - `GET /health`
///   - `POST /v1/chat`
///   - `POST /v1/chat/stream`（SSE 流式输出）。
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化 tracing 日志（打印到 stdout）。
    tracing_subscriber::fmt::init();

    let addr_str = std::env::var("NURO_HTTP_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let addr: SocketAddr = addr_str.parse()?;

    println!("Starting nuro HTTP server on http://{}", addr);
    run_server(addr).await?;
    Ok(())
}
