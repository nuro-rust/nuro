// 顶层统一入口：re-export 各子模块的公共 API，方便使用者通过 `nuro::` 直接访问。
pub use nuro_a2a::{A2aClient, A2aServer, A2aServerBuilder, AgentCard};
pub use nuro_adk::{AdkStarterApp, AdkStarterBuilder};
pub use nuro_core::*;
pub use nuro_graph::{
    AgentNode, Checkpointer, CompiledGraph, FnNode, GraphStateTrait, GraphTool,
    InMemoryCheckpointer, NodeContext, SqliteCheckpointer, StateGraph, SubGraphNode,
};
#[cfg(feature = "openai")]
pub use nuro_llm::OpenAiLlmProvider;
pub use nuro_llm::{
    LlmProviderAdapter, MockLlmProvider, ProviderCapability, ProviderDescriptor,
    ProviderStreamEvent, StreamingMockLlmProvider,
};
pub use nuro_macros::*;
pub use nuro_mcp::{McpClient, McpServer, McpServerBuilder};
pub use nuro_memory::{ConversationMemory, InMemoryMemoryStore, MemoryStore};
pub use nuro_rag::{Embedder, NoopEmbedder, NoopVectorStore, RetrieverTool, VectorStore};
pub use nuro_runtime::{
    AgentLoop, CheckpointStore, EventStore, Guardrail, GuardrailDecision, Hook, ReplayEngine,
    ReplayMode, ReplayResult, RuntimeExecutable, RuntimeMiddleware, SqliteCheckpointStore,
    SqliteEventStore, TracingMiddleware,
};
pub use nuro_server::{ChatRequest, ChatResponse, run_server};
pub use nuro_tools::{CalculatorTool, ToolBox};

/// 统一入口的 prelude，使用者通常只需：
///
/// ```rust
/// use nuro::prelude::*;
/// ```
pub mod prelude {
    pub use nuro_a2a::{A2aClient, A2aServer, A2aServerBuilder, AgentCard};
    pub use nuro_adk::{AdkStarterApp, AdkStarterBuilder};
    pub use nuro_core::prelude::*;
    pub use nuro_graph::{
        AgentNode, Checkpointer, CompiledGraph, FnNode, GraphStateTrait, GraphTool,
        InMemoryCheckpointer, NodeContext, SqliteCheckpointer, StateGraph, SubGraphNode,
    };
    #[cfg(feature = "openai")]
    pub use nuro_llm::OpenAiLlmProvider;
    pub use nuro_llm::{
        LlmProviderAdapter, MockLlmProvider, ProviderCapability, ProviderDescriptor,
        ProviderStreamEvent, StreamingMockLlmProvider,
    };
    pub use nuro_macros::*;
    pub use nuro_mcp::{McpClient, McpServer, McpServerBuilder};
    pub use nuro_memory::{ConversationMemory, InMemoryMemoryStore, MemoryStore};
    pub use nuro_rag::{Embedder, NoopEmbedder, NoopVectorStore, RetrieverTool, VectorStore};
    pub use nuro_runtime::{
        AgentLoop, CheckpointStore, EventStore, Guardrail, GuardrailDecision, Hook, ReplayEngine,
        ReplayMode, ReplayResult, RuntimeExecutable, RuntimeMiddleware, SqliteCheckpointStore,
        SqliteEventStore, TracingMiddleware,
    };
    pub use nuro_server::{ChatRequest, ChatResponse, run_server};
    pub use nuro_tools::{CalculatorTool, ToolBox};
}
