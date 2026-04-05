# Provider Adapter Contract (Phase 2)

## Scope

定义 provider adapter 的最小能力契约，用于统一文本生成、流式输出和 tool-call 事件对齐。

## Core Interfaces

- `LlmProviderAdapter`
  - `descriptor() -> ProviderDescriptor`
  - `stream_generate(request) -> Vec<ProviderStreamEvent>`
  - `stream_events_to_runtime_events(session_id, run_id, stream_events) -> Vec<Event>`

## Capability Model

- `ProviderCapability::TextGeneration`
- `ProviderCapability::Streaming`
- `ProviderCapability::ToolCall`

## Stream Event Model

- `TextDelta(String)`
- `ToolCallStart { name, input }`
- `ToolCallEnd { name, output }`
- `Done`

## Acceptance

1. 至少一个 provider 支持 `Streaming` capability。
2. stream events 可映射到 runtime event（`LlmResponse` / `ToolCallStart` / `ToolCallEnd`）。
3. session/run 上下文透传后可用于 replay 检索。
