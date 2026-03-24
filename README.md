# Nuro - A Rust-Native AI Agent SDK

Nuro 是一个用 Rust 构建的、事件驱动的、图编排优先的 AI Agent 开发框架。它旨在充分利用 Rust 的性能、安全性和现代异步生态，为构建从简单聊天机器人到复杂多 Agent 系统的各类应用提供坚实基础。

当前仓库实现的是一个“功能完整但保持精简”的版本，用于验证核心抽象与最小功能闭环，并为后续扩展打好基础。

## 架构概述

Workspace 采用 Cargo Workspace 的 monorepo 结构，核心功能被拆分为多个独立的 crate，以实现清晰的模块化和关注点分离。

- 架构设计文档：`docs/Nuro_—_Rust_Agent_SDK_技术架构设计文档.lark.md`
- **`nuro/`**: Workspace 根目录。
    - **`Cargo.toml`**: 定义整个 workspace 的成员和共享依赖。
    - **`nuro/`**: 根 crate，作为统一的公共 API 入口，re-export 核心组件。
    - **`crates/`**: 存放所有核心库 crate。
        - **`nuro-core/`**: 定义整个 SDK 的基础类型和 Trait，如 `Message`, `Event`, `Agent`, `LlmProvider`, `Tool` 等。
        - **`nuro-runtime/`**: 提供 Agent 的执行逻辑，其核心是 `AgentLoop`，负责驱动 `THINK -> ACT -> OBSERVE` 的 Agent 思考-行动循环；内置 Hook 与 Guardrail 扩展点，并支持事件流式输出。
        - **`nuro-llm/`**: 包含 `LlmProvider` 抽象与实现：
            - `MockLlmProvider`：默认使用的模拟 LLM，用于本地开发与示例；
            - `OpenAiLlmProvider`：在 `openai` feature 下启用的 OpenAI Provider（调用 `/v1/chat/completions`）。
        - **`nuro-tools/`**: 包含 `Tool` 抽象、工具执行相关的实现以及工具容器 `ToolBox`。内置 `CalculatorTool` 作为示例。
        - **`nuro-graph/`**: 状态图引擎 `StateGraph` 的最小实现，支持 `GraphStateTrait`、`FnNode`、条件边、结束节点与可选的内存 `Checkpointer`，并提供 `CompiledGraph::resume` 占位接口。
        - **`nuro-memory/`**: 对话记忆与记忆存储抽象，提供 `ConversationMemory`（按条数 FIFO 截断）与 `MemoryStore` 的内存实现，支持基于子串的简单查询。
        - **`nuro-rag/`**: RAG 相关组件：
            - `DocumentIndexer`：遍历本地目录 + 简单 chunk 切分并写入向量存储；
            - `RetrieverTool`：基于关键词匹配的检索工具；
            - `VectorStore`/`Embedder` trait，以及内存实现 `InMemoryVectorStore`（Jaccard/关键词得分）和占位实现 `NoopVectorStore`/`NoopEmbedder`。
        - **`nuro-mcp/`**: MCP 协议的简化实现：
            - `McpServer`：支持通过 STDIN/STDOUT 或自定义 IO 通道运行简化版 JSON-RPC（`list_tools` / `call_tool`）；
            - `McpClient`：基于任意异步 IO 通道的最小客户端实现。
        - **`nuro-a2a/`**: A2A 协议实现：
            - `A2aServer`：基于 axum 的 HTTP + SSE 服务，提供 `/.well-known/agent.json`、`POST /tasks` 与 `GET /tasks/:id/stream`；
            - `A2aClient`：基于 HTTP 的客户端，支持 `discover` / `send_task` / `subscribe_task`。
        - **`nuro-macros/`**: 占位用过程宏 crate，提供 no-op 的 `#[derive(Tool)]` 与 `#[agent]` 宏（当前不生成代码，仅保证 API 形状稳定）。
        - **`nuro-server/`**: 基于 axum 的最小 HTTP Server 封装，提供 `run_server` 等接口，内置 `/health`、`/v1/chat` 与 `/v1/chat/stream`（SSE）路由，并通过 `tracing` 输出结构化日志。
    - **`examples/`**: 存放使用 Nuro SDK 构建的示例应用。
        - **`simple_chatbot/`**: 命令行聊天机器人，演示如何将 `AgentLoop`、`MockLlmProvider` 和 `CalculatorTool` 组合在一起工作。
        - **`graph_demo/`**: 使用 `StateGraph` 与 `FnNode` 展示条件路由的最小示例。
        - **`http_server/`**: 启动基于 `nuro-server` 的 HTTP 服务示例，暴露 `/health`、`/v1/chat` 与 `/v1/chat/stream` 接口。
        - **`mcp_demo/`**: 在同一进程内启动 MCP Server + Client，通过内存通道演示 `list_tools` 与 `call_tool`。
        - **`a2a_demo/`**: 启动 A2A Server + Client，演示通过 HTTP + SSE 进行 Agent 发现与任务调用。

## 如何构建

项目依赖 Rust 2024 Edition 和 Cargo。请确保已安装最新版本的 Rust 工具链。

1. 克隆或下载本代码库。
2. 进入 `nuro` 目录：
   ```bash
   cd nuro
   ```
3. 执行构建命令：
   ```bash
   cargo build
   ```
   Cargo 将会自动下载所有依赖（如 `tokio`, `serde`, `meval`, `axum` 等）并编译所有 crate。如果网络受限，请确保依赖项已通过其他方式缓存。

> 注意：默认构建不会启用 `openai` feature，也不会访问任何在线 LLM 服务。所有示例默认使用 `MockLlmProvider`。

## 示例运行方式

MVP 包含一个名为 `simple_chatbot` 的示例，它启动一个交互式命令行程序，让您可以与一个由 `MockLlmProvider` 驱动的 Agent 对话。

在 `nuro` 根目录下执行：

```bash
cargo run -p simple_chatbot
```

程序启动后，您可以与机器人进行交互：

- **普通输入**: 输入任意文本，机器人会 "复述" 您的内容。
  ```text
  You: hello world
  Assistant: Echo: hello world
  ```
- **工具调用**: 输入以 `calc:` 开头的表达式，机器人会调用 `CalculatorTool` 并返回计算结果。
  ```text
  You: calc: 1 + 2 * (3 + 1)
  Assistant: 9
  ```

要退出示例，请输入 `quit`。

### 更多示例

除了 `simple_chatbot`，还可以运行以下示例：

- `graph_demo`：演示 StateGraph 条件路由 + FnNode 的最小用法。

  ```bash
  cargo run -p graph_demo

  # 显式指定分支
  cargo run -p graph_demo -- right
  ```

- `http_server`：启动一个最小 HTTP 服务，提供 `/health`、`POST /v1/chat` 与 `POST /v1/chat/stream` 接口。

  ```bash
  # 默认监听 127.0.0.1:3000
  cargo run -p http_server

  # 自定义监听地址
  NURO_HTTP_ADDR=0.0.0.0:3000 cargo run -p http_server
  ```

- `mcp_demo`：在同进程内使用内存通道启动 MCP Server + Client，演示工具注册与调用。

  ```bash
  cargo run -p mcp_demo
  ```

- `a2a_demo`：启动 A2A Server + Client，演示通过 HTTP + SSE 进行 Agent 发现与任务执行。

  ```bash
  cargo run -p a2a_demo
  ```

## 高级用法与 OpenAI 支持

如果希望接入真实的 OpenAI LLM，而不是默认的 `MockLlmProvider`，可以参考
`docs/QUICKSTART-ADVANCED.md` 中的说明，启用 `openai` feature 并配置
`OPENAI_API_KEY` 等环境变量。
