![Nuro logo](nuro.jpeg)

# Nuro - A Rust-Native AI Agent SDK

Nuro 是一个面向 Rust 开发者的 AI Agent SDK，强调事件驱动与图编排优先的执行模型，兼顾性能、安全与可组合性。它适合从命令行 Agent 到多 Agent 系统的研发与落地。

## 你能用 Nuro 做什么

- 用 `AgentLoop` 构建可控的 THINK -> ACT -> OBSERVE 执行循环
- 用 `StateGraph` 组织复杂流程、条件路由与可恢复状态
- 以 `Tool` 与 `ToolBox` 形式接入外部能力
- 通过 `nuro-server` 快速暴露 HTTP/SSE 接口
- 以 MCP/A2A 进行多 Agent 协作与发现

## 快速开始

确保已安装 Rust 2024 Edition 和 Cargo。

```bash
git clone https://github.com/nuro-rust/nuro.git
cd nuro
cargo build
```

### 运行示例

最小示例：

```bash
cargo run -p simple_chatbot
```

其他示例：

```bash
cargo run -p graph_demo
cargo run -p http_server
cargo run -p mcp_demo
cargo run -p a2a_demo
```

## 集成指南

### 方式一：以 git 依赖引入

在你的项目 `Cargo.toml` 中加入：

```toml
[dependencies]
nuro = { git = "https://github.com/nuro-rust/nuro.git" }
```

如果只需要某个模块，也可以直接引入对应 crate，例如：

```toml
[dependencies]
nuro-runtime = { git = "https://github.com/nuro-rust/nuro.git" }
nuro-graph = { git = "https://github.com/nuro-rust/nuro.git" }
```

### 方式二：作为 workspace 子模块开发

适合需要同时改 SDK 源码与业务代码的场景：

```bash
git submodule add https://github.com/nuro-rust/nuro.git vendor/nuro
```

然后在你的 workspace 中引用本地路径：

```toml
[dependencies]
nuro = { path = "vendor/nuro/nuro" }
```

### 可选功能

启用 OpenAI：

```toml
[dependencies]
nuro = { git = "https://github.com/nuro-rust/nuro.git", features = ["openai"] }
```

## Demo 速写

最小 StateGraph 示例：

```rust
use anyhow::Result;
use nuro::prelude::*;

#[derive(Debug, Clone)]
struct DemoState {
    text: String,
}

impl GraphStateTrait for DemoState {
    type Update = DemoState;

    fn apply_update(&mut self, update: Self::Update) {
        if !update.text.is_empty() {
            if !self.text.is_empty() {
                self.text.push(' ');
            }
            self.text.push_str(&update.text);
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let graph = StateGraph::<DemoState>::new()
        .add_node(
            "start",
            FnNode::new(|state: &DemoState, _ctx: &mut NodeContext| DemoState {
                text: format!("{}[start]", state.text),
            }),
        )
        .add_node(
            "end",
            FnNode::new(|state: &DemoState, _ctx: &mut NodeContext| DemoState {
                text: format!("{} -> [end]", state.text),
            }),
        )
        .set_entry_point("start")
        .add_edge("start", "end")
        .set_finish_point("end")
        .compile()?;

    let result = graph.invoke(DemoState { text: "demo".to_string() }).await?;
    println!("{:?}", result);
    Ok(())
}
```

最小 AgentLoop 示例：

```rust
use anyhow::Result;
use nuro::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let toolbox = ToolBox::new().with_tool(CalculatorTool::new());

    let agent = AgentLoop::builder()
        .llm(MockLlmProvider::new())
        .system_prompt("You are a simple echo bot with a calculator tool.")
        .toolbox(toolbox)
        .build()?;

    let mut ctx = AgentContext::new();
    let output = agent
        .run(AgentInput::text("hello".to_string()), &mut ctx)
        .await?;

    println!("{}", output.text().unwrap_or_else(|| "<no output>".to_string()));
    Ok(())
}
```

## 目录结构

- `nuro/`：根 crate，统一 API 入口
- `crates/nuro-core/`：基础类型与 Trait（`Message`, `Event`, `Agent`, `Tool` 等）
- `crates/nuro-runtime/`：Agent 执行循环与事件流
- `crates/nuro-graph/`：状态图引擎 `StateGraph`
- `crates/nuro-tools/`：工具抽象与工具容器
- `crates/nuro-llm/`：LLM Provider 抽象与实现
- `crates/nuro-memory/`：对话记忆与存储
- `crates/nuro-rag/`：RAG 相关组件
- `crates/nuro-mcp/`：MCP 协议最小实现
- `crates/nuro-a2a/`：A2A 协议实现
- `crates/nuro-server/`：最小 HTTP Server 封装
- `examples/`：示例工程

## 文档

- 架构设计文档：`docs/Nuro_—_Rust_Agent_SDK_技术架构设计文档.lark.md`
- 进阶用法与 OpenAI：`docs/QUICKSTART-ADVANCED.md`

## OpenAI 支持

默认使用 `MockLlmProvider`，不会访问在线服务。若需接入 OpenAI：

```bash
cargo build --features openai
export OPENAI_API_KEY=your_key
```

## 反馈与贡献

欢迎 Issue 与 PR。请在提交前确保示例可运行，接口变更有对应说明。

建议贡献前遵循以下约束：

- 采用增量迭代与兼容优先策略，尽量避免不必要 breaking change
- 公共 API 变更需标注影响级别（非破坏增强 / 弃用兼容 / 破坏性变更）
- 破坏性变更需提供迁移指南，并同步更新 changelog
- 新功能需同步补齐测试、文档与最小可运行示例
