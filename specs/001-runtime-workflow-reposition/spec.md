# Feature Specification: Nuro Runtime & Workflow Repositioning

**Feature Branch**: `001-runtime-workflow-reposition`  
**Created**: 2026-03-25  
**Status**: Draft  
**Input**: User description: "把 Nuro 从 Rust Agent SDK 升级定位为 Rust Native Agent Runtime & Workflow Framework，并分成 Core / Capability / Runtime / Platform 四层"

## Iteration Brief *(mandatory)*

### Background

Nuro 当前以 Rust Agent SDK 进行定位，能力点较多但对外认知更偏向“组件集合”。
为了支撑长期演进与生产落地，需要将项目定位升级为
“Rust Native Agent Runtime & Workflow Framework”，并形成统一的分层心智模型，
让用户、贡献者与维护者在能力边界、扩展路径与发布稳定性上达成一致。

### Goal

建立并发布 Nuro 的四层框架定位与模块边界：

- Core Layer: core / runtime / graph / events，保持最小、稳定、强类型
- Capability Layer: llm / tools / memory / rag / planner / policy，提供开箱即用能力
- Runtime Layer: checkpoint / scheduler / executor / session / human-in-loop，
  提供生产级任务执行
- Platform Layer: server / tracing / dashboard / playground / devtools，
  提供调试、运维与集成能力

该迭代目标是完成统一定位、分层映射、兼容演进策略与验收基线，
并确保后续功能可按层迭代。

### Non-Goals

- 不在本迭代内完成所有模块的重写或大规模迁移
- 不在本迭代内强制修改现有公共 API 的调用方式
- 不在本迭代内交付完整平台产品能力（如完整 dashboard）
- 不改变项目“增量迭代、兼容优先、可平滑升级”的治理原则

### Current State Assessment

- **Touched Modules/Crates**: core、runtime、graph、llm、tools、memory、rag、server、mcp、a2a 及相关文档与示例索引
- **Existing API/Behavior Baseline**: 已具备 agent loop、graph 编排、tool 抽象、provider 抽象、memory 与 server 等基础能力
- **Known Gaps/Risks**: 对外定位与层级边界尚未统一；部分能力归属可能重叠，存在后续演进中的职责漂移与兼容风险

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 分层定位统一发布 (Priority: P1)

作为项目维护者，我希望 Nuro 有清晰且稳定的四层定位与模块映射，
从而让后续迭代都能按统一边界推进并减少架构分歧。

**Why this priority**: 这是所有后续能力建设与发布管理的前置条件，
不先统一边界会导致持续的设计偏差和重复投入。

**Independent Test**: 审阅对外文档与模块映射清单，确认每个核心模块都被归入单一主层级，
并带有职责说明与边界规则。

**Acceptance Scenarios**:

1. **Given** 维护者准备发布新迭代定位，**When** 查看分层文档，**Then** 能看到四层定义、目标与模块归属规则
2. **Given** 存在历史模块，**When** 对照层级映射清单，**Then** 能明确其主归属层与跨层协作边界

---

### User Story 2 - 用户按层选择能力 (Priority: P2)

作为 SDK 使用者，我希望按层快速理解 Nuro 能力版图，
从而根据业务复杂度选择最小可用能力并平滑升级。

**Why this priority**: 能力可发现性直接影响接入效率与用户信心。

**Independent Test**: 让新用户仅通过文档导航完成“按层选型”任务，
验证其能够识别基础层、能力层、运行时层与平台层的差异与适用场景。

**Acceptance Scenarios**:

1. **Given** 新用户只想先做单 Agent，**When** 按分层导航阅读文档，**Then** 能确定最小起步层并找到对应能力入口

---

### User Story 3 - 贡献者按层扩展不破坏稳定性 (Priority: P3)

作为贡献者，我希望新功能提案能先归类到明确层级并评估兼容性影响，
从而避免跨层耦合和不必要 breaking changes。

**Why this priority**: 贡献流程标准化可降低维护成本并提升版本稳定性。

**Independent Test**: 对一个新增能力提案执行分层评审流程，
确认包含层级归属、兼容级别、迁移要求与验收标准。

**Acceptance Scenarios**:

1. **Given** 一项新能力提案，**When** 进入评审，**Then** 评审记录中包含层级归属、兼容分类和文档更新计划

---

### Edge Cases

- 当一个模块同时涉及多个层级能力时，如何确定主归属层并定义协作边界
- 当历史模块与新层级命名不一致时，如何保证用户可发现性与兼容认知
- 当提案跨越 Capability 与 Runtime 边界时，如何避免重复抽象与职责重叠
- 当平台能力尚不完整时，如何给出清晰的阶段性范围说明避免误解为已完整可用

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: 项目 MUST 将对外定位更新为“Rust Native Agent Runtime & Workflow Framework”
- **FR-002**: 项目 MUST 定义并发布四层架构模型及每层目标说明
- **FR-003**: 项目 MUST 为现有核心模块提供分层映射清单与边界描述
- **FR-004**: 每个模块 MUST 仅有一个主归属层，跨层关系 MUST 以协作关系明确标注
- **FR-005**: 功能规划与需求评审 MUST 记录层级归属、范围边界与兼容性分类
- **FR-006**: 本迭代 MUST 采用兼容优先策略，不得引入未声明的破坏性行为变化
- **FR-007**: 若存在弃用路径，项目 MUST 提供替代方案与迁移说明
- **FR-008**: 对外文档 MUST 提供按层导航方式，支持用户从最小能力到高级能力逐步采用
- **FR-009**: 示例索引 MUST 按层级关联，帮助用户快速定位对应场景示例
- **FR-010**: 验收标准 MUST 包含分层完整性、用户可发现性与兼容性可验证条件
- **FR-011**: 本次定位升级 MUST 保持现有项目演进连续性，不得采用推倒重来的交付方式
- **FR-012**: 后续迭代提案模板 MUST 要求提供背景、目标、非目标、API 影响、模块改动、兼容影响、测试计划、文档计划与验收标准

### API Design *(mandatory)*

- Public API additions/changes: 对外 API 以“语义分层可发现”为目标进行组织说明，本迭代不要求强制重命名现有公共入口
- Naming and discoverability considerations: 分层命名与能力命名保持一致语义，确保用户可据名称判断所在层级与职责
- Async/sync layering or feature-gate behavior: 保持 async-first 体验，同时确保不同复杂度场景可渐进采用而不增加不必要负担

### Module Changes *(mandatory)*

- `docs/` 与根级说明文档：增加框架定位、四层模型、能力边界与采用路径
- `specs/` 与治理模板：补充分层归属、兼容分类、验收约束，确保后续需求按层迭代
- 核心模块文档索引：补充分层映射与主归属层标注

### Compatibility and Versioning Impact *(mandatory)*

- Change classification:
  - [x] Non-breaking enhancement
  - [ ] Deprecation but compatible
  - [ ] Breaking change
- Deprecation markers and replacement APIs: 本迭代不强制引入 deprecation；若后续引入，将在对应发布周期提供替代入口
- Migration strategy/guide updates: 本迭代为定位与分层升级，迁移指南聚焦“概念与导航迁移”，不要求代码迁移
- Changelog impact: 记录为“项目定位升级 + 四层架构基线建立”

### Key Entities *(include if feature involves data)*

- **Layer Definition**: 四层模型定义，包含层级名称、目标、边界与进入条件
- **Module Ownership Record**: 模块到层级的主归属记录，包含跨层协作关系
- **Compatibility Classification**: 变更影响分类记录（非破坏增强/弃用兼容/破坏性变更）
- **Adoption Path**: 用户从最小能力到完整能力的分层采用路径

### Assumptions

- 现有模块目录与能力基础保持可复用，不需要一次性重构
- 本迭代以定位与治理基线为主，具体实现深挖在后续迭代逐步完成
- 贡献者与用户将以分层模型作为后续需求评审与文档导航的共同语言

### Dependencies

- 依赖现有模块文档与示例能够被映射到四层模型
- 依赖后续迭代在提案与评审流程中执行分层与兼容性检查

## Constitution Alignment *(mandatory)*

- **CA-001 Rust-Native**: 分层定位强调 Rust Native runtime 与 workflow 框架价值，保持强类型与性能导向
- **CA-002 Extensibility**: 通过 Core/Capability/Runtime/Platform 分层降低耦合，支持按层扩展能力
- **CA-003 Quality**: 将分层完整性与兼容分类纳入验收，要求测试、文档、示例同步更新
- **CA-004 Operability**: Runtime 与 Platform 层明确承担生产执行、调试与运维能力边界
- **CA-005 Documentation & Demo**: 要求文档与示例按层组织，支持用户快速理解与采用
- **CA-006 Compatibility**: 本次归类为非破坏增强，保持平滑升级与版本连续性
- **CA-007 Reliability**: 通过层级职责约束，减少跨层耦合导致的稳定性与可诊断性问题

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% 的现有核心模块完成四层主归属映射并具备职责说明
- **SC-002**: 新贡献提案中，100% 包含层级归属与兼容性分类信息
- **SC-003**: 新用户在 10 分钟内完成“选择起步层级并定位示例”的任务成功率达到 90% 以上
- **SC-004**: 本次发布后，因定位不清导致的重复设计/边界冲突反馈在两个迭代周期内下降 50% 以上
