<!-- BLOCK_1 | doxcnQmRRI1gnjLQ2nwURprwvvb -->
## 一、项目定位与设计哲学<!-- 标题序号: 1 --><!-- END_BLOCK_1 -->

<!-- BLOCK_2 | doxcnZyQi5OKLquUJ1ej0FCXP8c -->
### 1.1 项目定位<!-- END_BLOCK_2 -->

<!-- BLOCK_3 | doxcn9SIiK56AtW0kWLQjM2ipkc -->
**Nuro** 是一个用 Rust 构建的通用 AI Agent 开发框架，对标 LangGraph（图编排）、Google ADK（事件驱动 + 多 Agent 协作）、Claude Agent SDK（Harness + Agent Loop），但充分利用 Rust 的零成本抽象、内存安全、异步并发和类型系统优势，为开发者提供：
<!-- END_BLOCK_3 -->

<!-- BLOCK_4 | doxcn2Kl9fz6iuP3ecpiSazrgsd -->
- 高性能、低延迟的 Agent 运行时
<!-- END_BLOCK_4 -->

<!-- BLOCK_5 | doxcnxlTRj8wrwNcd0k49VuZjlh -->
- 类型安全的图（Graph）编排引擎
<!-- END_BLOCK_5 -->

<!-- BLOCK_6 | doxcnak2G5Ryedyxk2kTlgmhoUc -->
- 原生支持 MCP（Model Context Protocol）和 A2A（Agent2Agent Protocol）
<!-- END_BLOCK_6 -->

<!-- BLOCK_7 | doxcnqXTkk4nLsG0EioMf4vpsqb -->
- 可插拔的 LLM 后端、Tool 系统和存储层
<!-- END_BLOCK_7 -->

<!-- BLOCK_8 | doxcnHU6GdtHa0AvutslSmTdpOg -->
- 从单 Agent 到多 Agent 集群的全场景覆盖
<!-- END_BLOCK_8 -->

<!-- BLOCK_9 | doxcnECtoDSonuSIm8uEUhcfseg -->
### 1.2 设计哲学<!-- END_BLOCK_9 -->

<!-- BLOCK_10 | doxcnYkWOTZScOG5ScNjyif5Cch -->
<table col-widths="365,365">
    <tr>
        <td>原则</td>
        <td>说明</td>
    </tr>
    <tr>
        <td>**Rust-Native**</td>
        <td>不是 Python 绑定的 wrapper，所有核心逻辑纯 Rust 实现，充分利用 trait 系统和 async/await</td>
    </tr>
    <tr>
        <td>**Graph-First**</td>
        <td>以有向图为核心编排模型，节点 = 计算单元，边 = 控制流/数据流，支持条件边和循环</td>
    </tr>
    <tr>
        <td>**Event-Driven**</td>
        <td>参考 Google ADK 的事件循环架构，所有交互（LLM call、tool call、human input）统一为事件</td>
    </tr>
    <tr>
        <td>**Protocol-Native**</td>
        <td>MCP 和 A2A 作为一等公民内置，而非事后集成</td>
    </tr>
    <tr>
        <td>**Zero-Copy Streaming**</td>
        <td>SSE / WebSocket 流式输出使用 Rust 的零拷贝技术，最小化延迟</td>
    </tr>
    <tr>
        <td>**Composable**</td>
        <td>每一层都可独立使用，Agent 可以作为 Tool、Tool 可以是另一个 Agent</td>
    </tr>
</table>
<!-- END_BLOCK_10 -->

<!-- BLOCK_11 | doxcnHOeSmsjXPk2GkNsOqERJ1e -->
---
<!-- END_BLOCK_11 -->

<!-- BLOCK_12 | doxcn1Bzfbjv8fRfgiixnalwrhg -->
## 二、整体架构概览<!-- 标题序号: 2 --><!-- END_BLOCK_12 -->

<!-- BLOCK_13 | doxcnzISUJBx6QncyaCHnHVSktf -->
---
<!-- END_BLOCK_13 -->

<!-- BLOCK_14 | doxcngwpisSUgg7RNk7qDj937qd -->
## 三、Crate 拆分与依赖关系<!-- 标题序号: 3 --><!-- END_BLOCK_14 -->

<!-- BLOCK_15 | doxcn5QVQVwdYnm1scSaJZo67Pb -->
SDK 采用 Cargo Workspace 的 monorepo 结构，按关注点拆分为独立 crate，用户可按需引入：
<!-- END_BLOCK_15 -->

<!-- BLOCK_16 | doxcnx25n5P0Xekv9bef7WtMRec -->
```
nuro/
├── Cargo.toml                    # workspace root
├── crates/
│   ├── nuro-core/           # 核心 trait、类型定义、Event 模型
│   ├── nuro-graph/          # 图编排引擎（StateGraph）
│   ├── nuro-runtime/        # Agent 运行时（AgentLoop、执行器）
│   ├── nuro-llm/            # LLM Provider 抽象 + 主流实现
│   ├── nuro-tools/          # Tool 系统 + 内置工具集
│   ├── nuro-memory/         # Memory 系统（短期/长期/向量）
│   ├── nuro-mcp/            # MCP 协议实现（Server + Client）
│   ├── nuro-a2a/            # A2A 协议实现（Server + Client）
│   ├── nuro-transport/      # 传输层（HTTP/WS/SSE/gRPC/stdio）
│   ├── nuro-guardrail/      # 安全护栏系统
│   ├── nuro-rag/            # RAG 引擎（可选）
│   ├── nuro-macros/         # proc-macro 工具宏
│   └── nuro-server/         # 开箱即用的 Agent 服务器
├── examples/
│   ├── simple_chatbot/
│   ├── react_agent/
│   ├── multi_agent_graph/
│   ├── mcp_tool_server/
│   └── a2a_collaboration/
└── tests/
```
<!-- END_BLOCK_16 -->

<!-- BLOCK_17 | doxcn4bfCZTeFG475K4Vn1YeEUb -->
### Crate 依赖拓扑<!-- 标题序号: 3.1 --><!-- END_BLOCK_17 -->

<!-- BLOCK_18 | doxcnnyPIClJO4adrfWykb2B1od -->
---
<!-- END_BLOCK_18 -->

<!-- BLOCK_19 | doxcn3fCOldkdlQMWRcV7Hw5Dvg -->
## 四、核心模块详细设计<!-- 标题序号: 4 --><!-- END_BLOCK_19 -->

<!-- BLOCK_20 | doxcnt5w5b85SpmiTtsbqrnewud -->
### 4.1 nuro-core — 核心类型与 Trait<!-- END_BLOCK_20 -->

<!-- BLOCK_21 | doxcnVMHUwAAsHcWtTUFhhaUbYc -->
这是整个 SDK 的类型基础，定义所有核心 trait 和数据结构。
<!-- END_BLOCK_21 -->

<!-- BLOCK_22 | doxcnA3uUlgXcysmV504lI9qlCc -->
#### Message 统一消息模型<!-- 标题序号: 4.1 --><!-- END_BLOCK_22 -->

<!-- BLOCK_23 | doxcnFCcbaFHGb8NoZWgmDG2xmb -->
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: Vec<ContentBlock>,
    pub metadata: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentBlock {
    Text(String),
    Image { url: String, media_type: String },
    ToolUse { id: String, name: String, input: Value },
    ToolResult { tool_use_id: String, content: Value, is_error: bool },
}
```
<!-- END_BLOCK_23 -->

<!-- BLOCK_24 | doxcnzo8vnOk0uhZZjzhLwl1Qkg -->
#### Event 事件模型 — 所有交互的统一表示<!-- 标题序号: 4.2 --><!-- END_BLOCK_24 -->

<!-- BLOCK_25 | doxcnlnG5mJF7klDgrEzTlU656e -->
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub timestamp: DateTime<Utc>,
    pub kind: EventKind,
    pub agent_id: AgentId,
    pub metadata: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventKind {
    // Agent 生命周期
    AgentStart { input: Value },
    AgentEnd { output: Value },
    AgentError { error: AgentError },
    
    // LLM 交互
    LlmRequest { messages: Vec<Message>, config: LlmConfig },
    LlmResponseStart,
    LlmResponseDelta { delta: ContentBlock },
    LlmResponseEnd { message: Message, usage: TokenUsage },
    
    // Tool 调用
    ToolCallStart { tool_name: String, input: Value },
    ToolCallEnd { tool_name: String, output: Value },
    ToolCallError { tool_name: String, error: String },
    
    // 状态管理
    StateUpdate { key: String, old_value: Option<Value>, new_value: Value },
    CheckpointSaved { checkpoint_id: String },
    
    // Human-in-the-loop
    HumanInputRequest { prompt: String, options: Vec<String> },
    HumanInputResponse { input: String },
    
    // 多 Agent
    SubAgentSpawn { child_agent_id: AgentId, task: String },
    SubAgentComplete { child_agent_id: AgentId, result: Value },
    AgentHandoff { from: AgentId, to: AgentId, context: Value },
    
    // 自定义事件
    Custom { name: String, payload: Value },
}
```
<!-- END_BLOCK_25 -->

<!-- BLOCK_26 | doxcn4FSz2lEjbG6nrNQlgEyczc -->
#### 核心 Trait 定义<!-- 标题序号: 4.3 --><!-- END_BLOCK_26 -->

<!-- BLOCK_27 | doxcnRgVGNRnnYvidDO3rwD9PCe -->
```rust
/// Agent 的核心 trait — 所有 Agent 必须实现
#[async_trait]
pub trait Agent: Send + Sync {
    fn id(&self) -> &AgentId;
    fn descriptor(&self) -> AgentDescriptor;
    
    async fn invoke(
        &self, input: AgentInput, ctx: &mut AgentContext
    ) -> Result<AgentOutput>;
    
    fn stream(
        &self, input: AgentInput, ctx: &mut AgentContext
    ) -> Pin<Box<dyn Stream<Item = Result<Event>> + Send + '_>>;
}

/// LLM Provider — 统一的大模型接口
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse>;
    
    fn stream_generate(
        &self, request: LlmRequest
    ) -> Pin<Box<dyn Stream<Item = Result<LlmStreamEvent>> + Send + '_>>;
    
    fn model_info(&self) -> ModelInfo;
}

/// Tool — 工具的核心 trait
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters_schema(&self) -> Value;
    
    async fn execute(
        &self, input: Value, ctx: &ToolContext
    ) -> Result<ToolOutput>;
}

/// Memory — 记忆系统接口
#[async_trait]
pub trait MemoryStore: Send + Sync {
    async fn add(&self, key: &str, entry: MemoryEntry) -> Result<()>;
    async fn query(&self, query: &str, limit: usize) -> Result<Vec<MemoryEntry>>;
    async fn get_conversation(&self, session_id: &str) -> Result<Vec<Message>>;
    async fn save_conversation(
        &self, session_id: &str, messages: &[Message]
    ) -> Result<()>;
}

/// StateStore — 图状态的持久化
#[async_trait]
pub trait StateStore: Send + Sync {
    async fn save_checkpoint(
        &self, thread_id: &str, state: &GraphState
    ) -> Result<CheckpointId>;
    async fn load_checkpoint(
        &self, checkpoint_id: &CheckpointId
    ) -> Result<GraphState>;
    async fn list_checkpoints(
        &self, thread_id: &str
    ) -> Result<Vec<CheckpointMeta>>;
}

/// Guardrail — 安全护栏
#[async_trait]
pub trait Guardrail: Send + Sync {
    async fn check_input(&self, input: &AgentInput) -> Result<GuardrailResult>;
    async fn check_output(&self, output: &AgentOutput) -> Result<GuardrailResult>;
}
```
<!-- END_BLOCK_27 -->

<!-- BLOCK_28 | doxcn1qXT8zud5GdIsje8mUUUhg -->
---
<!-- END_BLOCK_28 -->

<!-- BLOCK_29 | doxcnfFovLWhngtlro3uPaTf4Fd -->
### 4.2 nuro-graph — 图编排引擎<!-- END_BLOCK_29 -->

<!-- BLOCK_30 | doxcneHaduKA2TSX3S2wYui7G7c -->
这是 SDK 的核心差异化能力，对标 LangGraph 但提供编译期类型安全。
<!-- END_BLOCK_30 -->

<!-- BLOCK_31 | doxcnqEAWiA798nWzRnbXEP2qJf -->
#### 类型安全的 StateGraph 定义<!-- 标题序号: 4.4 --><!-- END_BLOCK_31 -->

<!-- BLOCK_32 | doxcnZ7vZDWuxJE5bGaO67LPIjb -->
```rust
/// 图状态 — 使用 derive macro 自动生成 Reducer
#[derive(Debug, Clone, GraphState)]
pub struct MyState {
    #[reducer(append)]
    pub messages: Vec<Message>,
    
    #[reducer(overwrite)]
    pub current_step: String,
    
    #[reducer(custom = "merge_scores")]
    pub scores: HashMap<String, f64>,
}
```
<!-- END_BLOCK_32 -->

<!-- BLOCK_33 | doxcn4hDoHDDVx6wPMJU21M7Nxe -->
#### StateGraph Builder — 编译期类型安全的图构建<!-- 标题序号: 4.5 --><!-- END_BLOCK_33 -->

<!-- BLOCK_34 | doxcnQU7O4mvj5RdCGk9Bfgp61e -->
```rust
pub struct StateGraph<S: GraphStateTrait> {
    nodes: HashMap<NodeId, Box<dyn GraphNode<S>>>,
    edges: Vec<Edge<S>>,
    entry_point: Option<NodeId>,
    finish_points: HashSet<NodeId>,
}

impl<S: GraphStateTrait> StateGraph<S> {
    pub fn new() -> Self { /* ... */ }
    
    /// 添加节点
    pub fn add_node<N: GraphNode<S> + 'static>(
        mut self, name: impl Into<NodeId>, node: N
    ) -> Self { /* ... */ }
    
    /// 添加无条件边
    pub fn add_edge(
        mut self, from: impl Into<NodeId>, to: impl Into<NodeId>
    ) -> Self { /* ... */ }
    
    /// 添加条件边
    pub fn add_conditional_edge<F>(
        mut self,
        from: impl Into<NodeId>,
        router: F,
        routes: HashMap<String, NodeId>,
    ) -> Self
    where F: Fn(&S) -> String + Send + Sync + 'static
    { /* ... */ }
    
    /// 编译图 — 验证完整性
    pub fn compile(self) -> Result<CompiledGraph<S>> {
        self.validate()?;
        Ok(CompiledGraph::new(self))
    }
}

/// 编译后的图 — 可执行
pub struct CompiledGraph<S: GraphStateTrait> { /* ... */ }

impl<S: GraphStateTrait> CompiledGraph<S> {
    pub async fn invoke(&self, input: S) -> Result<S> { /* ... */ }
    
    pub fn stream(
        &self, input: S
    ) -> Pin<Box<dyn Stream<Item = Result<Event>> + Send + '_>> { /* ... */ }
    
    pub async fn resume(
        &self, checkpoint_id: &CheckpointId, input: Option<S::Update>
    ) -> Result<S> { /* ... */ }
    
    pub fn with_checkpointer(
        mut self, store: impl StateStore + 'static
    ) -> Self { /* ... */ }
}
```
<!-- END_BLOCK_34 -->

<!-- BLOCK_35 | doxcnOSqtjhrgFyyhymqPEJs1Ke -->
#### 图节点 Trait 与适配器<!-- 标题序号: 4.6 --><!-- END_BLOCK_35 -->

<!-- BLOCK_36 | doxcnyivPfKjeSZcFzgzY9oeSvg -->
```rust
/// 图节点 Trait
#[async_trait]
pub trait GraphNode<S: GraphStateTrait>: Send + Sync {
    async fn execute(
        &self, state: &S, ctx: &mut NodeContext
    ) -> Result<S::Update>;
}

// Agent 作为图节点
pub struct AgentNode<A: Agent> { agent: A }

// Tool 作为图节点
pub struct ToolNode { tool: Box<dyn Tool> }

// 函数作为图节点（最简方式）
pub struct FnNode<S, F, Fut> { func: F, _phantom: PhantomData<S> }
```
<!-- END_BLOCK_36 -->

<!-- BLOCK_37 | doxcnuh7oOuCCZBHrOYsIwhCfld -->
---
<!-- END_BLOCK_37 -->

<!-- BLOCK_38 | doxcns7ira81dYiqWmslUyHqXNb -->
### 4.3 nuro-runtime — Agent 运行时<!-- END_BLOCK_38 -->

<!-- BLOCK_39 | doxcnGqgEvHYfOKPNgeu6LT09Fh -->
参考 Claude Agent SDK 的 Agent Loop 设计，提供开箱即用的 Agent 执行模式。
<!-- END_BLOCK_39 -->

<!-- BLOCK_40 | doxcnbP68OwuQM2Fx8WmBnAYLAh -->
#### Agent Loop 核心执行循环<!-- 标题序号: 4.7 --><!-- END_BLOCK_40 -->

<!-- BLOCK_41 | doxcn2J0ntFGmGe3GtUfQDnbgTc -->
```rust
pub struct AgentLoopConfig {
    pub max_iterations: usize,
    pub max_tokens_per_turn: usize,
    pub system_prompt: String,
    pub tools: Vec<Arc<dyn Tool>>,
    pub hooks: Vec<Arc<dyn Hook>>,
    pub guardrails: Vec<Arc<dyn Guardrail>>,
    pub memory: Option<Arc<dyn MemoryStore>>,
    pub stop_conditions: Vec<Box<dyn StopCondition>>,
}

pub struct AgentLoop {
    config: AgentLoopConfig,
    llm: Arc<dyn LlmProvider>,
    event_bus: EventBus,
}

impl AgentLoop {
    pub fn builder() -> AgentLoopBuilder { /* ... */ }
    
    pub async fn run(
        &self, input: &str, ctx: &mut AgentContext
    ) -> Result<AgentOutput> {
        let mut messages = self.build_initial_messages(input, ctx).await?;
        
        for iteration in 0..self.config.max_iterations {
            // 1. THINK — 调用 LLM
            let response = self.llm.generate(LlmRequest {
                messages: messages.clone(),
                tools: self.get_tool_schemas(),
                config: self.get_llm_config(),
            }).await?;
            
            messages.push(response.message.clone());
            let tool_calls = response.message.extract_tool_calls();
            
            if tool_calls.is_empty() {
                return Ok(AgentOutput::from_message(response.message));
            }
            
            // 2. ACT — 执行 Tool Calls（支持并行）
            let tool_results = self
                .execute_tools_parallel(&tool_calls, ctx).await?;
            
            // 3. OBSERVE — 将结果反馈给 LLM
            for result in tool_results {
                messages.push(Message::tool_result(result));
            }
            
            // 4. Hook 注入反馈
            for hook in &self.config.hooks {
                if let Some(feedback) = hook
                    .after_tool_call(&messages, ctx).await? {
                    messages.push(Message::system(feedback));
                }
            }
        }
        Err(AgentError::MaxIterationsExceeded(
            self.config.max_iterations
        ))
    }
}
```
<!-- END_BLOCK_41 -->

<!-- BLOCK_42 | doxcn0oV1pjQnVtgC7JsDB4f54e -->
#### 预置 Agent 模式<!-- 标题序号: 4.8 --><!-- END_BLOCK_42 -->

<!-- BLOCK_43 | doxcnYrsUsNdHZsFOFZvLBAgbsc -->
```rust
/// ReAct Agent — Reason + Act 模式
pub struct ReActAgent {
    loop_: AgentLoop,
    thought_format: ThoughtFormat, // CoT / XML / JSON
}

/// Plan-and-Execute Agent — 先规划后执行
pub struct PlanExecuteAgent {
    planner: Arc<dyn LlmProvider>,
    executor: AgentLoop,
    replanner: Option<Arc<dyn LlmProvider>>,
}

/// Multi-Agent Coordinator — 多 Agent 协调器
pub struct CoordinatorAgent {
    agents: HashMap<String, Arc<dyn Agent>>,
    router: Arc<dyn AgentRouter>,
    strategy: CoordinationStrategy,
}

pub enum CoordinationStrategy {
    Sequential,
    Parallel { max_concurrency: usize },
    Hierarchical,
    LlmRouted,
}
```
<!-- END_BLOCK_43 -->

<!-- BLOCK_44 | doxcnUDdtiT5CFM6RPLUeOR0V3b -->
#### Hook 系统 — 参考 Claude Agent SDK<!-- 标题序号: 4.9 --><!-- END_BLOCK_44 -->

<!-- BLOCK_45 | doxcnUv95GJfFrLLwmnzkD0VGAe -->
```rust
#[async_trait]
pub trait Hook: Send + Sync {
    async fn before_agent_start(
        &self, _input: &AgentInput
    ) -> Result<Option<String>> { Ok(None) }
    
    async fn before_llm_call(
        &self, _messages: &[Message]
    ) -> Result<Option<Vec<Message>>> { Ok(None) }
    
    async fn before_tool_call(
        &self, _call: &ToolCall
    ) -> Result<HookAction> { Ok(HookAction::Continue) }
    
    async fn after_tool_call(
        &self, _messages: &[Message], _ctx: &AgentContext
    ) -> Result<Option<String>> { Ok(None) }
    
    async fn after_agent_end(
        &self, _output: &AgentOutput
    ) -> Result<()> { Ok(()) }
}

pub enum HookAction {
    Continue,
    Skip,
    Replace(ToolResult),
    Reject(String),
    Feedback(String),
}
```
<!-- END_BLOCK_45 -->

<!-- BLOCK_46 | doxcnTHVeaItAHjxmvHVXjflHjc -->
---
<!-- END_BLOCK_46 -->

<!-- BLOCK_47 | doxcn6Ux0bzhHJfHiAXmTruJCnd -->
### 4.4 nuro-llm — LLM Provider 抽象<!-- END_BLOCK_47 -->

<!-- BLOCK_48 | doxcnSIf95oJs0SyopwoN69mvVd -->
```rust
pub struct LlmRequest {
    pub messages: Vec<Message>,
    pub tools: Option<Vec<ToolSchema>>,
    pub config: LlmConfig,
}

pub struct LlmConfig {
    pub model: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<usize>,
    pub top_p: Option<f32>,
    pub stop_sequences: Option<Vec<String>>,
    pub response_format: Option<ResponseFormat>,
    pub extra: HashMap<String, Value>,
}

pub enum LlmStreamEvent {
    Start { model: String },
    Delta { content: ContentBlock },
    ToolCallStart { id: String, name: String },
    ToolCallDelta { id: String, input_delta: String },
    Usage { usage: TokenUsage },
    End { finish_reason: FinishReason },
}
```
<!-- END_BLOCK_48 -->

<!-- BLOCK_49 | doxcnSZsPcRDls7NqLV9dqmVizg -->
内置 Provider 实现（通过 feature flag 控制）：
<!-- END_BLOCK_49 -->

<!-- BLOCK_50 | doxcn9LYc7VTeh5PdshsgXjvWPd -->
<table col-widths="244,244,244">
    <tr>
        <td>Provider</td>
        <td>Feature</td>
        <td>说明</td>
    </tr>
    <tr>
        <td>`OpenAiProvider`</td>
        <td>`openai`</td>
        <td>OpenAI / Azure OpenAI</td>
    </tr>
    <tr>
        <td>`AnthropicProvider`</td>
        <td>`anthropic`</td>
        <td>Claude 系列</td>
    </tr>
    <tr>
        <td>`GeminiProvider`</td>
        <td>`gemini`</td>
        <td>Google Gemini</td>
    </tr>
    <tr>
        <td>`OllamaProvider`</td>
        <td>`ollama`</td>
        <td>本地 Ollama</td>
    </tr>
    <tr>
        <td>`OpenAiCompatibleProvider`</td>
        <td>`openai`</td>
        <td>vLLM / DeepSeek 等兼容接口</td>
    </tr>
</table>
<!-- END_BLOCK_50 -->

<!-- BLOCK_51 | doxcnKz7uACwKdnZvXIImqrT9Ib -->
---
<!-- END_BLOCK_51 -->

<!-- BLOCK_52 | doxcn4bgBvpiv4STpLQzQ9EcZ5g -->
### 4.5 nuro-tools — 工具系统<!-- END_BLOCK_52 -->

<!-- BLOCK_53 | doxcnz2CQq8YCArwWM36VX85TOg -->
#### 使用 derive macro 定义 Tool<!-- 标题序号: 4.10 --><!-- END_BLOCK_53 -->

<!-- BLOCK_54 | doxcnjx3aM08Nm1WBXEbRap4Ahg -->
```rust
#[derive(Tool)]
#[tool(
    name = "web_search",
    description = "Search the web for information"
)]
pub struct WebSearchTool {
    client: reqwest::Client,
    api_key: String,
}

#[derive(ToolArgs, Serialize, Deserialize, JsonSchema)]
pub struct WebSearchArgs {
    #[schemars(description = "The search query string")]
    pub query: String,
    
    #[schemars(description = "Max results, defaults to 5")]
    #[serde(default = "default_max_results")]
    pub max_results: usize,
}

#[async_trait]
impl ToolExecute for WebSearchTool {
    type Args = WebSearchArgs;
    
    async fn execute(
        &self, args: Self::Args, ctx: &ToolContext
    ) -> Result<ToolOutput> {
        // 实际搜索逻辑...
        Ok(ToolOutput::json(results))
    }
}
```
<!-- END_BLOCK_54 -->

<!-- BLOCK_55 | doxcnnmB3ZpchFXWvWzIcEyxA3d -->
#### 快捷方式：用闭包定义简单 Tool<!-- 标题序号: 4.11 --><!-- END_BLOCK_55 -->

<!-- BLOCK_56 | doxcnQ2J2xKVbfvZZYsMM1Xj7Jh -->
```rust
let calculator = FnTool::new(
    "calculator",
    "Evaluate a mathematical expression",
    schema,
    |input: Value, _ctx: &ToolContext| async move {
        let expr = input["expression"].as_str().unwrap();
        let result = meval::eval_str(expr)?;
        Ok(ToolOutput::text(format!("{}", result)))
    },
);
```
<!-- END_BLOCK_56 -->

<!-- BLOCK_57 | doxcnH6MNZ9CmUiD2YuPXOYhZNg -->
#### ToolBox — 工具集管理器<!-- 标题序号: 4.12 --><!-- END_BLOCK_57 -->

<!-- BLOCK_58 | doxcnWzhPIjgfFfQIjgKYtWtfBd -->
```rust
pub struct ToolBox { tools: HashMap<String, Arc<dyn Tool>> }

impl ToolBox {
    pub fn new() -> Self { /* ... */ }
    pub fn add<T: Tool + 'static>(mut self, tool: T) -> Self { /* ... */ }
    
    /// 从 MCP Server 加载工具
    pub async fn add_mcp_server(
        mut self, transport: impl McpTransport
    ) -> Result<Self> { /* ... */ }
    
    /// Agent-as-Tool 模式
    pub fn add_agent_as_tool<A: Agent + 'static>(
        mut self, agent: A, description: &str
    ) -> Self { /* ... */ }
}
```
<!-- END_BLOCK_58 -->

<!-- BLOCK_59 | doxcn3x6FfzfQl3uN2MIzRC0abd -->
---
<!-- END_BLOCK_59 -->

<!-- BLOCK_60 | doxcnpJDVJV1uGbQHzj0FujQNqg -->
### 4.6 nuro-mcp — MCP 协议实现<!-- END_BLOCK_60 -->

<!-- BLOCK_61 | doxcnAI8H57EoVFqyZBhWoBDykd -->
```rust
pub struct McpServer {
    info: ServerInfo,
    tools: Vec<Arc<dyn Tool>>,
    resources: Vec<Arc<dyn McpResource>>,
    prompts: Vec<Arc<dyn McpPrompt>>,
}

impl McpServer {
    pub fn builder(name: &str, version: &str) -> McpServerBuilder { /* ... */ }
    
    pub async fn serve_stdio(self) -> Result<()> { /* ... */ }
    pub async fn serve_sse(self, addr: SocketAddr) -> Result<()> { /* ... */ }
    pub async fn serve_websocket(self, addr: SocketAddr) -> Result<()> { /* ... */ }
    pub async fn serve_http(self, addr: SocketAddr) -> Result<()> { /* ... */ }
}

pub struct McpClient { /* ... */ }

impl McpClient {
    pub async fn connect_stdio(cmd: &str, args: &[&str]) -> Result<Self> { /* ... */ }
    pub async fn connect_sse(url: &str) -> Result<Self> { /* ... */ }
    pub async fn connect_websocket(url: &str) -> Result<Self> { /* ... */ }
    
    pub async fn list_tools(&self) -> Result<Vec<ToolSchema>> { /* ... */ }
    pub async fn call_tool(&self, name: &str, args: Value) -> Result<Value> { /* ... */ }
}
```
<!-- END_BLOCK_61 -->

<!-- BLOCK_62 | doxcn35jRQ6rJZhLqcqMpicOWUc -->
---
<!-- END_BLOCK_62 -->

<!-- BLOCK_63 | doxcnmU2hyA2Ds4ntAmCfZHtn7e -->
### 4.7 nuro-a2a — A2A 协议实现<!-- END_BLOCK_63 -->

<!-- BLOCK_64 | doxcnIJFz7HAlC3UcWVrC4VSXoh -->
```rust
/// Agent Card — A2A 协议的服务发现
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCard {
    pub name: String,
    pub description: String,
    pub url: String,
    pub version: String,
    pub capabilities: AgentCapabilities,
    pub skills: Vec<AgentSkill>,
    pub authentication: AuthenticationInfo,
}

/// A2A Task 生命周期
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Submitted,
    Working,
    InputRequired { message: String },
    Completed,
    Failed { error: String },
    Canceled,
}

impl A2aServer {
    pub fn from_agent(agent: impl Agent + 'static) -> Self { /* ... */ }
    pub async fn serve(self, addr: SocketAddr) -> Result<()> { /* ... */ }
}

impl A2aClient {
    pub async fn discover(url: &str) -> Result<AgentCard> { /* ... */ }
    pub async fn send_task(&self, request: TaskRequest) -> Result<Task> { /* ... */ }
    pub fn subscribe_task(
        &self, task_id: &TaskId
    ) -> Pin<Box<dyn Stream<Item = Result<TaskStatusUpdate>> + Send + '_>> { /* ... */ }
}
```
<!-- END_BLOCK_64 -->

<!-- BLOCK_65 | doxcnwryzMWzl4sKAL7uO4Tknnc -->
---
<!-- END_BLOCK_65 -->

<!-- BLOCK_66 | doxcnnmEbCDNgoeB0tuaGOrsF3g -->
### 4.8 nuro-memory — 记忆系统<!-- END_BLOCK_66 -->

<!-- BLOCK_67 | doxcnPpCPxIwb60EvJ9gAov4Q5b -->
```rust
/// 短期记忆 — 对话上下文窗口管理
pub struct ConversationMemory {
    max_messages: usize,
    max_tokens: usize,
    strategy: TruncationStrategy, // FIFO / Summary / Sliding Window
}

/// 长期记忆 — 向量检索增强
pub struct VectorMemory {
    store: Box<dyn VectorStore>,
    embedder: Box<dyn Embedder>,
    similarity_threshold: f32,
}

/// 向量存储后端 trait
#[async_trait]
pub trait VectorStore: Send + Sync {
    async fn upsert(&self, entries: &[VectorEntry]) -> Result<()>;
    async fn search(
        &self, query_vector: &[f32], limit: usize, filter: Option<Value>
    ) -> Result<Vec<VectorSearchResult>>;
    async fn delete(&self, ids: &[String]) -> Result<()>;
}

/// 组合记忆
pub struct CompositeMemory {
    conversation: ConversationMemory,
    long_term: Option<VectorMemory>,
    entity_memory: Option<EntityMemory>,
}
```
<!-- END_BLOCK_67 -->

<!-- BLOCK_68 | doxcninSIfXFnqOHs5vNy03UUqc -->
---
<!-- END_BLOCK_68 -->

<!-- BLOCK_69 | doxcn0yllaO8unYOvEoBnIBAeVe -->
### 4.9 nuro-macros — 过程宏<!-- END_BLOCK_69 -->

<!-- BLOCK_70 | doxcn9aOD1I1ozpUObRqbgYymbe -->
```rust
/// #[derive(GraphState)] — 自动生成 GraphStateTrait 实现
#[proc_macro_derive(GraphState, attributes(reducer))]

/// #[derive(Tool)] — 自动实现 Tool trait
#[proc_macro_derive(Tool, attributes(tool))]

/// #[derive(ToolArgs)] — 从 struct 生成 JSON Schema
#[proc_macro_derive(ToolArgs, attributes(tool_arg))]

/// #[agent] — 属性宏，简化 Agent 定义
#[proc_macro_attribute]
pub fn agent(attr: TokenStream, item: TokenStream) -> TokenStream
```
<!-- END_BLOCK_70 -->

<!-- BLOCK_71 | doxcnHEViimXSyNIuAnHJS4kNPg -->
---
<!-- END_BLOCK_71 -->

<!-- BLOCK_72 | doxcnBsPpNOa1QlGS6WiKwjdVob -->
## 五、关键流程设计<!-- 标题序号: 5 --><!-- END_BLOCK_72 -->

<!-- BLOCK_73 | doxcnBK1J5JU3gTfoHD2N6Eo0bf -->
### 5.1 Single Agent ReAct 流程<!-- END_BLOCK_73 -->

<!-- BLOCK_74 | doxcnKZ89a7ZSVdf809ymqC6Jng -->
### 5.2 Multi-Agent Graph 流程<!-- END_BLOCK_74 -->

<!-- BLOCK_75 | doxcnAV0V6SU1sZF71Sbdokj8Mg -->
---
<!-- END_BLOCK_75 -->

<!-- BLOCK_76 | doxcnFeSuu3l3pxKRdeS1X60Wkb -->
## 六、核心技术选型<!-- 标题序号: 6 --><!-- END_BLOCK_76 -->

<!-- BLOCK_77 | doxcnQ8ZCeVpAQPMcJhv59wuElg -->
<table col-widths="244,244,244">
    <tr>
        <td>领域</td>
        <td>技术选择</td>
        <td>理由</td>
    </tr>
    <tr>
        <td>异步运行时</td>
        <td>`tokio`</td>
        <td>Rust 异步生态事实标准</td>
    </tr>
    <tr>
        <td>HTTP 客户端</td>
        <td>`reqwest`</td>
        <td>基于 tokio，支持 streaming</td>
    </tr>
    <tr>
        <td>HTTP 服务器</td>
        <td>`axum`</td>
        <td>tower middleware，性能优秀</td>
    </tr>
    <tr>
        <td>序列化</td>
        <td>`serde` + `serde_json`</td>
        <td>Rust 序列化事实标准</td>
    </tr>
    <tr>
        <td>JSON Schema</td>
        <td>`schemars`</td>
        <td>从 Rust struct 自动生成</td>
    </tr>
    <tr>
        <td>错误处理</td>
        <td>`thiserror` + `anyhow`</td>
        <td>库用 thiserror，应用层用 anyhow</td>
    </tr>
    <tr>
        <td>日志/追踪</td>
        <td>`tracing`</td>
        <td>结构化日志 + OpenTelemetry</td>
    </tr>
    <tr>
        <td>SSE</td>
        <td>`axum` + `tokio-stream`</td>
        <td>原生流式支持</td>
    </tr>
    <tr>
        <td>WebSocket</td>
        <td>`tokio-tungstenite`</td>
        <td>成熟实现</td>
    </tr>
    <tr>
        <td>gRPC</td>
        <td>`tonic`</td>
        <td>高性能 gRPC</td>
    </tr>
    <tr>
        <td>proc-macro</td>
        <td>`syn` + `quote` + `proc-macro2`</td>
        <td>标准工具链</td>
    </tr>
    <tr>
        <td>数据库</td>
        <td>`sqlx`（可选）</td>
        <td>异步 SQL，checkpoint 持久化</td>
    </tr>
    <tr>
        <td>向量搜索</td>
        <td>`qdrant-client` / `sqlite-vec`</td>
        <td>可选 feature</td>
    </tr>
    <tr>
        <td>配置</td>
        <td>`config` + `serde`</td>
        <td>分层配置</td>
    </tr>
</table>
<!-- END_BLOCK_77 -->

<!-- BLOCK_78 | doxcnQ3UVoiofjfPXwgK3553E3d -->
---
<!-- END_BLOCK_78 -->

<!-- BLOCK_79 | doxcnQ726t7Ik3c3JEpL4PuU34c -->
## 七、Feature Flags 设计<!-- 标题序号: 7 --><!-- END_BLOCK_79 -->

<!-- BLOCK_80 | doxcn02i7BT2jRi2lINjyROIMeg -->
```
[features]
default = ["openai", "tools-builtin"]

# LLM Providers
openai = ["dep:reqwest"]
anthropic = ["dep:reqwest"]
gemini = ["dep:reqwest"]
ollama = ["dep:reqwest"]

# Protocol
mcp = ["dep:nuro-mcp"]
a2a = ["dep:nuro-a2a"]

# Memory backends
memory-sqlite = ["dep:sqlx", "dep:sqlite-vec"]
memory-qdrant = ["dep:qdrant-client"]

# Built-in tools
tools-builtin = ["tools-web-search", "tools-code-exec", "tools-file-system"]

# Server
server = ["dep:axum", "dep:tower"]
server-grpc = ["dep:tonic"]

# Observability
tracing-otel = ["dep:opentelemetry", "dep:tracing-opentelemetry"]

# WASM support
wasm = ["dep:wasm-bindgen"]

# Full bundle
full = [
    "openai", "anthropic", "gemini", "mcp", "a2a",
    "server", "memory-sqlite", "tools-builtin", "tracing-otel"
]
```
<!-- END_BLOCK_80 -->

<!-- BLOCK_81 | doxcnsQxrsWuq7D8O7DLwpDUBxc -->
---
<!-- END_BLOCK_81 -->

<!-- BLOCK_82 | doxcnGDx0oK155eWSRl0zQ5fDUb -->
## 八、使用示例<!-- 标题序号: 8 --><!-- END_BLOCK_82 -->

<!-- BLOCK_83 | doxcnZJ4zL69MmDbrQXqi6re4ee -->
### 8.1 最简单的 Agent<!-- END_BLOCK_83 -->

<!-- BLOCK_84 | doxcndBiomteSGnzxrnFOPvO2if -->
```rust
use nuro::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let agent = AgentLoop::builder()
        .llm(OpenAiProvider::new("gpt-4o"))
        .system_prompt("You are a helpful assistant.")
        .tool(WebSearchTool::new("api_key"))
        .tool(CalculatorTool::new())
        .build()?;
    
    let output = agent.invoke("What's the population of Tokyo?").await?;
    println!("{}", output.text());
    Ok(())
}
```
<!-- END_BLOCK_84 -->

<!-- BLOCK_85 | doxcnT3BvMnY8Ugey45r2vOCZpb -->
### 8.2 Graph-Based Multi-Agent<!-- END_BLOCK_85 -->

<!-- BLOCK_86 | doxcnIIuEKwPIiwE7mZvXYNBC1f -->
```rust
use nuro::prelude::*;
use nuro::graph::*;

#[derive(Debug, Clone, GraphState)]
struct TeamState {
    #[reducer(append)]
    messages: Vec<Message>,
    #[reducer(overwrite)]
    next_agent: String,
    #[reducer(append)]
    artifacts: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let researcher = AgentLoop::builder()
        .llm(AnthropicProvider::new("claude-sonnet-4-20250514"))
        .system_prompt("You are a research specialist...")
        .tool(WebSearchTool::new("key"))
        .build()?;
    
    let writer = AgentLoop::builder()
        .llm(OpenAiProvider::new("gpt-4o"))
        .system_prompt("You are a technical writer...")
        .build()?;
    
    let graph = StateGraph::<TeamState>::new()
        .add_node("supervisor", supervisor_node)
        .add_node("researcher", AgentNode::new(researcher))
        .add_node("writer", AgentNode::new(writer))
        .set_entry_point("supervisor")
        .add_conditional_edge(
            "supervisor",
            |state: &TeamState| state.next_agent.clone(),
            routes,
        )
        .add_edge("researcher", "supervisor")
        .add_edge("writer", "supervisor")
        .compile()?
        .with_checkpointer(SqliteStateStore::new("state.db").await?);
    
    let result = graph.invoke(TeamState {
        messages: vec![Message::user("Write a blog post about Rust for AI")],
        next_agent: String::new(),
        artifacts: vec![],
    }).await?;
    
    println!("Artifacts: {:?}", result.artifacts);
    Ok(())
}
```
<!-- END_BLOCK_86 -->

<!-- BLOCK_87 | doxcnINXQUjerlFQAVRCHr6n7Xf -->
### 8.3 MCP Server — 将工具暴露给 Claude Code<!-- END_BLOCK_87 -->

<!-- BLOCK_88 | doxcn9hAimQly1dEX6OY4vxFr0b -->
```rust
use nuro::mcp::*;

#[tokio::main]
async fn main() -> Result<()> {
    McpServer::builder("my-tools", "1.0.0")
        .tool(DatabaseQueryTool::new("postgres://..."))
        .tool(FileSearchTool::new("/workspace"))
        .resource(ConfigResource::new("/etc/app/config.json"))
        .serve_stdio()
        .await?;
    Ok(())
}
```
<!-- END_BLOCK_88 -->

<!-- BLOCK_89 | doxcnzJ6yKd7GNd6F4Sjfb3uGpb -->
### 8.4 A2A 多 Agent 协作<!-- END_BLOCK_89 -->

<!-- BLOCK_90 | doxcnRYQMRLjzOCm90RoDulCNRd -->
```rust
use nuro::a2a::*;

#[tokio::main]
async fn main() -> Result<()> {
    let card = A2aClient::discover("https://agent.example.com").await?;
    
    let agent = AgentLoop::builder()
        .llm(AnthropicProvider::new("claude-sonnet-4-20250514"))
        .tool(A2aAgentTool::from_card(card))
        .build()?;
    
    let result = agent.invoke("Translate this document to Japanese").await?;
    println!("{}", result.text());
    Ok(())
}
```
<!-- END_BLOCK_90 -->

<!-- BLOCK_91 | doxcnRuvFlKOUZvy8cMsSvLsSYe -->
---
<!-- END_BLOCK_91 -->

<!-- BLOCK_92 | doxcnS4ewpw5isuJjx4idSxNcpf -->
## 八-B、完整 Demo 接入示例（多场景）<!-- 标题序号: 9 --><!-- END_BLOCK_92 -->

<!-- BLOCK_93 | doxcn7Wjtn1MSFA33KIUxr9rDLc -->
## 以下提供 **8 个完整可运行的 Demo 示例**，覆盖从单 Agent 到多 Agent、从 MCP/A2A 协议到生产部署的全场景，每个示例包含完整的 `Cargo.toml` 配置、项目结构和可编译代码。<!-- 标题序号: 10 --><!-- END_BLOCK_93 -->

<!-- BLOCK_94 | doxcnZWlqPUzh7v7P1fDY6zbYXb -->
### Demo 1：Conversational Chatbot（对话聊天机器人）<!-- 标题序号: 10.1 --><!-- END_BLOCK_94 -->

<!-- BLOCK_95 | doxcn8LNaVRKxtPw1DvCHz0yWSc -->
**场景**：最基础的交互式 CLI 聊天机器人，带记忆（多轮对话），适合快速验证 SDK 是否跑通。**项目结构**：
<!-- END_BLOCK_95 -->

<!-- BLOCK_96 | doxcnGcM8hzxyqgoNlWs4dMCMHd -->
```
examples/chatbot/
├── Cargo.toml
└── src/
    └── main.rs
```
<!-- END_BLOCK_96 -->

<!-- BLOCK_97 | doxcnZXQ81REXZFmWPL0dsHQMsb -->
**Cargo.toml**：
<!-- END_BLOCK_97 -->

<!-- BLOCK_98 | doxcnQDwbVOtNqMfnF46ykd9SUf -->
```toml
[package]
name = "chatbot-demo"
version = "0.1.0"
edition = "2024"

[dependencies]
nuro = { path = "../../", features = ["openai"] }
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```
<!-- END_BLOCK_98 -->

<!-- BLOCK_99 | doxcnpFypUTX94dja9OUIy7D57f -->
**src/main.rs**：
<!-- END_BLOCK_99 -->

<!-- BLOCK_100 | doxcnUO0nkbC5Np4hrUtSQVHzNb -->
```rust
use anyhow::Result;
use nuro::prelude::*;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化 tracing
    tracing_subscriber::init();

    // 创建带有对话记忆的 Agent
    let agent = AgentLoop::builder()
        .llm(OpenAiProvider::from_env()?)
        .system_prompt(
            "You are a friendly and knowledgeable assistant. \
             Remember the context of our conversation and refer \
             back to previous messages when relevant."
        )
        .memory(ConversationMemory::new()
            .max_messages(50)
            .max_tokens(8000)
            .strategy(TruncationStrategy::SlidingWindow)
        )
        .max_iterations(1) // 纯对话，无 tool 调用
        .build()?;

    // 创建会话上下文
    let mut ctx = AgentContext::new()
        .with_session_id("cli-session-001");

    println!("Nuro Chatbot (type 'quit' to exit)");
    println!("=========================================\n");

    loop {
        print!("You: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        // 流式输出回复
        print!("Assistant: ");
        let mut stream = agent.stream(
            AgentInput::text(input), &mut ctx
        );
        
        while let Some(event) = stream.next().await {
            match event? {
                Event { kind: EventKind::LlmResponseDelta { 
                    delta: ContentBlock::Text(text) 
                }, .. } => {
                    print!("{}", text);
                    io::stdout().flush()?;
                }
                _ => {}
            }
        }
        println!("\n");
    }

    Ok(())
}
```
<!-- END_BLOCK_100 -->

<!-- BLOCK_101 | doxcnoySAVyUAvXaHPaFP0187Kc -->
---
<!-- END_BLOCK_101 -->

<!-- BLOCK_102 | doxcnFNtjzCuJPumYWeZfpXti5d -->
### Demo 2：ReAct Research Agent（研究助手）<!-- 标题序号: 10.2 --><!-- END_BLOCK_102 -->

<!-- BLOCK_103 | doxcnTMI0wVOjAwU2yyUCpX5qIe -->
**场景**：带 Web 搜索 + 文件读写 + 计算器工具的 ReAct Agent，能够自主搜索信息、分析数据、生成报告。**项目结构**：
<!-- END_BLOCK_103 -->

<!-- BLOCK_104 | doxcneGapJA7RaVYjQlPNqpXbed -->
```
examples/research_agent/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── tools/
    │   ├── mod.rs
    │   ├── web_search.rs
    │   ├── file_ops.rs
    │   └── calculator.rs
    └── hooks/
        ├── mod.rs
        └── safety.rs
```
<!-- END_BLOCK_104 -->

<!-- BLOCK_105 | doxcnMHjHXRtgohsZnamoGrmuGc -->
**Cargo.toml**：
<!-- END_BLOCK_105 -->

<!-- BLOCK_106 | doxcna1vvFLvbJaDoVEwQTb7wwh -->
```toml
[package]
name = "research-agent-demo"
version = "0.1.0"
edition = "2024"

[dependencies]
nuro = { path = "../../", features = [
    "anthropic", "tools-builtin", "memory-sqlite"
] }
tokio = { version = "1", features = ["full"] }
anyhow = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing-subscriber = "0.3"
```
<!-- END_BLOCK_106 -->

<!-- BLOCK_107 | doxcnfuIT2Ro4CF8f3lARGeC8Wg -->
**src/main.rs**：
<!-- END_BLOCK_107 -->

<!-- BLOCK_108 | doxcnGsOSWCAq86aB2K0iFZh3jc -->
```rust
use anyhow::Result;
use nuro::prelude::*;
use nuro::memory::VectorMemory;

mod tools;
mod hooks;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // ─── 1. 配置工具集 ───
    let toolbox = ToolBox::new()
        .add(tools::WebSearchTool::new(
            std::env::var("SEARCH_API_KEY")?
        ))
        .add(tools::FileOpsTool::new("./workspace"))
        .add(tools::CalculatorTool::new())
        // 内置 shell 工具（受限模式，只读文件系统）
        .add(BashTool::new()
            .sandbox_mode(SandboxMode::ReadOnly)
            .allowed_commands(vec!["grep", "wc", "head", "tail", "cat"])
        );

    // ─── 2. 配置记忆系统 ───
    let memory = CompositeMemory::new()
        .conversation(ConversationMemory::new()
            .max_tokens(16000)
            .strategy(TruncationStrategy::SlidingWindow)
        )
        .long_term(VectorMemory::new(
            SqliteVectorStore::open("./research_memory.db").await?,
            OpenAiEmbedder::new("text-embedding-3-small"),
        ).similarity_threshold(0.7));

    // ─── 3. 配置安全护栏 ───
    let guardrail = ContentGuardrail::new()
        .block_patterns(vec![
            r"(?i)(password|secret|api.key)",
        ])
        .max_output_tokens(4000);

    // ─── 4. 配置 Hook ───
    let read_before_write_hook = hooks::ReadBeforeWriteHook::new();
    let cost_tracker_hook = hooks::CostTrackerHook::new(0.50); // 最多花 $0.50

    // ─── 5. 构建 ReAct Agent ───
    let agent = ReActAgent::builder()
        .llm(AnthropicProvider::new("claude-sonnet-4-20250514")
            .max_tokens(4096)
            .temperature(0.3)
        )
        .system_prompt(include_str!("../prompts/research_system.txt"))
        .toolbox(toolbox)
        .memory(memory)
        .guardrail(guardrail)
        .hook(read_before_write_hook)
        .hook(cost_tracker_hook)
        .max_iterations(20)
        .thought_format(ThoughtFormat::Xml) // <thinking>...</thinking>
        .build()?;

    // ─── 6. 执行研究任务 ───
    let mut ctx = AgentContext::new()
        .with_session_id("research-001")
        .with_metadata("task_type", "deep_research");

    let task = "\
        Research the current state of Rust async runtimes in 2026. \
        Compare tokio, async-std, smol, and glommio. \
        Create a comparison table and save it to ./workspace/report.md. \
        Include benchmarks if available.";

    println!("Starting research task...\n");

    // 流式执行 + 实时打印事件
    let mut stream = agent.stream(AgentInput::text(task), &mut ctx);
    
    while let Some(event) = stream.next().await {
        match event? {
            Event { kind: EventKind::ToolCallStart { 
                tool_name, .. 
            }, .. } => {
                println!("[Tool] Calling: {}", tool_name);
            }
            Event { kind: EventKind::ToolCallEnd { 
                tool_name, .. 
            }, .. } => {
                println!("[Tool] Completed: {}", tool_name);
            }
            Event { kind: EventKind::LlmResponseEnd { 
                usage, .. 
            }, .. } => {
                println!("[LLM] Tokens: input={}, output={}", 
                    usage.input_tokens, usage.output_tokens);
            }
            Event { kind: EventKind::AgentEnd { output }, .. } => {
                println!("\n=== Research Complete ===");
                println!("{}", output);
            }
            _ => {}
        }
    }

    Ok(())
}
```
<!-- END_BLOCK_108 -->

<!-- BLOCK_109 | doxcnsImwkiJtnxXEDnjiEqbO6f -->
**src/hooks/safety.rs**（ReadBeforeWrite Hook 实现）：
<!-- END_BLOCK_109 -->

<!-- BLOCK_110 | doxcnNlvfe4oJAwuCHJwaTpiJUc -->
```rust
use nuro::prelude::*;

/// 强制 Agent 在写文件之前先读取目标文件
pub struct ReadBeforeWriteHook {
    recent_reads: std::sync::Mutex<std::collections::HashSet<String>>,
}

impl ReadBeforeWriteHook {
    pub fn new() -> Self {
        Self {
            recent_reads: std::sync::Mutex::new(
                std::collections::HashSet::new()
            ),
        }
    }
}

#[async_trait]
impl Hook for ReadBeforeWriteHook {
    async fn before_tool_call(
        &self, call: &ToolCall
    ) -> Result<HookAction> {
        // 如果是写文件操作
        if call.name == "file_write" {
            let path = call.input["path"].as_str()
                .unwrap_or_default();
            let reads = self.recent_reads.lock().unwrap();
            
            if !reads.contains(path) {
                return Ok(HookAction::Feedback(format!(
                    "Before writing to '{}', please read the file first \
                     to understand its current content. Use file_read tool.",
                    path
                )));
            }
        }
        
        // 如果是读文件操作，记录下来
        if call.name == "file_read" {
            let path = call.input["path"].as_str()
                .unwrap_or_default();
            self.recent_reads.lock().unwrap()
                .insert(path.to_string());
        }
        
        Ok(HookAction::Continue)
    }
}
```
<!-- END_BLOCK_110 -->

<!-- BLOCK_111 | doxcnAA7ggd7khkfRRfhYbkBH1c -->
---
<!-- END_BLOCK_111 -->

<!-- BLOCK_112 | doxcn6xmrryBvPUEc1L0ULQIbvg -->
### Demo 3：Supervisor Multi-Agent（主管多 Agent 协作）<!-- 标题序号: 10.3 --><!-- END_BLOCK_112 -->

<!-- BLOCK_113 | doxcnVAgdXniy3d5Rg2LD3d9Ksg -->
**场景**：一个 Supervisor Agent 统筹调度多个专业 Agent（研究员、程序员、评审员），通过 StateGraph 编排协作流程。这是最典型的多 Agent 模式。**src/main.rs**：
<!-- END_BLOCK_113 -->

<!-- BLOCK_114 | doxcnrPaD15TzHSD2w4uDYkeBah -->
```rust
use anyhow::Result;
use nuro::prelude::*;
use nuro::graph::*;
use std::collections::HashMap;

// ─── 定义图状态 ───
#[derive(Debug, Clone, GraphState)]
struct TeamState {
    /// 对话历史 — 所有 Agent 共享
    #[reducer(append)]
    messages: Vec<Message>,
    
    /// 下一个要执行的 Agent
    #[reducer(overwrite)]
    next_agent: String,
    
    /// 各 Agent 产出的工件
    #[reducer(append)]
    artifacts: Vec<Artifact>,
    
    /// 迭代次数
    #[reducer(overwrite)]
    iteration: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Artifact {
    agent: String,
    content_type: String, // "code" | "document" | "review"
    content: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::init();

    // ─── 1. 创建各个专业 Agent ───
    
    let researcher = AgentLoop::builder()
        .llm(AnthropicProvider::new("claude-sonnet-4-20250514"))
        .system_prompt("\
            You are a senior research specialist. Your job is to:
            1. Search for relevant information on the given topic
            2. Synthesize findings into clear, structured notes
            3. Cite all sources
            Output your findings as structured markdown.")
        .tool(WebSearchTool::new_from_env()?)
        .tool(WikipediaTool::new())
        .max_iterations(10)
        .build()?;

    let coder = AgentLoop::builder()
        .llm(AnthropicProvider::new("claude-sonnet-4-20250514"))
        .system_prompt("\
            You are an expert Rust programmer. Your job is to:
            1. Read the research findings and requirements
            2. Write clean, idiomatic, well-documented Rust code
            3. Include unit tests
            4. Save code files to ./workspace/src/")
        .tool(FileOpsTool::new("./workspace"))
        .tool(BashTool::new()
            .allowed_commands(vec!["cargo", "rustfmt", "clippy"]))
        .max_iterations(15)
        .build()?;

    let reviewer = AgentLoop::builder()
        .llm(OpenAiProvider::new("gpt-4o"))
        .system_prompt("\
            You are a code review expert. Your job is to:
            1. Review the code for correctness, performance, and style
            2. Check that tests are comprehensive
            3. Provide specific, actionable feedback
            4. Rate the code: APPROVE or REQUEST_CHANGES
            Always end with a clear verdict line: VERDICT: APPROVE or VERDICT: REQUEST_CHANGES")
        .tool(FileOpsTool::new("./workspace").read_only())
        .max_iterations(5)
        .build()?;

    // ─── 2. 定义 Supervisor 节点 ───
    
    let supervisor_llm = AnthropicProvider::new("claude-sonnet-4-20250514");
    
    let supervisor_node = FnNode::new(
        |state: &TeamState, ctx: &mut NodeContext| async move {
            let llm = ctx.get::<Arc<dyn LlmProvider>>("supervisor_llm")?;
            
            let prompt = format!("\
                You are a project supervisor coordinating a team of:
                - researcher: searches and synthesizes information
                - coder: writes Rust code based on research
                - reviewer: reviews code quality
                
                Current iteration: {}
                Artifacts produced so far: {}
                
                Based on the conversation history, decide which team 
                member should work next. If the project is complete 
                (code has been APPROVED by reviewer), respond with FINISH.
                
                Respond with EXACTLY one of: researcher, coder, reviewer, FINISH",
                state.iteration,
                state.artifacts.len(),
            );
            
            let response = llm.generate(LlmRequest {
                messages: vec![
                    Message::system(&prompt),
                    Message::user(&format!(
                        "Recent messages:\n{}",
                        state.messages.iter()
                            .rev().take(5).rev()
                            .map(|m| format!("{:?}: {}", m.role, m.text()))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )),
                ],
                tools: None,
                config: LlmConfig::default(),
            }).await?;
            
            let decision = response.message.text()
                .trim().to_lowercase();
            
            Ok(TeamState::Update {
                next_agent: Some(decision),
                iteration: Some(state.iteration + 1),
                ..Default::default()
            })
        }
    );

    // ─── 3. 构建 StateGraph ───
    
    let graph = StateGraph::<TeamState>::new()
        // 节点
        .add_node("supervisor", supervisor_node)
        .add_node("researcher", AgentNode::new(researcher))
        .add_node("coder", AgentNode::new(coder))
        .add_node("reviewer", AgentNode::new(reviewer))
        // 入口
        .set_entry_point("supervisor")
        // Supervisor 的条件路由
        .add_conditional_edge(
            "supervisor",
            |state: &TeamState| state.next_agent.clone(),
            HashMap::from([
                ("researcher".into(), "researcher".into()),
                ("coder".into(),      "coder".into()),
                ("reviewer".into(),   "reviewer".into()),
                ("finish".into(),     END.into()),
            ]),
        )
        // 所有 Worker Agent 执行完后回到 Supervisor
        .add_edge("researcher", "supervisor")
        .add_edge("coder", "supervisor")
        .add_edge("reviewer", "supervisor")
        // 编译 + 添加状态持久化
        .compile()?
        .with_checkpointer(
            SqliteStateStore::new("./team_state.db").await?
        );

    // ─── 4. 执行 ───
    
    let initial_state = TeamState {
        messages: vec![Message::user("\
            Build a Rust CLI tool that monitors CPU and memory usage 
            in real-time, with a TUI interface using the ratatui crate. 
            The tool should update every second and show a sparkline chart.")],
        next_agent: String::new(),
        artifacts: vec![],
        iteration: 0,
    };

    println!("Starting multi-agent project...\n");
    
    // 流式执行，实时追踪每个 Agent 的行为
    let mut stream = graph.stream(initial_state);
    
    while let Some(event) = stream.next().await {
        match event? {
            Event { agent_id, kind: EventKind::AgentStart { .. }, .. } => {
                println!("\n>>> Agent [{}] started working", agent_id);
            }
            Event { agent_id, kind: EventKind::AgentEnd { output }, .. } => {
                println!("<<< Agent [{}] finished", agent_id);
                println!("    Output preview: {}...", 
                    &output.to_string()[..100.min(output.to_string().len())]);
            }
            Event { kind: EventKind::StateUpdate { key, .. }, .. } => {
                if key == "next_agent" {
                    println!("--- Supervisor routed to next agent");
                }
            }
            _ => {}
        }
    }

    println!("\n=== Project Complete ===");
    Ok(())
}
```
<!-- END_BLOCK_114 -->

<!-- BLOCK_115 | doxcnlTfEvNSy8r99uTXUL3qOVe -->
---
<!-- END_BLOCK_115 -->

<!-- BLOCK_116 | doxcn4TVlt0gqemEYwFzDsKClAc -->
### Demo 4：Human-in-the-Loop Approval（人工审批流程）<!-- 标题序号: 10.4 --><!-- END_BLOCK_116 -->

<!-- BLOCK_117 | doxcncwUcfnXFUal2G2EXdN29Ac -->
**场景**：Agent 在执行关键操作前需要人工确认，利用 Graph 的 Checkpoint + Interrupt 机制暂停并恢复。
<!-- END_BLOCK_117 -->

<!-- BLOCK_118 | doxcnioDa2l9yp60lUoLvBBWkdd -->
```rust
use anyhow::Result;
use nuro::prelude::*;
use nuro::graph::*;

#[derive(Debug, Clone, GraphState)]
struct ApprovalState {
    #[reducer(append)]
    messages: Vec<Message>,
    #[reducer(overwrite)]
    pending_action: Option<PendingAction>,
    #[reducer(overwrite)]
    approved: Option<bool>,
    #[reducer(append)]
    audit_log: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PendingAction {
    description: String,
    tool_name: String,
    tool_input: Value,
    risk_level: String, // "low" | "medium" | "high"
}

#[tokio::main]
async fn main() -> Result<()> {
    let agent = AgentLoop::builder()
        .llm(AnthropicProvider::new("claude-sonnet-4-20250514"))
        .system_prompt("You are a DevOps automation agent...")
        .tool(KubernetesTool::new())
        .tool(DatabaseTool::new())
        .build()?;

    // 构建带人工审批的图
    let graph = StateGraph::<ApprovalState>::new()
        .add_node("agent", AgentNode::new(agent))
        .add_node("risk_evaluator", FnNode::new(
            |state: &ApprovalState, _ctx| async move {
                // 评估操作风险
                let action = state.pending_action.as_ref().unwrap();
                let risk = match action.tool_name.as_str() {
                    "kubectl_delete" | "db_drop" => "high",
                    "kubectl_scale" | "db_migrate" => "medium",
                    _ => "low",
                };
                Ok(ApprovalState::Update {
                    pending_action: Some(PendingAction {
                        risk_level: risk.to_string(),
                        ..action.clone()
                    }),
                    ..Default::default()
                })
            }
        ))
        // 人工审批节点 — 触发 interrupt
        .add_node("human_approval", HumanInputNode::new(
            |state: &ApprovalState| {
                let action = state.pending_action.as_ref().unwrap();
                format!(
                    "[APPROVAL REQUIRED]\n\
                     Action: {}\n\
                     Risk Level: {}\n\
                     Details: {}\n\n\
                     Type 'approve' or 'reject':",
                    action.tool_name,
                    action.risk_level,
                    action.description,
                )
            }
        ))
        .add_node("executor", FnNode::new(
            |state: &ApprovalState, _ctx| async move {
                if state.approved == Some(true) {
                    // 执行已批准的操作
                    let action = state.pending_action.as_ref().unwrap();
                    println!("Executing: {}", action.description);
                    Ok(ApprovalState::Update {
                        audit_log: vec![format!(
                            "[EXECUTED] {} at {}", 
                            action.description,
                            chrono::Utc::now()
                        )],
                        ..Default::default()
                    })
                } else {
                    Ok(ApprovalState::Update {
                        audit_log: vec!["[REJECTED] Action was rejected"
                            .to_string()],
                        ..Default::default()
                    })
                }
            }
        ))
        .set_entry_point("agent")
        .add_edge("agent", "risk_evaluator")
        .add_conditional_edge(
            "risk_evaluator",
            |state: &ApprovalState| {
                let risk = &state.pending_action
                    .as_ref().unwrap().risk_level;
                match risk.as_str() {
                    "high" | "medium" => "need_approval".into(),
                    _ => "auto_approve".into(),
                }
            },
            HashMap::from([
                ("need_approval".into(), "human_approval".into()),
                ("auto_approve".into(), "executor".into()),
            ]),
        )
        .add_edge("human_approval", "executor")
        .set_finish_point("executor")
        .compile()?
        .with_checkpointer(SqliteStateStore::new("approvals.db").await?);

    // 执行 — 高风险操作会暂停等待人工输入
    let result = graph.invoke(ApprovalState {
        messages: vec![Message::user(
            "Scale down the production database replicas to 1"
        )],
        ..Default::default()
    }).await?;

    println!("Audit log: {:?}", result.audit_log);
    Ok(())
}
```
<!-- END_BLOCK_118 -->

<!-- BLOCK_119 | doxcnv5sOfLOuWHZOCy2xGKK7Bg -->
---
<!-- END_BLOCK_119 -->

<!-- BLOCK_120 | doxcnsBXsTR4lFpy2VrwO1K3ZUh -->
### Demo 5：MCP Tool Server + Client 全链路<!-- 标题序号: 10.5 --><!-- END_BLOCK_120 -->

<!-- BLOCK_121 | doxcna6XWdHH1H2KY47gCaDMYBg -->
**场景**：构建一个 MCP Tool Server 暴露数据库查询和文件搜索能力，然后在 Agent 中通过 MCP Client 消费这些工具。支持 stdio 和 SSE 两种传输模式。
<!-- END_BLOCK_121 -->

<!-- BLOCK_122 | doxcn4DorHYpX1iKKmpSMt4rb2c -->
#### 5a. MCP Server 端<!-- END_BLOCK_122 -->

<!-- BLOCK_123 | doxcnkHkZKvW4W0EPPXNdrsOgPg -->
```rust
// examples/mcp_server/src/main.rs
use anyhow::Result;
use nuro::mcp::*;
use nuro::prelude::*;

/// 数据库查询工具
#[derive(Tool)]
#[tool(
    name = "query_database",
    description = "Execute a read-only SQL query against the database"
)]
struct DatabaseQueryTool {
    pool: sqlx::PgPool,
}

#[derive(ToolArgs, Serialize, Deserialize, JsonSchema)]
struct QueryArgs {
    /// SQL query (SELECT only)
    #[schemars(description = "Read-only SQL query to execute")]
    sql: String,
    /// Maximum rows to return
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_limit() -> usize { 100 }

#[async_trait]
impl ToolExecute for DatabaseQueryTool {
    type Args = QueryArgs;

    async fn execute(
        &self, args: Self::Args, _ctx: &ToolContext
    ) -> Result<ToolOutput> {
        // 安全检查：只允许 SELECT
        if !args.sql.trim().to_uppercase().starts_with("SELECT") {
            return Err(anyhow!("Only SELECT queries are allowed"));
        }
        let sql = format!("{} LIMIT {}", args.sql, args.limit);
        let rows: Vec<serde_json::Value> = sqlx::query(&sql)
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(|row| row_to_json(row))
            .collect();
        Ok(ToolOutput::json(serde_json::json!({
            "rows": rows,
            "count": rows.len()
        })))
    }
}

/// 全文搜索工具
#[derive(Tool)]
#[tool(
    name = "search_codebase",
    description = "Search the codebase using ripgrep patterns"
)]
struct CodeSearchTool {
    root_dir: String,
}

#[derive(ToolArgs, Serialize, Deserialize, JsonSchema)]
struct SearchArgs {
    /// Search pattern (regex supported)
    pattern: String,
    /// File type filter (e.g., "rs", "py", "ts")
    file_type: Option<String>,
    /// Maximum results
    #[serde(default = "default_max_results")]
    max_results: usize,
}

fn default_max_results() -> usize { 20 }

#[async_trait]
impl ToolExecute for CodeSearchTool {
    type Args = SearchArgs;

    async fn execute(
        &self, args: Self::Args, _ctx: &ToolContext
    ) -> Result<ToolOutput> {
        let mut cmd = tokio::process::Command::new("rg");
        cmd.arg("--json")
           .arg("--max-count").arg(args.max_results.to_string())
           .arg(&args.pattern)
           .arg(&self.root_dir);
        if let Some(ft) = &args.file_type {
            cmd.arg("--type").arg(ft);
        }
        let output = cmd.output().await?;
        Ok(ToolOutput::text(String::from_utf8_lossy(&output.stdout)))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = sqlx::PgPool::connect(
        &std::env::var("DATABASE_URL")?
    ).await?;

    let server = McpServer::builder("dev-tools", "1.0.0")
        .description("Development tools: database queries and code search")
        // 注册工具
        .tool(DatabaseQueryTool { pool: pool.clone() })
        .tool(CodeSearchTool { root_dir: "./src".into() })
        // 注册资源（MCP Resources）
        .resource(
            McpFileResource::new("config://app", "./config/app.toml")
                .description("Application configuration")
                .mime_type("application/toml")
        )
        .resource(
            McpFileResource::new("schema://database", "./schema.sql")
                .description("Database schema definition")
        )
        // 注册提示词模板（MCP Prompts）
        .prompt(
            McpPrompt::new("analyze-query")
                .description("Analyze a slow SQL query")
                .argument("sql", "The SQL query to analyze", true)
                .template("Analyze this SQL query for performance: {sql}")
        )
        .build();

    // 根据命令行参数选择传输方式
    match std::env::args().nth(1).as_deref() {
        Some("--sse") => {
            let addr = "0.0.0.0:8080".parse()?;
            println!("MCP Server starting on http://{} (SSE)", addr);
            server.serve_sse(addr).await?
        }
        Some("--http") => {
            let addr = "0.0.0.0:8080".parse()?;
            println!("MCP Server starting on http://{} (Streamable HTTP)", addr);
            server.serve_http(addr).await?
        }
        _ => {
            // 默认 stdio 模式（用于 Claude Code 集成）
            server.serve_stdio().await?
        }
    }

    Ok(())
}
```
<!-- END_BLOCK_123 -->

<!-- BLOCK_124 | doxcnzW5cW0K5lZeVNHa0QLetEg -->
#### 5b. MCP Client 端 — Agent 消费 MCP 工具<!-- END_BLOCK_124 -->

<!-- BLOCK_125 | doxcnQYDomvDNqWPNGDO60Vonde -->
```rust
// examples/mcp_client/src/main.rs
use anyhow::Result;
use nuro::prelude::*;
use nuro::mcp::McpClient;

#[tokio::main]
async fn main() -> Result<()> {
    // 方式1：通过 stdio 连接 MCP Server
    let mcp_stdio = McpClient::connect_stdio(
        "cargo", &["run", "--bin", "mcp-server"]
    ).await?;

    // 方式2：通过 SSE 连接远程 MCP Server
    let mcp_remote = McpClient::connect_sse(
        "https://tools.example.com/mcp/sse"
    ).await?;

    // 构建 ToolBox — 从多个 MCP Server 加载工具
    let toolbox = ToolBox::new()
        .add_mcp_client(mcp_stdio).await?    // 本地工具
        .add_mcp_client(mcp_remote).await?   // 远程工具
        .add(CalculatorTool::new());          // 本地原生工具

    println!("Available tools:");
    for schema in toolbox.get_schemas() {
        println!("  - {} : {}", schema.name, schema.description);
    }

    // 构建 Agent
    let agent = AgentLoop::builder()
        .llm(AnthropicProvider::new("claude-sonnet-4-20250514"))
        .system_prompt("\
            You are a senior developer assistant with access to:
            - Database query tool (read-only SQL)
            - Codebase search tool (ripgrep)
            - Calculator
            Help the user analyze code and data.")
        .toolbox(toolbox)
        .max_iterations(15)
        .build()?;

    let mut ctx = AgentContext::new();
    let output = agent.invoke(AgentInput::text(
        "Find all functions in the codebase that query the 'users' table, \
         then check how many users we have in the database."
    ), &mut ctx).await?;

    println!("{}", output.text());
    Ok(())
}
```
<!-- END_BLOCK_125 -->

<!-- BLOCK_126 | doxcnyxBC8owSo5E5sv9QY51IOh -->
---
<!-- END_BLOCK_126 -->

<!-- BLOCK_127 | doxcnbODURW77YZaSiEJLmbZ85e -->
### Demo 6：A2A 跨 Agent 协作（Agent 联邦）<!-- 标题序号: 10.6 --><!-- END_BLOCK_127 -->

<!-- BLOCK_128 | doxcnZhzdmacpwuKzkLe6aRLSmd -->
**场景**：多个独立部署的 Agent 通过 A2A 协议发现彼此能力并协作。一个翻译 Agent、一个摘要 Agent、一个主 Agent 协调它们。
<!-- END_BLOCK_128 -->

<!-- BLOCK_129 | doxcnADN6PUj8994ldTfeEezsnd -->
#### 6a. 翻译 Agent — A2A Server<!-- END_BLOCK_129 -->

<!-- BLOCK_130 | doxcnBgRjNnJmaQKNeECgnrG4vh -->
```rust
// services/translator/src/main.rs
use anyhow::Result;
use nuro::prelude::*;
use nuro::a2a::*;

#[tokio::main]
async fn main() -> Result<()> {
    let translator = AgentLoop::builder()
        .llm(OpenAiProvider::new("gpt-4o"))
        .system_prompt("\
            You are a professional translator. Translate the given 
            text to the target language. Preserve formatting, tone, 
            and technical terms. Always output ONLY the translation.")
        .build()?;

    // 定义 Agent Card（A2A 服务发现）
    let server = A2aServer::builder()
        .agent(translator)
        .name("Translation Agent")
        .description("Professional multi-language translation service")
        .version("1.0.0")
        .skill(AgentSkill {
            id: "translate".into(),
            name: "Text Translation".into(),
            description: "Translate text between languages".into(),
            tags: vec!["translation", "i18n", "localization"]
                .into_iter().map(String::from).collect(),
            examples: vec![
                "Translate to Japanese: Hello World".into(),
                "Translate to Spanish: The quick brown fox".into(),
            ],
        })
        .input_modes(vec!["text/plain".into()])
        .output_modes(vec!["text/plain".into()])
        .auth(AuthenticationInfo::bearer("TOKEN_ENV_VAR"))
        .build();

    let addr = "0.0.0.0:8081".parse()?;
    println!("Translation A2A Agent running on http://{}", addr);
    println!("Agent Card: http://{}/.well-known/agent.json", addr);
    server.serve(addr).await?;
    Ok(())
}
```
<!-- END_BLOCK_130 -->

<!-- BLOCK_131 | doxcnKryQrnrDxqe0ih3KriPYne -->
#### 6b. 主 Agent — 消费 A2A Agent<!-- END_BLOCK_131 -->

<!-- BLOCK_132 | doxcntBHP3v5UkWWBlSz2Amw42c -->
```rust
// services/orchestrator/src/main.rs
use anyhow::Result;
use nuro::prelude::*;
use nuro::a2a::*;

#[tokio::main]
async fn main() -> Result<()> {
    // 发现远程 A2A Agent
    let translator_card = A2aClient::discover(
        "https://translator.internal:8081"
    ).await?;
    
    let summarizer_card = A2aClient::discover(
        "https://summarizer.internal:8082"
    ).await?;

    println!("Discovered agents:");
    println!("  - {} : {}", translator_card.name, 
        translator_card.description);
    println!("  - {} : {}", summarizer_card.name, 
        summarizer_card.description);

    // 将远程 Agent 包装为本地 Tool
    let toolbox = ToolBox::new()
        .add(A2aAgentTool::from_card(translator_card.clone()))
        .add(A2aAgentTool::from_card(summarizer_card.clone()))
        .add(WebSearchTool::new_from_env()?);

    // 主 Agent — 可以调度远程 Agent
    let orchestrator = AgentLoop::builder()
        .llm(AnthropicProvider::new("claude-sonnet-4-20250514"))
        .system_prompt("\
            You are a global content coordinator. You have access to:
            - Translation Agent: translates text to any language
            - Summarizer Agent: creates concise summaries
            - Web Search: finds information online
            
            Use these capabilities to help users with international 
            content tasks.")
        .toolbox(toolbox)
        .max_iterations(10)
        .build()?;

    let mut ctx = AgentContext::new();

    // 复杂任务：搜索 → 摘要 → 翻译
    let output = orchestrator.invoke(AgentInput::text("\
        Find the latest Rust 2026 roadmap announcement. 
        Summarize the key points in 3 bullet points. 
        Then translate the summary to Japanese and Chinese."
    ), &mut ctx).await?;

    println!("{}", output.text());
    Ok(())
}
```
<!-- END_BLOCK_132 -->

<!-- BLOCK_133 | doxcnSJecUSuAc8ZPccJrP0a0hd -->
---
<!-- END_BLOCK_133 -->

<!-- BLOCK_134 | doxcn2RK7ZRTxBABza751qch8ic -->
### Demo 7：RAG Agent（知识库问答）<!-- 标题序号: 10.7 --><!-- END_BLOCK_134 -->

<!-- BLOCK_135 | doxcnIVcOD1Mxh69BQMWakZ1TQb -->
**场景**：基于本地文档库构建 RAG Agent，支持文档索引、向量检索、带引用的问答。
<!-- END_BLOCK_135 -->

<!-- BLOCK_136 | doxcnjRQJodFDOYgysbMxvYTTDc -->
```rust
use anyhow::Result;
use nuro::prelude::*;
use nuro::rag::*;
use nuro::memory::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // ─── 1. 构建向量存储 ───
    let vector_store = SqliteVectorStore::open("./knowledge.db").await?;
    let embedder = OpenAiEmbedder::new("text-embedding-3-small");

    // ─── 2. 索引文档（首次运行时执行）───
    let indexer = DocumentIndexer::new()
        .embedder(embedder.clone())
        .vector_store(vector_store.clone())
        .chunker(RecursiveChunker::new()
            .chunk_size(512)
            .chunk_overlap(64)
            .separators(vec!["\n\n", "\n", ". ", " "])
        )
        .metadata_extractor(FileMetadataExtractor::new());

    // 索引 docs/ 目录下所有 markdown 和 PDF
    let doc_dir = PathBuf::from("./docs");
    if !vector_store.has_index("docs-v1").await? {
        println!("Indexing documents...");
        let stats = indexer.index_directory(
            &doc_dir,
            &["md", "pdf", "txt"],
            "docs-v1",
        ).await?;
        println!("Indexed {} documents, {} chunks", 
            stats.documents, stats.chunks);
    }

    // ─── 3. 构建 RAG Retriever Tool ───
    let retriever = RetrieverTool::new()
        .vector_store(vector_store)
        .embedder(embedder)
        .top_k(5)
        .similarity_threshold(0.65)
        .reranker(CohereReranker::new()?) // 可选：重排序
        .build();

    // ─── 4. 构建 RAG Agent ───
    let agent = AgentLoop::builder()
        .llm(AnthropicProvider::new("claude-sonnet-4-20250514")
            .max_tokens(4096)
        )
        .system_prompt("\
            You are a knowledge assistant for our internal documentation.
            
            IMPORTANT RULES:
            1. ALWAYS use the retriever tool to search docs before answering
            2. Base your answers ONLY on retrieved documents
            3. Include [Source: filename] citations for every claim
            4. If no relevant docs found, say 'I could not find information 
               about this in the documentation'
            5. Never make up information not in the documents")
        .tool(retriever)
        .max_iterations(5)
        .build()?;

    // ─── 5. 交互式问答 ───
    let mut ctx = AgentContext::new();
    loop {
        print!("Question: ");
        std::io::Write::flush(&mut std::io::stdout())?;
        let mut q = String::new();
        std::io::stdin().read_line(&mut q)?;
        let q = q.trim();
        if q.is_empty() || q == "quit" { break; }

        let output = agent.invoke(
            AgentInput::text(q), &mut ctx
        ).await?;
        println!("\nAnswer: {}\n", output.text());
    }

    Ok(())
}
```
<!-- END_BLOCK_136 -->

<!-- BLOCK_137 | doxcnJ0RzAFMY8qeWFckMUiEPXc -->
---
<!-- END_BLOCK_137 -->

<!-- BLOCK_138 | doxcn3I4T1jDac0bSbKhoqSwQAh -->
### Demo 8：Production HTTP Server（生产级 Agent 服务）<!-- 标题序号: 10.8 --><!-- END_BLOCK_138 -->

<!-- BLOCK_139 | doxcnENTRtaOFfE2zfZHZzYOyYc -->
**场景**：将 Agent 包装为生产级 HTTP 服务，支持 SSE 流式输出、OpenTelemetry 追踪、健康检查、速率限制。
<!-- END_BLOCK_139 -->

<!-- BLOCK_140 | doxcnWHrf13h1KCZO875Z65gbTf -->
```rust
use anyhow::Result;
use nuro::prelude::*;
use nuro::server::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    // ─── 1. 初始化 OpenTelemetry ───
    nuro::tracing::init_otel(
        "nuro-service",
        &std::env::var("OTEL_ENDPOINT")
            .unwrap_or("http://localhost:4317".into()),
    )?;

    // ─── 2. 构建 Agent ───
    let agent = AgentLoop::builder()
        .llm(AnthropicProvider::new("claude-sonnet-4-20250514"))
        .system_prompt("You are a customer support assistant...")
        .toolbox(
            ToolBox::new()
                .add(DatabaseQueryTool::new_from_env().await?)
                .add(TicketSystemTool::new_from_env()?)
                .add_mcp_client(
                    McpClient::connect_sse("http://tools:8080/sse").await?
                ).await?
        )
        .memory(CompositeMemory::new()
            .conversation(ConversationMemory::new().max_tokens(16000))
            .long_term(VectorMemory::new(
                QdrantVectorStore::new("http://qdrant:6334", "support").await?,
                OpenAiEmbedder::new("text-embedding-3-small"),
            ))
        )
        .guardrail(ContentGuardrail::new()
            .block_pii(true)
            .max_output_tokens(2000)
        )
        .max_iterations(15)
        .build()?;

    // ─── 3. 构建 HTTP Server ───
    let server = AgentServer::builder()
        .agent(agent)
        // REST API 端点
        .route("POST", "/v1/chat", ChatHandler::new())
        .route("POST", "/v1/chat/stream", StreamChatHandler::new())
        // 同时暴露为 MCP Server
        .mcp_endpoint("/mcp")
        // 同时暴露为 A2A Server
        .a2a_endpoint("/a2a")
        // 中间件
        .middleware(CorsMiddleware::permissive())
        .middleware(RateLimitMiddleware::new()
            .requests_per_second(10)
            .burst_size(20)
        )
        .middleware(AuthMiddleware::bearer("API_KEY_ENV"))
        .middleware(TracingMiddleware::new()) // OpenTelemetry
        .middleware(MetricsMiddleware::new()) // Prometheus metrics
        // 健康检查
        .health_check("/health")
        .ready_check("/ready")
        // Prometheus metrics
        .metrics_endpoint("/metrics")
        .build();

    // ─── 4. 启动服务 ───
    let addr: SocketAddr = "0.0.0.0:3000".parse()?;
    tracing::info!("Agent server starting on http://{}", addr);
    tracing::info!("  Chat API:  POST /v1/chat");
    tracing::info!("  Stream:    POST /v1/chat/stream (SSE)");
    tracing::info!("  MCP:       /mcp (stdio/sse/http)");
    tracing::info!("  A2A:       /a2a/.well-known/agent.json");
    tracing::info!("  Health:    GET /health");
    tracing::info!("  Metrics:   GET /metrics");

    server.serve(addr).await?;
    Ok(())
}
```
<!-- END_BLOCK_140 -->

<!-- BLOCK_141 | doxcnexQXdEoy7z8BOBRg60zWtf -->
**对应的 Dockerfile**：
<!-- END_BLOCK_141 -->

<!-- BLOCK_142 | doxcnB1J5j9PGGY6PJSf6zORlbd -->
```dockerfile
FROM rust:1.82-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --features full

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/agent-server /usr/local/bin/

ENV RUST_LOG=info
EXPOSE 3000
CMD ["agent-server"]
```
<!-- END_BLOCK_142 -->

<!-- BLOCK_143 | doxcnUPydACfqkGhyPttuy2brwg -->
**对应的 docker-compose.yml**：
<!-- END_BLOCK_143 -->

<!-- BLOCK_144 | doxcnLBqyyNPZ0Ik5713Y61ZHHh -->
```yaml
version: '3.8'
services:
  agent-server:
    build: .
    ports:
      - "3000:3000"
    environment:
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
      - DATABASE_URL=postgres://user:pass@postgres:5432/app
      - OTEL_ENDPOINT=http://jaeger:4317
    depends_on:
      - postgres
      - qdrant
      - jaeger

  postgres:
    image: postgres:16
    environment:
      POSTGRES_PASSWORD: pass
      POSTGRES_USER: user
      POSTGRES_DB: app
    volumes:
      - pgdata:/var/lib/postgresql/data

  qdrant:
    image: qdrant/qdrant:v1.12.0
    ports:
      - "6333:6333"
    volumes:
      - qdrant_data:/qdrant/storage

  jaeger:
    image: jaegertracing/all-in-one:1.60
    ports:
      - "16686:16686"  # Jaeger UI
      - "4317:4317"    # OTLP gRPC

volumes:
  pgdata:
  qdrant_data:
```
<!-- END_BLOCK_144 -->

<!-- BLOCK_145 | doxcnlapv3oI5RSex1Yw2sG7KMb -->
---
<!-- END_BLOCK_145 -->

<!-- BLOCK_146 | doxcn9sdkSaXVCA8pXFfAijnvKb -->
### Demo 场景总览<!-- 标题序号: 10.9 --><!-- END_BLOCK_146 -->

<!-- BLOCK_147 | doxcnuXCpXIH9BrqfALnbl6nrGd -->
<table header-row="true" col-widths="183,183,183,183">
    <tr>
        <td>Demo</td>
        <td>场景</td>
        <td>核心能力展示</td>
        <td>复杂度</td>
    </tr>
    <tr>
        <td>**Demo 1**</td>
        <td>CLI 聊天机器人</td>
        <td>AgentLoop + 流式输出 + 对话记忆</td>
        <td>入门</td>
    </tr>
    <tr>
        <td>**Demo 2**</td>
        <td>ReAct 研究助手</td>
        <td>Tool 系统 + Hook + Guardrail + 向量记忆</td>
        <td>中级</td>
    </tr>
    <tr>
        <td>**Demo 3**</td>
        <td>Supervisor 多 Agent</td>
        <td>StateGraph + 条件路由 + Checkpoint</td>
        <td>高级</td>
    </tr>
    <tr>
        <td>**Demo 4**</td>
        <td>人工审批流程</td>
        <td>Human-in-the-Loop + Graph Interrupt + 恢复</td>
        <td>高级</td>
    </tr>
    <tr>
        <td>**Demo 5**</td>
        <td>MCP 全链路</td>
        <td>MCP Server/Client + Resource + Prompt</td>
        <td>中级</td>
    </tr>
    <tr>
        <td>**Demo 6**</td>
        <td>A2A 跨 Agent 协作</td>
        <td>A2A 协议 + Agent Card + 服务发现</td>
        <td>高级</td>
    </tr>
    <tr>
        <td>**Demo 7**</td>
        <td>RAG 知识库问答</td>
        <td>文档索引 + 向量检索 + 引用生成</td>
        <td>中级</td>
    </tr>
    <tr>
        <td>**Demo 8**</td>
        <td>生产级 HTTP 服务</td>
        <td>Server + OTel + 限流 + Docker 部署</td>
        <td>生产级</td>
    </tr>
</table>
<!-- END_BLOCK_147 -->

<!-- BLOCK_148 | doxcnSkXMbWi5vd6WmUUODa000g -->
---
<!-- END_BLOCK_148 -->

<!-- BLOCK_149 | doxcnFPnW5Wtn1cwDQyQspjSfse -->
## 九、项目 Roadmap<!-- 标题序号: 11 --><!-- END_BLOCK_149 -->

<!-- BLOCK_150 | doxcn3lnLhowRbcuYUvE6z5eGzd -->
<table col-widths="244,244,244">
    <tr>
        <td>阶段</td>
        <td>里程碑</td>
        <td>核心交付</td>
    </tr>
    <tr>
        <td>**Phase 1: Foundation** (M1-M2)</td>
        <td>v0.1.0</td>
        <td>`core` trait、`llm` Provider、基础 `AgentLoop`、`tools` 系统</td>
    </tr>
    <tr>
        <td>**Phase 2: Graph Engine** (M3-M4)</td>
        <td>v0.2.0</td>
        <td>`graph` StateGraph、条件边/循环、Checkpoint 持久化</td>
    </tr>
    <tr>
        <td>**Phase 3: Protocols** (M5-M6)</td>
        <td>v0.3.0</td>
        <td>`mcp` + `a2a` 完整实现、多传输层支持</td>
    </tr>
    <tr>
        <td>**Phase 4: Memory & RAG** (M7-M8)</td>
        <td>v0.4.0</td>
        <td>分层记忆系统、向量存储、RAG Pipeline</td>
    </tr>
    <tr>
        <td>**Phase 5: Production** (M9-M10)</td>
        <td>v0.5.0</td>
        <td>Guardrail、OpenTelemetry、性能优化</td>
    </tr>
    <tr>
        <td>**Phase 6: Ecosystem** (M11-M12)</td>
        <td>v1.0.0</td>
        <td>文档/examples、WASM、Python binding（PyO3）</td>
    </tr>
</table>
<!-- END_BLOCK_150 -->

<!-- BLOCK_151 | doxcnjbUA6q2sjEa8hp1jfAsDNb -->
---
<!-- END_BLOCK_151 -->

<!-- BLOCK_152 | doxcnuKDVSxt0uNYycMMFxPPibg -->
## 十、关键设计决策与权衡<!-- 标题序号: 12 --><!-- END_BLOCK_152 -->

<!-- BLOCK_153 | doxcnRwvcZcMD1v7fWdscrrO7jh -->
### 10.1 为什么选择 Graph 而非纯 Agent Loop？<!-- END_BLOCK_153 -->

<!-- BLOCK_154 | doxcndYvJiV2yBrqH0wK4znewsd -->
Graph 模型是 Agent Loop 的超集。单个 Agent Loop 本身可以表达为图中的单个节点。但图结构额外提供了：
<!-- END_BLOCK_154 -->

<!-- BLOCK_155 | doxcnAQka9Ynv3jEwnJgSOFFbsf -->
- **可视化**：图天然可以可视化，便于 Debug
<!-- END_BLOCK_155 -->

<!-- BLOCK_156 | doxcnn1BJTWZ6fzAtiPMIj2zR8e -->
- **可组合**：多个 Agent 的协作关系用图表达最自然
<!-- END_BLOCK_156 -->

<!-- BLOCK_157 | doxcnfvBbFP5doW1GiSEOd0cPVg -->
- **Checkpoint**：图的每个节点执行后可以存档状态，支持断点恢复
<!-- END_BLOCK_157 -->

<!-- BLOCK_158 | doxcnLbD6SH4I38i1NV8f8GwBOd -->
- **Human-in-the-loop**：在图的任意边上插入人工审核节点
<!-- END_BLOCK_158 -->

<!-- BLOCK_159 | doxcnyhmvP6rOaM5t3uHl4PndYb -->
### 10.2 Trait vs Enum — Agent 多态性<!-- END_BLOCK_159 -->

<!-- BLOCK_160 | doxcnWzEVf1R3HqPPYdRtp09vEf -->
选择 Trait Object (`dyn Agent`) 而非 Enum 的理由：
<!-- END_BLOCK_160 -->

<!-- BLOCK_161 | doxcn52AnlTrdFh8aRmb8eblLIh -->
- **开放扩展**：用户可以自定义 Agent 类型而无需修改 SDK
<!-- END_BLOCK_161 -->

<!-- BLOCK_162 | doxcnDGxEyBJ2Dt3rAneEf1Xghh -->
- **跨 crate 兼容**：第三方可以发布自己的 Agent 实现
<!-- END_BLOCK_162 -->

<!-- BLOCK_163 | doxcniAmqHpgmBZYaztdjX5aEuf -->
- **权衡**：动态分发有轻微性能开销，但 Agent 执行瓶颈在 LLM 网络调用，开销可忽略
<!-- END_BLOCK_163 -->

<!-- BLOCK_164 | doxcn3igdQX6lDXXHyORmtgHqUd -->
### 10.3 异步优先设计<!-- END_BLOCK_164 -->

<!-- BLOCK_165 | doxcn7iLqA7jTL2afVRjYNoy6Hd -->
所有 IO 操作默认异步。Rust 的 async/await 加 tokio 提供了：
<!-- END_BLOCK_165 -->

<!-- BLOCK_166 | doxcnnGUqRFnN0S6XXqMhIPwNVd -->
- **并行 Tool 执行**：多个 tool call 可以 `join_all` 并行
<!-- END_BLOCK_166 -->

<!-- BLOCK_167 | doxcn82FRrEfRwGadaadHGxaXMc -->
- **流式处理**：LLM 输出直接作为 `Stream` 传递，零缓冲
<!-- END_BLOCK_167 -->

<!-- BLOCK_168 | doxcn67HxKyanmZAy8SM0uUQ7vh -->
- **背压控制**：Rust 的 Stream 天然支持背压
<!-- END_BLOCK_168 -->

<!-- BLOCK_169 | doxcn6plaiYELQyHSKPgq37euKf -->
### 10.4 安全性<!-- END_BLOCK_169 -->

<!-- BLOCK_170 | doxcnur2bu12rvksPYkPzxqEVMf -->
- **类型安全**：Graph 的状态更新在编译期检查类型匹配
<!-- END_BLOCK_170 -->

<!-- BLOCK_171 | doxcn8LEHXe5Ih0d95XWSaa7ACb -->
- **内存安全**：Rust 所有权系统防止数据竞争
<!-- END_BLOCK_171 -->

<!-- BLOCK_172 | doxcntnfMxz36JS6jRMMKbvTPCh -->
- **运行时安全**：Guardrail 系统在 input/output 两端拦截
<!-- END_BLOCK_172 -->

<!-- BLOCK_173 | doxcnHOQxbo6oVNEAhuWZqxRLLc -->
- **Tool 安全**：Hook 系统允许在 tool call 前进行权限检查
<!-- END_BLOCK_173 -->

