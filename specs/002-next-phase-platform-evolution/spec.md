# Spec: Next Phase Platform Evolution

## Meta

- Spec ID: `002-next-phase-platform-evolution`
- Status: Draft
- Created: 2026-03-27
- Owner Layer: Runtime（主）+ Platform（协同）
- Related Specs:
  - `specs/001-runtime-workflow-reposition/spec.md`
  - `specs/001-runtime-workflow-reposition/plan.md`
  - `specs/001-runtime-workflow-reposition/tasks.md`
- Key References:
  - `README.md`
  - `docs/architecture/layer-model.md`
  - `docs/architecture/module-layer-mapping.md`
  - `docs/quickstart-layered.md`
  - `docs/examples/layer-index.md`
  - `docs/QUICKSTART-ADVANCED.md`
  - `.specify/memory/constitution.md`

## Background

Nuro 已完成从“Rust Agent SDK”到“Rust Native Agent Runtime & Workflow Framework”的定位升级，并在 `001-runtime-workflow-reposition` 中建立了四层架构（Core/Capability/Runtime/Platform）、模块归属与治理基线。当前仓库已经具备分层入口、基础示例、快速开始路径和规范模板，说明“定位”工作已完成。

下一阶段重点不是再次命名或重做战略叙事，而是把定位做实：将 Runtime 与 Platform 的关键能力从“概念可描述”推进到“工程可执行、可验证、可扩展”的状态，形成可持续拆分的执行路线与后续 spec 基线。

## Current State Assessment

### 已有能力（基于仓库现状）

- 架构与治理层面：
  - 已有四层架构文档与模块映射：`docs/architecture/layer-model.md`、`docs/architecture/module-layer-mapping.md`
  - 已有定位升级 spec、plan、tasks 与 contracts：`specs/001-runtime-workflow-reposition/`
  - 已有 `.specify` 规范模板与宪章（兼容优先、Rust-Native、可观测性、文档与示例作为发布门禁）
- 代码能力层面：
  - Core：`nuro-core` 提供 Agent/LLM/Tool/Event 基础类型
  - Runtime：`nuro-runtime` 提供 `AgentLoop`、基础 Hook/Guardrail
  - Graph：`nuro-graph` 提供 `StateGraph`、`FnNode`、`AgentNode`、`Checkpointer` 接口
  - Platform：`nuro-server` 提供最小 HTTP/SSE 服务
  - Integrations：`nuro-mcp`、`nuro-a2a`
- DX 与示例：
  - 分层 quickstart 与 example index 已建立：`docs/quickstart-layered.md`、`docs/examples/layer-index.md`
  - examples 覆盖 simple_chatbot / graph_demo / http_server / mcp_demo / a2a_demo

### 当前落地缺口（基于代码与文档）

- Event 模型仍偏最小化，缺统一 schema/versioning、event store、replay 规范。
- Graph 与 Agent 的运行时抽象尚未统一：`AgentNode` 仍是占位映射，Graph/Agent 上下文模型未打通。
- Checkpoint 主要是 `InMemoryCheckpointer`，缺 SQLite/Postgres 等可生产持久化路径。
- Session/Resume/HITL 没有体系化抽象，`AgentContext` 当前主要是 metadata 容器。
- `nuro-server` 为最小服务骨架，缺策略、租户隔离、任务生命周期与更完整协议面。
- Observability 仍处于基础日志阶段，指标、token accounting、结构化追踪链路未形成标准。
- `nuro-adk` 尚缺位（或未形成可用工程包）。
- Provider/Store/Tool Adapter 生态仍薄，缺统一适配契约与扩展注册机制。
- DX 工具体系不足：trace viewer/playground/cookbook/evals 尚未形成闭环。

## Problem Statement

在完成 reposition 后，Nuro 已有“方向正确的框架骨架”，但离“生产落地的运行时平台”仍有系统性差距，主要问题如下：

1. Graph 与 Agent 尚未形成统一 runtime 抽象，导致组合能力与运行语义不一致。
2. Event model / store / replay 缺乏统一标准，难以支撑审计、回放、调试与稳定演进。
3. Checkpoint / Resume / Session / HITL 尚未形成端到端体系。
4. `nuro-adk` 缺位或不完整，应用开发与复用范式不稳定。
5. Observability / Middleware / Policy 能力不足，生产可运维性不足。
6. Provider / Store / Tool Adapter 生态不充分，难以快速扩展业务场景。
7. Runtime Server 能力仍偏 demo 化，缺少生产级服务治理特性。
8. DX（Trace Viewer / Playground / Cookbook / Eval）能力不足，影响采用和迭代效率。

## Goals

1. 在不破坏 001 基线与兼容策略的前提下，建立下一阶段平台演进路线图。
2. 明确四层架构下的“模块建设优先级 + 里程碑 + 依赖关系”。
3. 形成可继续拆分的 follow-up specs 清单与验收口径。
4. 以事件驱动、图编排优先、强类型、生产落地为主线推进能力建设。
5. 参考成熟框架（如 Eino 的组件化/编排/中断恢复/工程生态实践）补齐关键能力短板，但保持 Nuro 的 Rust-Native 路径与分层边界。

## Non-Goals

- 不进行新一轮定位重命名或品牌叙事重构。
- 不在本 spec 中一次性交付全部平台能力实现。
- 不引入未经迁移方案约束的破坏性 API 改动。
- 不将本 spec 写成框架对比报告。

## Architecture Vision

### Core Layer（稳定抽象层）

下一步补齐：

- 统一的 Runtime Trait 合同（Graph/Agent 共用运行语义）
- 事件元模型基础类型（event id、causation/correlation、schema version）
- Context 抽象基元（AgentContext/SessionContext 的最小公共部分）

### Capability Layer（能力组件层）

下一步补齐：

- GraphTool、AgentNode、SubGraphNode 的能力化适配器
- Provider/Tool/Store Adapter 抽象协议（注册、配置、错误模型）
- 面向应用开发的 ADK 组件层（builder、模板、最佳实践）

### Runtime Layer（执行与状态层）

下一步补齐：

- Unified Runtime 执行器（支持 graph-first orchestration + agent loop）
- Event Store + Replay + Checkpoint 协议
- Session 生命周期、interrupt/resume、HITL/approval 基础流程
- Middleware 链路（tracing hooks、policy hooks、retry/timeout）

### Platform Layer（服务与工程层）

下一步补齐：

- Runtime Server 增强（任务 API、会话 API、回放 API、管理面）
- Observability 平台能力（metrics、structured logging、trace 聚合）
- Registry / Playground / Trace Viewer / Eval / Cookbook
- MCP/A2A 互操作增强与生态接入工具

## Scope

### In Scope

- 下一阶段（Phase 1~3）的路线、优先级、依赖与验收标准。
- 8 个关键 workstream 的目标与交付定义。
- P0/P1/P2 的任务分层和里程碑基线。
- 后续可拆分 specs 的推荐清单。

### Out of Scope

- 全量代码实现细节与每个 crate 的 API 逐行设计。
- UI 产品化细节（如完整 dashboard 信息架构）。
- 与外部框架的逐项 benchmark 报告。

## Phases

### Phase 1：把路线做实（Foundation Solidification）

重点：

- Unified Runtime 抽象（Graph + Agent 统一运行接口）
- `GraphTool` / `AgentNode` / `SubGraphNode` 明确语义并可运行
- `AgentContext` / `SessionContext` 分层建模
- SQLite checkpoint 与 event store 最小可用实现
- 最小 middleware 与 tracing hooks
- 文档与 examples 补齐，明确与 001 的执行承接关系

阶段产出：

- 可落地的最小统一 runtime 垂直切片（含示例、测试、文档）

### Phase 2：工程化闭环（Engineering Closure）

重点：

- event model/versioning/replay/checkpoint 体系化
- `nuro-adk`（项目脚手架、组件组合、标准集成模式）
- Postgres store 支持
- runtime server 增强（任务控制、会话管理、回放）
- metrics / token accounting / structured logging
- provider / adapter 生态扩展

阶段产出：

- 面向生产集成团队的工程化闭环能力（非 demo）

### Phase 3：平台化增强（Platformization）

重点：

- HITL / approval / interrupt-resume 完整链路
- policy / governance
- registry
- graph / trace 可视化
- playground / evals / cookbook
- 更完整的 MCP / A2A 互操作能力

阶段产出：

- 面向平台团队与复杂多 Agent 场景的可运营能力集合

## Workstreams

### 1) Unified Runtime

- Goals:
  - 统一 Graph 与 Agent 的执行抽象与上下文契约
- Tasks:
  - 定义 `RuntimeExecutable`/`RuntimeNode`/`RuntimeContext` 最小接口
  - 完成 `GraphTool`、`AgentNode`、`SubGraphNode` 的真实映射实现
  - 提供 graph-first + agent-loop 混编示例
- Deliverables:
  - 统一 runtime RFC + crate API 草案 + 示例 + 测试
- Acceptance Criteria:
  - 同一上下文可在 graph 与 agent 节点间传递并保持类型安全
  - `AgentNode` 不再是占位实现

### 2) Event & Replay

- Goals:
  - 形成可追踪、可回放、可扩展的事件标准
- Tasks:
  - 定义 event schema/version 与兼容策略
  - 引入 event store trait + SQLite/Postgres 实现路线
  - 支持 replay 与 deterministic debug 的最小流程
- Deliverables:
  - `event-model` 规范文档、store API、replay 示例
- Acceptance Criteria:
  - 核心流程可通过 event log 回放复现关键步骤

### 3) Context / Memory / Checkpoint

- Goals:
  - 让 session、checkpoint、resume 成为统一生命周期能力
- Tasks:
  - 设计 `SessionContext` 与 `AgentContext` 的边界
  - 抽象 checkpoint metadata 与恢复策略
  - 打通 runtime-server 与 checkpoint/resume API
- Deliverables:
  - context/checkpoint 模型文档、持久化实现、恢复流程测试
- Acceptance Criteria:
  - 失败中断后可按 session 精确恢复，且可验证一致性

### 4) ADK

- Goals:
  - 形成应用开发工具层，降低接入成本
- Tasks:
  - 规划 `nuro-adk` crate 边界（模板、组合器、运行配置）
  - 提供最小 project template 与 starter
  - 建立 capability/runtime/platform 的推荐接入路径
- Deliverables:
  - `nuro-adk` 最小可用版本 + quickstart
- Acceptance Criteria:
  - 新项目可在 10~15 分钟内从模板运行首个可观测 agent workflow

### 5) Observability / Middleware

- Goals:
  - 建立生产可观测与可治理的执行管线
- Tasks:
  - middleware pipeline（timeout/retry/policy/tracing）
  - 统一 structured logging 字段约定
  - token/accounting 与 metrics 指标面
- Deliverables:
  - middleware API、指标清单、日志规范、仪表验证脚本
- Acceptance Criteria:
  - 核心调用链具备 trace 关联、成本统计、失败诊断信息

### 6) Ecosystem Integrations

- Goals:
  - 扩大 Provider/Store/Tool Adapter 可用面
- Tasks:
  - provider 接口能力分级（sync/stream/tool-call）
  - store adapter（SQLite/Postgres/可选向量后端）
  - tool adapter 标准（schema、权限、超时、幂等约束）
- Deliverables:
  - adapter contract、至少 2 类 provider 和 2 类 store 的参考实现
- Acceptance Criteria:
  - 通过统一 adapter 契约接入新后端，无需改动 core 语义

### 7) Runtime Server

- Goals:
  - 从最小 demo server 升级为可用于服务化接入的 runtime gateway
- Tasks:
  - 增加 session/task/replay/interrupt API
  - 明确认证、限流、错误码与多租户边界（最小实现）
  - 引入 server-side middleware 与审计日志
- Deliverables:
  - server API 文档、端到端示例、基础压测报告
- Acceptance Criteria:
  - server 支撑长会话任务与恢复流程，接口语义稳定

### 8) DX / Tooling

- Goals:
  - 建立“开发-调试-验证-复盘”闭环体验
- Tasks:
  - trace viewer（最小版）
  - playground（示例驱动）
  - cookbook（按场景）
  - evals 基线（回归 + 能力评估）
- Deliverables:
  - DX 门户文档、可运行工具原型、评测模板
- Acceptance Criteria:
  - 团队可基于统一工具完成问题复盘与版本回归

## Milestones

- M1（Phase 1 完成）: Unified Runtime + SQLite event/checkpoint + 基础 middleware + 文档示例到位
- M2（Phase 2 完成）: event/replay 标准化 + `nuro-adk` + Postgres + server 增强 + observability 闭环
- M3（Phase 3 完成）: HITL/policy/registry/visual tooling + MCP/A2A 平台化增强

## Prioritization

### P0（必须先做）

- Unified Runtime 抽象与节点适配器实装
- SQLite event store + checkpoint 最小闭环
- `AgentContext`/`SessionContext` 分层
- 最小 middleware + tracing hooks
- 与 001 对齐的文档与 examples 更新

### P1（工程闭环关键）

- event model versioning + replay
- `nuro-adk` 最小可用版
- Postgres store
- runtime server 增强
- metrics/token accounting/structured logging

### P2（平台化增益）

- HITL/approval/interrupt-resume 体系
- policy/governance/registry
- graph+trace 可视化、playground、evals、cookbook
- MCP/A2A 深化互操作

## Risks

- 抽象过早风险：Unified Runtime 设计若过度泛化，会损失可实现性。
- 兼容性风险：event schema 和 context 模型变更可能影响现有 API 语义。
- 范围膨胀风险：Phase 2/3 容易把“平台产品化”与“runtime 核心建设”混在一起。
- 质量风险：新增能力跨 crate 边界较多，测试和文档若不同步会造成回归。
- 生态风险：adapter 接口不稳定会导致外部接入方反复改造。

## Success Metrics

### Adoption

- 新项目使用分层 quickstart + ADK 在 15 分钟内跑通首个 workflow 的成功率 >= 85%
- 每个阶段至少新增 2 个面向真实场景的示例（非 toy）

### Engineering

- Unified Runtime 相关核心路径具备完整单元/集成测试，关键路径覆盖率达到仓库门禁要求
- 关键能力（event/replay/checkpoint/server）具备可复现验收脚本
- 变更均通过 `fmt/clippy/test/doc` 质量门禁

### Platform Readiness

- Runtime Server 支持 session/task/replay 基础 API 并通过端到端验证
- 观测链路具备 trace 关联 + token/accounting + 结构化日志
- HITL interrupt/resume 至少覆盖一个完整可演示流程

## Open Questions

1. Unified Runtime 的 owning crate 应放在 `nuro-runtime` 还是新增 `nuro-runtime-core`？
2. Event schema 版本策略采用“单全局版本”还是“按事件族版本”？
3. `nuro-adk` 是独立 crate 还是先以 `nuro` 内模块形态孵化？
4. Runtime Server 的认证/租户能力首版做到什么深度才不阻塞主线？
5. Replay 的确定性边界如何定义（涉及外部 provider/tool side effects）？

## Recommended Follow-up Specs

- `003-unified-runtime`
- `004-event-model-and-replay`
- `005-checkpoint-and-session`
- `006-nuro-adk`
- `007-runtime-server`
- `008-observability-and-tooling`
- `009-ecosystem-integrations`

## Immediate Next Actions

1. 在 `003-unified-runtime` 中冻结最小接口与迁移策略（1 周内）。
2. 在 `004-event-model-and-replay` 中定义 event schema v1 + SQLite store（1~2 周）。
3. 在 `005-checkpoint-and-session` 中完成 SessionContext 与 resume 语义（并行推进）。
4. 为 `nuro-server` 增补任务/会话 API 草案并补一条端到端示例链路。
5. 建立 Phase 1 验收看板：每个 workstream 至少 1 条“可运行、可回放、可观测”演示用例。
