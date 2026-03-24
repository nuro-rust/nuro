use anyhow::Result;
use nuro::prelude::*;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    // 构建工具箱：仅包含一个 CalculatorTool
    let toolbox = ToolBox::new().with_tool(CalculatorTool::new());

    // 使用 MockLlmProvider + 工具箱构建最小 AgentLoop
    let agent = AgentLoop::builder()
        .llm(MockLlmProvider::new())
        .system_prompt("You are a simple echo bot with a calculator tool.")
        .toolbox(toolbox)
        .build()?;

    let mut ctx = AgentContext::new();

    println!("Simple Chatbot (type 'quit' to exit)");

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut line = String::new();
        if io::stdin().read_line(&mut line)? == 0 {
            break;
        }
        let line = line.trim();
        if line.eq_ignore_ascii_case("quit") {
            break;
        }
        if line.is_empty() {
            continue;
        }

        let output = agent
            .run(AgentInput::text(line.to_string()), &mut ctx)
            .await?;

        let reply = output.text().unwrap_or_else(|| "<no output>".to_string());
        println!("Assistant: {}", reply);
    }

    Ok(())
}
