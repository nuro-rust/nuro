use std::net::SocketAddr;
use std::time::Duration;

use anyhow::Result;
use nuro::prelude::*;

/// A2A demo：
///
/// - 启动一个基于 `AgentLoop` 的 A2A Server；
/// - 通过 `A2aClient::discover` 读取 `AgentCard`；
/// - 使用 `send_task` 发送一次任务；
/// - 使用 `subscribe_task` 订阅该任务的 SSE 流并打印各个数据块。
#[tokio::main]
async fn main() -> Result<()> {
    // 1. 构建底层 Agent。
    let toolbox = ToolBox::new().with_tool(CalculatorTool::new());
    let agent = AgentLoop::builder()
        .llm(MockLlmProvider::new())
        .system_prompt("You are an A2A demo agent with a calculator tool.")
        .toolbox(toolbox)
        .build()?;

    // 2. 启动 A2A Server。
    let addr: SocketAddr = "127.0.0.1:4010".parse()?;
    let server = A2aServer::builder()
        .agent(agent)
        .name("nuro-a2a-demo")
        .description("A minimal A2A demo agent built with Nuro")
        .version("0.1.0")
        .build();

    let server_handle = tokio::spawn(async move {
        if let Err(err) = server.serve(addr).await {
            eprintln!("[a2a_demo] server error: {err}");
        }
    });

    // 简单等待一下，确保服务已启动。
    tokio::time::sleep(Duration::from_millis(200)).await;

    let base_url = format!("http://{}", addr);

    // 3. 通过 /.well-known/agent.json 发现远程 Agent。
    let card = A2aClient::discover(&base_url).await?;
    println!(
        "Discovered agent: {} ({}) at {}",
        card.name, card.description, card.url
    );

    // 4. 构建客户端并发送一次任务。
    let client = A2aClient::from_card(&card);
    let (task_id, output) = client.send_task("calc: 1 + 2 * 3").await?;
    println!("Task created: id = {task_id}, output = {output}");

    // 5. 订阅该任务的 SSE 流，并打印每个数据块。
    let chunks = client.subscribe_task(&task_id).await?;
    println!("Stream chunks:");
    for (i, chunk) in chunks.iter().enumerate() {
        println!("  [{}] {}", i, chunk);
    }

    // 结束 Demo，终止 server 任务。
    server_handle.abort();

    Ok(())
}
