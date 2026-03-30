use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt};
use tokio::time::{Duration, timeout};

use crate::rpc::{RpcRequest, RpcResponse};

/// MCP Client 的简化实现：
///
/// - 通过任意实现 `AsyncBufRead`/`AsyncWrite` 的 IO 通道发送 JSON-RPC 请求；
/// - 当前仅支持顺序调用，不做并发复用；
/// - 提供基础的超时与错误处理。
pub struct McpClient {
    reader: Box<dyn AsyncBufRead + Unpin + Send>,
    writer: Box<dyn AsyncWrite + Unpin + Send>,
    timeout: Duration,
    next_id: u64,
}

impl McpClient {
    /// 通过自定义 IO 构造一个 MCP Client。
    ///
    /// 典型用法是在同进程内使用 `tokio::io::duplex` 建立内存通道，
    /// 一端交给 `McpServer::serve_stdio` 的变体，另一端交给 `McpClient::new`。
    pub fn new<R, W>(reader: R, writer: W) -> Self
    where
        R: AsyncBufRead + Unpin + Send + 'static,
        W: AsyncWrite + Unpin + Send + 'static,
    {
        Self {
            reader: Box::new(reader),
            writer: Box::new(writer),
            timeout: Duration::from_secs(30),
            next_id: 1,
        }
    }

    /// 通过 stdio 连接 MCP Server 的占位实现。
    ///
    /// 当前版本尚未内置子进程管理逻辑，因此该方法返回错误，
    /// 建议在调用方自行启动子进程并将其 IO 封装为 `McpClient::new(...)`。
    pub async fn connect_stdio(_cmd: &str, _args: &[&str]) -> Result<Self> {
        Err(anyhow!(
            "McpClient::connect_stdio is not implemented; use McpClient::new(reader, writer) with a custom IO transport instead"
        ))
    }

    /// 返回远程提供的工具列表（每个元素为 JSON 对象，至少包含 `name` 与 `description` 字段）。
    pub async fn list_tools(&mut self) -> Result<Vec<Value>> {
        let result = self.send_request("list_tools", json!({})).await?;
        let tools = result
            .get("tools")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        Ok(tools)
    }

    /// 调用远程工具并返回其结果 JSON。
    pub async fn call_tool(&mut self, name: &str, args: Value) -> Result<Value> {
        let params = json!({ "name": name, "args": args });
        let result = self.send_request("call_tool", params).await?;
        Ok(result)
    }

    async fn send_request(&mut self, method: &str, params: Value) -> Result<Value> {
        let id = self.next_id.to_string();
        self.next_id += 1;

        let req = RpcRequest {
            id: id.clone(),
            method: method.to_string(),
            params,
        };

        let text = serde_json::to_string(&req)?;
        self.writer.write_all(text.as_bytes()).await?;
        self.writer.write_all(b"\n").await?;
        self.writer.flush().await?;

        let mut line = String::new();
        let read_future = self.reader.read_line(&mut line);
        let result = timeout(self.timeout, read_future)
            .await
            .map_err(|_| anyhow!("MCP response timed out"))?;

        let bytes_read = result?;
        if bytes_read == 0 {
            return Err(anyhow!("MCP server closed the connection"));
        }

        let trimmed = line.trim();
        let resp: RpcResponse = serde_json::from_str(trimmed)
            .map_err(|e| anyhow!("failed to parse MCP response: {e}"))?;

        if resp.id != id {
            return Err(anyhow!(
                "mismatched response id: expected {}, got {}",
                id,
                resp.id
            ));
        }

        if let Some(err) = resp.error {
            return Err(anyhow!(err.message));
        }

        Ok(resp.result.unwrap_or(Value::Null))
    }
}
