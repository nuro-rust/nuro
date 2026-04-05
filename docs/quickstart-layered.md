# Layer-First Quickstart

This guide helps users adopt Nuro progressively by framework layer.

## Step 1: Start with Core + Capability

Run minimal examples:

```bash
cargo run -p simple_chatbot
cargo run -p graph_demo
```

Outcome: understand loop + graph + capability composition.

## Step 2: Move to Runtime

Run runtime-oriented flow:

```bash
cargo run -p http_server
```

Outcome: understand execution lifecycle and service exposure.

## Step 3: Move to Platform Integration

Run protocol and integration examples:

```bash
cargo run -p mcp_demo
cargo run -p a2a_demo
```

Outcome: understand external interoperability and platform boundary.

## Step 4: Add Runtime Persistence and Tracing

Use runtime stores and middleware in your app bootstrap:

- `SqliteEventStore` for event persistence
- `SqliteCheckpointStore` for resume checkpoints
- `TracingMiddleware` for structured runtime hooks

Outcome: get a minimal production-oriented execution baseline.

You can also verify graph checkpoint/resume with:

```bash
# first run (save checkpoints)
cargo run -p graph_demo -- left

# resume from a checkpointed node
cargo run -p graph_demo -- left start
```

For session/run replay, use `ReplayEngine` with `SqliteEventStore` in your app code and call:

- `replay_session(session_id, ReplayMode::Strict)`
- `replay_session_run(session_id, run_id, ReplayMode::Lenient)`

## Decision Aid

- Need strict typed orchestration first: start with Core.
- Need out-of-box model/tool/memory ability: add Capability.
- Need production lifecycle controls: adopt Runtime.
- Need server/protocol/operator integration: adopt Platform.
