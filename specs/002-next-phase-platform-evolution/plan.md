# Implementation Plan: Next Phase Platform Evolution

**Branch**: `002-next-phase-platform-evolution`  
**Date**: 2026-03-27  
**Spec**: `specs/002-next-phase-platform-evolution/spec.md`

## Summary

本计划承接 `001-runtime-workflow-reposition`，目标是在兼容优先前提下，把 Nuro 的四层架构推进为可执行的平台工程路线。执行主线为：先统一 runtime 与事件/状态基线（Phase 1），再完成工程闭环（Phase 2），最后进入平台化增强（Phase 3）。

## Milestones

### Milestone 1（Phase 1 完成）

- Unified Runtime 最小接口落地
- GraphTool / AgentNode / SubGraphNode 可运行语义
- SQLite event store + checkpoint store
- AgentContext / SessionContext 分层
- 最小 middleware 与 tracing hooks
- 文档与 examples 对齐 001 并可跑通

### Milestone 2（Phase 2 完成）

- Event model versioning + replay 机制
- `nuro-adk` 最小可用版
- Postgres store
- Runtime Server（session/task/replay）增强
- metrics/token accounting/structured logging
- provider/store/tool adapter 扩展

### Milestone 3（Phase 3 完成）

- HITL/approval/interrupt-resume 完整链路
- policy/governance + registry
- graph/trace 可视化
- playground/evals/cookbook
- MCP/A2A 更完整互操作

## Dependencies

### 关键依赖链

1. Unified Runtime -> Context Model -> Event/Checkpoint Store -> Replay -> Runtime Server
2. Unified Runtime 稳定后，`nuro-adk` 才能收敛 API 形态
3. Event/Replay 能力是 Trace Viewer、Evals、HITL 审计的前提
4. Middleware 与 Observability 是 Runtime Server 工程化上线的基础约束

### 外部依赖

- SQLite/Postgres 驱动与迁移方案选型
- 观测栈（tracing + metrics）统一字段规范
- MCP/A2A 协议增强的兼容策略

## Execution Order

### 顺序策略

1. **先基础再增强**：先完成 runtime/event/checkpoint 核心闭环，再做平台化 UI 与治理层。
2. **先语义再扩展**：先冻结统一接口与语义，再扩 provider/store/adapters。
3. **先最小可运行再规模化**：每阶段至少交付一条端到端“可运行 + 可观测 + 可回放”流程。

### 并行策略

- 可并行 A：Unified Runtime 接口设计 与 Event Schema 设计
- 可并行 B：SQLite store 实现 与 文档/examples 草案准备
- 可并行 C：Phase 2 的 ADK 与 Runtime Server（在共享契约冻结后）

## Risk Control

### 风险 1：抽象过度导致交付延迟

- 控制措施：Unified Runtime 采用“最小接口优先”，先覆盖现有 AgentLoop + StateGraph 主路径。
- 触发阈值：两周内无法产出可运行样例则冻结扩展议题。

### 风险 2：兼容性回归

- 控制措施：所有接口变更附带 compatibility classification 与迁移注记；延续 001 的非破坏优先原则。
- 触发阈值：出现公共 API 破坏时必须追加迁移说明与回滚方案。

### 风险 3：Phase 2/3 范围膨胀

- 控制措施：按 P0/P1/P2 看板推进，任何 P2 不得阻塞 P0/P1 里程碑验收。
- 触发阈值：若 P1 核心未完成，暂停新增平台 UI 类需求。

### 风险 4：可观测性与调试不可用

- 控制措施：将 tracing/metrics/token accounting 列为 Phase 2 出门条件，不作为可选项。
- 触发阈值：若无法按 session 追踪核心链路，禁止进入 Phase 3。

## Acceptance Gates

- Gate A（M1）: Unified Runtime + SQLite event/checkpoint + 文档示例通过
- Gate B（M2）: replay + ADK + server 增强 + observability 闭环通过
- Gate C（M3）: HITL/policy/registry + DX tooling 与互操作增强通过

## Tracking

- 主跟踪文档：`specs/002-next-phase-platform-evolution/tasks.md`
- 后续拆分依据：`Recommended Follow-up Specs`（见 `spec.md`）
- 每周节奏：里程碑看板更新 + 风险复盘 + 依赖阻塞清理
