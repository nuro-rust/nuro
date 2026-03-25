# Nuro SDK 快速上手（高级）

本文档介绍如何启用 `nuro` SDK 的高级功能，特别是如何接入真实的 OpenAI LLM Provider，并提供一个最小的可用示例。

> 分层导航：
> - 四层模型：`docs/architecture/layer-model.md`
> - 模块映射：`docs/architecture/module-layer-mapping.md`
> - 分层上手：`docs/quickstart-layered.md`
> - 示例索引：`docs/examples/layer-index.md`

## 1. 启用 `openai` Feature

Nuro SDK 默认不开启任何需要网络或外部依赖的重型 feature，以保证 `cargo build` 默认通过且依赖集最小。

要启用 `OpenAiLlmProvider`，您需要在项目的 `Cargo.toml` 中为 `nuro` 添加 `openai` feature。

例如，在一个新的 Cargo 项目中，您的 `Cargo.toml` 可能看起来像这样：

```toml
[package]
name = "my-openai-agent"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1"
tokio = { version = "1", features = ["full"] }

# 启用 nuro 的 openai feature
nuro = { path = "/path/to/your/nuro/workspace/nuro", features = ["openai"] }
```

> **注意**：请将 `path` 指向您本地的 Nuro SDK workspace 根目录下的 `nuro` crate。如果通过 `crates.io` 引用，则使用 `version` 字段。

启用此 feature 后，`nuro` 会引入 `reqwest` 等网络库，并编译 `nuro-llm/src/openai.rs` 中的 `OpenAiLlmProvider`。

## 2. 配置环境变量

`OpenAiLlmProvider` 通过环境变量读取 API Key 与可选的 Base URL：

- `OPENAI_API_KEY` (必需): 您的 OpenAI API 密钥。
- `OPENAI_BASE_URL` (可选): 如果您使用 Azure OpenAI 或其他兼容的代理服务，可以在这里指定 API 的 base URL。如果未设置，默认为 `https://api.openai.com/v1`。

在您的 shell 中导出这些变量：

```bash
export OPENAI_API_KEY="sk-..."
# export OPENAI_BASE_URL="https://your-proxy.example.com/v1" # （可选）
```

## 3. 最小示例：使用 `OpenAiLlmProvider`

以下是一个简单的命令行聊天程序，它使用 `OpenAiLlmProvider` 替换了默认的 `MockLlmProvider`。

```rust
use anyhow::Result;
use nuro::prelude::*;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    // 检查环境变量是否已设置。
    if std::env::var("OPENAI_API_KEY").is_err() {
        eprintln!("Error: OPENAI_API_KEY environment variable is not set.");
        eprintln!("Please export your OpenAI API key to run this example.");
        return Ok(());
    }

    // 1. 构建 OpenAI LLM Provider。
    //    默认使用 "gpt-4o-mini" 模型，您也可以用 new_with_model("...") 自定义。
    let llm = OpenAiLlmProvider::new()?;

    // 2. 构建 AgentLoop，这次传入真实的 LLM Provider。
    let agent = AgentLoop::builder()
        .llm(llm)
        .system_prompt("You are a helpful assistant.")
        .build()?;

    let mut ctx = AgentContext::new();

    println!("Simple OpenAI Chatbot (type 'quit' to exit)");

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

        // 3. 运行 Agent 并打印结果。
        let output = agent
            .run(AgentInput::text(line.to_string()), &mut ctx)
            .await?;

        let reply = output.text().unwrap_or_else(|| "<no output>".to_string());
        println!("Assistant: {}", reply);
    }

    Ok(())
}
```

## 4. 运行示例

假设您已将上述代码保存为 `main.rs`，并配置好了 `Cargo.toml` 和环境变量，现在可以运行它：

```bash
cargo run
```

程序启动后，您可以向其提问，它会通过 OpenAI API 获取回答并打印出来。

```text
Simple OpenAI Chatbot (type 'quit' to exit)
You: What is the capital of France?
Assistant: The capital of France is Paris.
```

这个示例展示了如何从默认的 `MockLlmProvider` 平滑切换到真实的 `OpenAiLlmProvider`，只需更改几行初始化代码并启用相应的 feature gate。SDK 的其余部分（如 `AgentLoop`、`ToolBox` 等）保持不变，体现了 `nuro-core` 中 `LlmProvider` trait 的抽象能力。
