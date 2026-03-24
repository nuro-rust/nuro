use anyhow::Result;
use nuro::prelude::*;
use tokio::io::{self, BufReader, BufWriter};

/// 一个最小的 MCP in-process demo：
///
/// - 使用 `McpServer` 注册内置的 `CalculatorTool`；
/// - 通过 `tokio::io::duplex` 建立内存双向通道；
/// - 同进程内启动 Server 和 Client，演示 `list_tools` 与 `call_tool`。
#[tokio::main]
async fn main() -> Result<()> {
    // 1. 构建 MCP Server，注册一个 calculator 工具。
    let server = McpServer::builder("mcp_demo_server", "0.1.0")
        .tool(CalculatorTool::new())
        .build();

    // 2. 使用 duplex 在同进程内模拟 stdio 通道。
    let (server_stream, client_stream) = io::duplex(1024);

    // 3. 启动 Server 任务：在内存通道上处理 JSON-RPC 请求。
    let server_task = tokio::spawn(async move {
        let (read_half, write_half) = io::split(server_stream);
        let reader = BufReader::new(read_half);
        let writer = BufWriter::new(write_half);

        if let Err(err) = server.serve_io(reader, writer).await {
            eprintln!("[mcp_demo] server error: {err}");
        }
    });

    // 4. 构建 Client，使用另一端的内存通道。
    let (client_read, client_write) = io::split(client_stream);
    let reader = BufReader::new(client_read);
    let writer = BufWriter::new(client_write);
    let mut client = McpClient::new(reader, writer);

    // 5. 调用 list_tools。
    let tools = client.list_tools().await?;
    println!("Remote tools:\n{}", serde_json::to_string_pretty(&tools)?);

    // 6. 调用远程 calculator 工具。
    let args = serde_json::json!({ "expression": "1 + 2 * 3" });
    let result = client.call_tool("calculator", args).await?;
    println!("calculator result: {}", result);

    // 结束 Demo，终止 server 任务。
    server_task.abort();

    Ok(())
}
