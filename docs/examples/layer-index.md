# Example Layer Index

This index maps runnable examples to Nuro's four-layer model.

| Example | Command | Primary Layer | Why |
|---|---|---|---|
| `simple_chatbot` | `cargo run -p simple_chatbot` | Capability | Minimal agent loop with provider abstraction |
| `graph_demo` | `cargo run -p graph_demo` | Core | Graph orchestration and typed state flow |
| `http_server` | `cargo run -p http_server` | Runtime | Runtime execution exposed through service entrypoint |
| `mcp_demo` | `cargo run -p mcp_demo` | Platform | MCP protocol integration |
| `a2a_demo` | `cargo run -p a2a_demo` | Platform | A2A multi-agent interoperability |

Use this table with `docs/quickstart-layered.md` to choose the right starting point.
