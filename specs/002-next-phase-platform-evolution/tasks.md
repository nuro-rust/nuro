# Tasks: Next Phase Platform Evolution

**Input**: `specs/002-next-phase-platform-evolution/spec.md`  
**目标**: 将 reposition 后的四层架构推进为可执行的平台工程路线。

## P0 Tasks（Phase 1：把路线做实）

### Workstream: Unified Runtime

- [X] **定义 Unified Runtime 最小接口（RuntimeExecutable/RuntimeContext）**
  - Priority: P0
  - Owner: `@runtime-owner`（placeholder）
  - Deliverable: runtime 接口 RFC + `crates/nuro-runtime` API 草案
  - Acceptance: Graph 与 Agent 均可挂接统一接口并通过编译与最小集成测试
  - Dependency: 无

- [X] **落地 GraphTool / AgentNode / SubGraphNode 的实装语义**
  - Priority: P0
  - Owner: `@graph-owner`（placeholder）
  - Deliverable: `crates/nuro-graph` 节点适配实现与示例
  - Acceptance: `AgentNode` 非占位；示例可展示 Graph 与 Agent 双向组合
  - Dependency: Unified Runtime 最小接口

### Workstream: Event & Replay

- [X] **定义 Event Schema v1（含 version/correlation/causation）**
  - Priority: P0
  - Owner: `@events-owner`（placeholder）
  - Deliverable: `docs/architecture/` 下事件模型文档 + schema 文件
  - Acceptance: 核心事件类型可覆盖 AgentLoop + Graph 关键生命周期
  - Dependency: 无

- [X] **实现 SQLite Event Store 最小版**
  - Priority: P0
  - Owner: `@storage-owner`（placeholder）
  - Deliverable: EventStore trait + SQLite 实现 + 基础迁移脚本
  - Acceptance: 可持久化/读取/按 session 查询事件流
  - Dependency: Event Schema v1

### Workstream: Context / Memory / Checkpoint

- [X] **拆分 AgentContext 与 SessionContext 边界**
  - Priority: P0
  - Owner: `@runtime-owner`（placeholder）
  - Deliverable: context 数据模型与 API 变更提案
  - Acceptance: 能表达 session id、run id、metadata、resume token 等关键字段
  - Dependency: Unified Runtime 最小接口

- [X] **实现 SQLite Checkpoint Store 与最小 Resume 流程**
  - Priority: P0
  - Owner: `@storage-owner`（placeholder）
  - Deliverable: checkpoint 持久化实现 + resume 示例
  - Acceptance: 任务中断后可按 checkpoint 恢复并验证状态一致性
  - Dependency: SessionContext 模型

### Workstream: Observability / Middleware

- [X] **建立最小 Middleware Pipeline（tracing/policy hook）**
  - Priority: P0
  - Owner: `@obs-owner`（placeholder）
  - Deliverable: middleware trait + runtime 接入
  - Acceptance: 至少支持 before/after invoke 链路与错误透传
  - Dependency: Unified Runtime 最小接口

- [X] **补齐基础 tracing hooks 与示例链路**
  - Priority: P0
  - Owner: `@obs-owner`（placeholder）
  - Deliverable: tracing 字段规范 + 端到端示例日志输出
  - Acceptance: 单次执行可关联 request/session/node/tool 关键字段
  - Dependency: Middleware Pipeline

### Workstream: DX / Tooling

- [X] **更新 Phase 1 文档与 examples（对齐 001）**
  - Priority: P0
  - Owner: `@dx-owner`（placeholder）
  - Deliverable: `README.md`、`docs/quickstart-layered.md`、`docs/examples/layer-index.md` 更新
  - Acceptance: 新增 unified runtime 场景示例并可按层导航运行
  - Dependency: Unified Runtime + SQLite store 最小能力

## P1 Tasks（Phase 2：工程化闭环）

### Workstream: Event & Replay

- [X] **实现 Replay Engine（按 session/run 回放）**
  - Priority: P1
  - Owner: `@events-owner`（placeholder）
  - Deliverable: replay API + deterministic 回放策略文档
  - Acceptance: 至少 1 条端到端流程可回放并复现关键状态
  - Dependency: Event Store + Checkpoint Store

### Workstream: ADK

- [X] **创建 `nuro-adk` crate（最小可用）**
  - Priority: P1
  - Owner: `@adk-owner`（placeholder）
  - Deliverable: crate 结构、模板入口、基础 builder
  - Acceptance: 新项目可通过 ADK 模板启动并运行一个 workflow
  - Dependency: Unified Runtime 接口稳定

- [X] **提供 ADK quickstart 与应用模板**
  - Priority: P1
  - Owner: `@adk-owner`（placeholder）
  - Deliverable: quickstart 文档 + starter 项目
  - Acceptance: 文档到运行成功链路 <= 15 分钟
  - Dependency: `nuro-adk` crate

### Workstream: Ecosystem Integrations

- [X] **扩展 Provider Adapter（至少新增 1 流式 provider）**
  - Priority: P1
  - Owner: `@ecosystem-owner`（placeholder）
  - Deliverable: provider adapter 契约与实现
  - Acceptance: 支持 tool-call + streaming 事件对齐到统一 event model
  - Dependency: Event Schema v1

- [X] **实现 Postgres Store（event/checkpoint）**
  - Priority: P1
  - Owner: `@storage-owner`（placeholder）
  - Deliverable: Postgres backend + migration + 测试
  - Acceptance: 与 SQLite 共用同一 store trait 并通过一致性测试
  - Dependency: SQLite store 基线

### Workstream: Runtime Server

- [X] **增强 Runtime Server：session/task/replay API**
  - Priority: P1
  - Owner: `@server-owner`（placeholder）
  - Deliverable: API 定义 + 实现 + 示例客户端
  - Acceptance: 可创建会话、执行任务、查询回放状态
  - Dependency: Event/Checkpoint/Replay 能力

- [X] **补齐 metrics/token accounting/structured logging**
  - Priority: P1
  - Owner: `@obs-owner`（placeholder）
  - Deliverable: 指标字典、成本统计、结构化日志输出规范
  - Acceptance: 服务端可按 session 和 provider 聚合基础成本指标
  - Dependency: Middleware + Event Model

## P2 Tasks（Phase 3：平台化增强）

### Workstream: Context / Memory / Checkpoint

- [X] **实现 HITL / approval / interrupt-resume 完整链路**
  - Priority: P2
  - Owner: `@runtime-owner`（placeholder）
  - Deliverable: 状态机模型 + API + 演示流程
  - Acceptance: 人工审批可中断执行并在批准后恢复
  - Dependency: Session + Replay + Runtime Server API

### Workstream: Observability / Middleware

- [X] **引入 Policy/Governance 中间件**
  - Priority: P2
  - Owner: `@policy-owner`（placeholder）
  - Deliverable: policy rule 引擎接口与审计输出
  - Acceptance: 可配置拒绝策略并产出可审计记录
  - Dependency: Middleware Pipeline 稳定

### Workstream: DX / Tooling

- [X] **交付 Trace Viewer（最小版）**
  - Priority: P2
  - Owner: `@dx-owner`（placeholder）
  - Deliverable: trace 查询与可视化原型
  - Acceptance: 可按 session 查看 graph/agent/tool 时间线
  - Dependency: Event Store + Replay API

- [X] **交付 Playground / Evals / Cookbook 套件**
  - Priority: P2
  - Owner: `@dx-owner`（placeholder）
  - Deliverable: playground 原型、评测模板、场景化 cookbook
  - Acceptance: 至少覆盖 3 个典型场景并支持回归评估
  - Dependency: ADK + Runtime Server + Observability 基线

### Workstream: Ecosystem Integrations

- [X] **增强 MCP / A2A 互操作能力**
  - Priority: P2
  - Owner: `@ecosystem-owner`（placeholder）
  - Deliverable: 互操作协议增强与示例
  - Acceptance: 跨协议任务协作可观测、可回放、可中断恢复
  - Dependency: Runtime Server + Event/Replay 能力
