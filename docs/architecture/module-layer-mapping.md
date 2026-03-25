# Module-to-Layer Mapping

This document assigns one primary layer to each main Nuro module.

| Module | Path | Primary Layer | Collaborates With |
|---|---|---|---|
| `nuro-core` | `crates/nuro-core` | Core | Capability, Runtime, Platform |
| `nuro-graph` | `crates/nuro-graph` | Core | Runtime, Capability |
| `nuro-runtime` | `crates/nuro-runtime` | Runtime | Core, Capability, Platform |
| `nuro-llm` | `crates/nuro-llm` | Capability | Core, Runtime, Platform |
| `nuro-tools` | `crates/nuro-tools` | Capability | Core, Runtime |
| `nuro-memory` | `crates/nuro-memory` | Capability | Runtime, Platform |
| `nuro-rag` | `crates/nuro-rag` | Capability | Runtime, Platform |
| `nuro-server` | `crates/nuro-server` | Platform | Runtime, Capability |
| `nuro-mcp` | `crates/nuro-mcp` | Platform | Runtime, Capability |
| `nuro-a2a` | `crates/nuro-a2a` | Platform | Runtime, Capability |
| `nuro` | `nuro` | Platform | Core, Capability, Runtime |

Source of truth artifact: `specs/001-runtime-workflow-reposition/artifacts/module-ownership.json`.
