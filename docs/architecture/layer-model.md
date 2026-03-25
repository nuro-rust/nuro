# Nuro Four-Layer Model

Nuro is positioned as a **Rust Native Agent Runtime & Workflow Framework**.

## Layers

### Core Layer

- Scope: `core`, `runtime`, `graph`, `events`
- Goal: minimal, stable, strongly typed foundations for orchestration.
- Boundary: avoid embedding optional vendor or platform concerns.

### Capability Layer

- Scope: `llm`, `tools`, `memory`, `rag`, `planner`, `policy`
- Goal: provide out-of-the-box capabilities on top of core contracts.
- Boundary: capabilities integrate with runtime, but do not own runtime lifecycle.

### Runtime Layer

- Scope: `checkpoint`, `scheduler`, `executor`, `session`, `human-in-loop`
- Goal: production-grade task execution lifecycle.
- Boundary: runtime owns execution semantics and operational safeguards.

### Platform Layer

- Scope: `server`, `tracing`, `dashboard`, `playground`, `devtools`
- Goal: integration, observability, debugging, and operations.
- Boundary: platform exposes and operates the framework; it does not redefine core contracts.

## Ownership Rules

- Each module has exactly one primary ownership layer.
- Cross-layer behavior is represented as collaboration edges.
- Proposals must declare owned layer and compatibility classification.

## Compatibility Policy

- This positioning rollout is non-breaking.
- Future breaking changes require migration guidance and release notes.
