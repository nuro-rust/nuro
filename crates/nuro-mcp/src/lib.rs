//! nuro-mcp — MCP 协议最小可用实现（简化版 JSON-RPC over stdio）。
//!
//! 该 crate 提供：
//! - `McpServer::builder(name, version).tool(...).serve_stdio()`：
//!   通过 STDIN/STDOUT 监听简化版 JSON-RPC 请求，支持 `list_tools` 与
//!   `call_tool` 两种方法；
//! - `McpClient`：基于任意实现 `AsyncBufRead`/`AsyncWrite` 的 IO 通道发送
//!   请求并等待响应；
//!   - `McpClient::new(reader, writer)`：从自定义 IO 构造客户端；
//!   - `list_tools`：返回远程工具列表；
//!   - `call_tool`：调用远程工具并返回结果。
//!
//! 为了保持实现轻量：
//! - 协议仅覆盖 MCP 的工具部分，不包含资源/提示等扩展；
//! - 不支持并发流水线请求，所有调用按顺序串行执行；
//! - 仅实现简化错误与超时处理逻辑。

mod client;
mod rpc;
mod server;

pub use client::McpClient;
pub use server::{McpServer, McpServerBuilder};
