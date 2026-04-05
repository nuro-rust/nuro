# Nuro Event Model v1

本文档定义 Nuro 在下一阶段统一 runtime 中使用的事件模型 v1。

## Envelope

每个事件都包含统一 envelope 字段：

- `event_id`: 全局唯一事件标识（字符串）
- `schema_version`: 当前固定为 `v1`
- `timestamp_ms`: 毫秒时间戳
- `session_id`: 会话 ID（可选）
- `run_id`: 一次运行 ID（可选）
- `correlation_id`: 关联链路 ID（可选）
- `causation_id`: 因果上游事件 ID（可选）
- `metadata`: 扩展元数据（JSON Object）
- `kind`: 事件类型负载

## EventKind（最小集）

- `LlmRequest`
- `LlmResponse`
- `ToolCallStart`
- `ToolCallEnd`

以上最小集合覆盖当前 `AgentLoop` 的 THINK/ACT/OBSERVE 主路径，后续可按兼容策略增量扩展。

## Versioning Policy

1. 新增字段优先采用 optional，避免破坏读取方。
2. 删除字段或语义重定义必须升级 schema major 版本。
3. `EventKind` 新增枚举值属于非破坏增强，但消费端应提供未知类型降级处理。

## Replay Baseline

- 事件回放最小要求：支持按 `session_id + run_id` 顺序读取事件流。
- 回放结果需可还原关键节点：LLM 调用边界、Tool 调用边界、错误边界。
