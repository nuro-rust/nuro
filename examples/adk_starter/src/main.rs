use anyhow::Result;
use nuro::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let app = AdkStarterBuilder::new()
        .system_prompt("You are an ADK starter assistant with calculator support.")
        .tool(CalculatorTool::new())
        .session_id("adk-starter-demo")
        .build_with_mock()?;

    let reply = app.invoke_text("Compute 21 * 2").await?;
    println!("ADK starter reply: {}", reply);

    Ok(())
}
