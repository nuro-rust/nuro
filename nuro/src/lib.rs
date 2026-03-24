// 顶层统一入口：re-export 各子模块的公共 API，方便使用者通过 `nuro::` 直接访问。
pub use nuro_core::*;
pub use nuro_runtime::{AgentLoop, Guardrail, GuardrailDecision, Hook};
pub use nuro_tools::{CalculatorTool, ToolBox};
pub use nuro_graph::{
    AgentNode, Checkpointer, CompiledGraph, FnNode, GraphStateTrait, InMemoryCheckpointer,
    NodeContext, StateGraph,
};
pub use nuro_memory::{ConversationMemory, InMemoryMemoryStore, MemoryStore};
pub use nuro_rag::{Embedder, NoopEmbedder, NoopVectorStore, RetrieverTool, VectorStore};
pub use nuro_mcp::{McpClient, McpServer, McpServerBuilder};
pub use nuro_a2a::{A2aClient, A2aServer, A2aServerBuilder, AgentCard};
pub use nuro_server::{run_server, ChatRequest, ChatResponse};
pub use nuro_macros::*;
pub use nuro_llm::MockLlmProvider;
#[cfg(feature = "openai")]
pub use nuro_llm::OpenAiLlmProvider;

/// 统一入口的 prelude，使用者通常只需：
///
/// ```rust
/// use nuro::prelude::*;
/// ```
pub mod prelude {
    pub use nuro_core::prelude::*;
    pub use nuro_runtime::{AgentLoop, Guardrail, GuardrailDecision, Hook};
    pub use nuro_tools::{CalculatorTool, ToolBox};
    pub use nuro_graph::{
        AgentNode, Checkpointer, CompiledGraph, FnNode, GraphStateTrait, InMemoryCheckpointer,
        NodeContext, StateGraph,
    };
    pub use nuro_memory::{ConversationMemory, InMemoryMemoryStore, MemoryStore};
    pub use nuro_rag::{Embedder, NoopEmbedder, NoopVectorStore, RetrieverTool, VectorStore};
    pub use nuro_mcp::{McpClient, McpServer, McpServerBuilder};
    pub use nuro_a2a::{A2aClient, A2aServer, A2aServerBuilder, AgentCard};
    pub use nuro_server::{run_server, ChatRequest, ChatResponse};
    pub use nuro_macros::*;
    pub use nuro_llm::MockLlmProvider;
    #[cfg(feature = "openai")]
    pub use nuro_llm::OpenAiLlmProvider;
}
