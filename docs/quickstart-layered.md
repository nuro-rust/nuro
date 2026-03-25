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

## Decision Aid

- Need strict typed orchestration first: start with Core.
- Need out-of-box model/tool/memory ability: add Capability.
- Need production lifecycle controls: adopt Runtime.
- Need server/protocol/operator integration: adopt Platform.
