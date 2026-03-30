# Example Layer Index

This index maps runnable examples to Nuro's four-layer model.

| Example | Command | Primary Layer | Why |
|---|---|---|---|
| `simple_chatbot` | `cargo run -p simple_chatbot` | Capability | Minimal agent loop with provider abstraction |
| `graph_demo` | `cargo run -p graph_demo` | Core | Graph orchestration and typed state flow |
| `http_server` | `cargo run -p http_server` | Runtime | Runtime execution exposed through service entrypoint |
| `adk_starter` | `cargo run -p adk_starter` | Runtime | ADK starter app scaffold with session-aware invoke |
| `mcp_demo` | `cargo run -p mcp_demo` | Platform | MCP protocol integration |
| `a2a_demo` | `cargo run -p a2a_demo` | Platform | A2A multi-agent interoperability |

Use this table with `docs/quickstart-layered.md` to choose the right starting point.

For runtime persistence and replay preparation, combine examples with:

- `docs/architecture/event-model-v1.md`
- `SqliteEventStore` / `SqliteCheckpointStore` / `TracingMiddleware`
