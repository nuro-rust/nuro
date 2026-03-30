# ADK Quickstart

本指南展示如何用 `nuro-adk` 在几分钟内启动一个可运行 workflow。

## 1. 运行 starter 示例

```bash
cargo run -p adk_starter
```

输出会打印一条由 `AdkStarterApp` 生成的回复。

## 2. 在你自己的应用中使用 ADK

```rust
use nuro::prelude::*;

let app = AdkStarterBuilder::new()
    .system_prompt("You are my workflow assistant")
    .tool(CalculatorTool::new())
    .session_id("my-session")
    .build_with_mock()?;

let text = app.invoke_text("hello").await?;
println!("{}", text);
```

## 3. 切换到 OpenAI（可选）

启用 `openai` feature 并设置 `OPENAI_API_KEY` 后可使用：

```rust
let app = AdkStarterBuilder::new()
    .build_with_openai_model("gpt-4o-mini")?;
```

## 4. 推荐下一步

- 接入 `SqliteEventStore` + `ReplayEngine` 做调试回放
- 接入 `SqliteCheckpointStore` 做最小恢复
- 迁移到 `nuro-server` 暴露 API
- 评估流式 provider：`StreamingMockLlmProvider` + `LlmProviderAdapter`
