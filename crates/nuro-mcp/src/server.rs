use std::sync::Arc;

use anyhow::Result;
use nuro_core::{tool::ToolContext, Tool};
use serde_json::{json, Value};
use tokio::io::{self, AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufWriter};
use tokio::time::{timeout, Duration};

use crate::rpc::{RpcError, RpcRequest, RpcResponse};

/// MCP Server 的最小实现：
///
/// - 通过 JSON 行协议（1 行一个 JSON 对象）在给定 IO 通道上处理请求；
/// - 支持的方法：
///   - `list_tools`：返回所有已注册工具的名称与描述；
///   - `call_tool`：调用指定工具并返回结果；
/// - 传输格式为极简 JSON-RPC：
///   - 请求：`{"id": "1", "method": "list_tools", "params": {...}}`；
///   - 响应：`{"id": "1", "result": {...}}` 或 `{"id": "1", "error": {"message": "..."}}`。
pub struct McpServer {
    name: String,
    version: String,
    tools: Vec<Arc<dyn Tool>>,    
}

impl McpServer {
    pub fn builder(name: &str, version: &str) -> McpServerBuilder {
        McpServerBuilder {
            name: name.to_string(),
            version: version.to_string(),
            tools: Vec::new(),
        }
    }

    /// 使用 STDIN/STDOUT 作为传输通道运行 Server 主循环。
    pub async fn serve_stdio(self) -> Result<()> {
        let stdin = io::stdin();
        let stdout = io::stdout();

        let reader = BufReader::new(stdin);
        let writer = BufWriter::new(stdout);

        println!(
            "[MCP] '{}' v{} listening on stdio with {} tool(s)",
            self.name,
            self.version,
            self.tools.len()
        );

        self.serve_io(reader, writer).await
    }

    pub async fn serve_io<R, W>(self, reader: R, mut writer: W) -> Result<()>
    where
        R: AsyncBufRead + Unpin + Send + 'static,
        W: AsyncWrite + Unpin + Send + 'static,
    {
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let req: RpcRequest = match serde_json::from_str(trimmed) {
                Ok(r) => r,
                Err(err) => {
                    let resp = RpcResponse {
                        id: "".to_string(),
                        result: None,
                        error: Some(RpcError {
                            message: format!("failed to parse request: {err}"),
                        }),
                    };
                    let text = serde_json::to_string(&resp)?;
                    writer.write_all(text.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                    writer.flush().await?;
                    continue;
                }
            };

            let resp = handle_request(&self.tools, req).await;
            let text = serde_json::to_string(&resp)?;
            writer.write_all(text.as_bytes()).await?;
            writer.write_all(b"\n").await?;
            writer.flush().await?;
        }

        Ok(())
    }
}

pub struct McpServerBuilder {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) tools: Vec<Arc<dyn Tool>>,    
}

impl McpServerBuilder {
    /// 注册一个工具。
    pub fn tool<T>(mut self, tool: T) -> Self
    where
        T: Tool + 'static,
    {
        self.tools.push(Arc::new(tool));
        self
    }

    pub fn build(self) -> McpServer {
        McpServer {
            name: self.name,
            version: self.version,
            tools: self.tools,
        }
    }
}

async fn handle_request(tools: &[Arc<dyn Tool>], req: RpcRequest) -> RpcResponse {
    match req.method.as_str() {
        "list_tools" => {
            let tools_info: Vec<Value> = tools
                .iter()
                .map(|t| {
                    json!({
                        "name": t.name(),
                        "description": t.description(),
                    })
                })
                .collect();

            RpcResponse {
                id: req.id,
                result: Some(json!({ "tools": tools_info })),
                error: None,
            }
        }
        "call_tool" => {
            let name = req
                .params
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let args = req.params.get("args").cloned().unwrap_or_else(|| json!({}));

            let Some(tool) = tools.iter().find(|t| t.name() == name) else {
                return RpcResponse {
                    id: req.id,
                    result: None,
                    error: Some(RpcError {
                        message: format!("tool '{}' not found", name),
                    }),
                };
            };

            let ctx = ToolContext::new();

            // 简化版超时控制：单次工具调用最长 30 秒。
            let result = timeout(Duration::from_secs(30), tool.execute(args, &ctx)).await;

            match result {
                Err(_) => RpcResponse {
                    id: req.id,
                    result: None,
                    error: Some(RpcError {
                        message: format!("tool '{}' execution timed out", name),
                    }),
                },
                Ok(Err(err)) => RpcResponse {
                    id: req.id,
                    result: None,
                    error: Some(RpcError {
                        message: err.to_string(),
                    }),
                },
                Ok(Ok(output)) => RpcResponse {
                    id: req.id,
                    result: Some(json!({ "content": output.content })),
                    error: None,
                },
            }
        }
        other => RpcResponse {
            id: req.id,
            result: None,
            error: Some(RpcError {
                message: format!("unknown method '{other}'"),
            }),
        },
    }
}
