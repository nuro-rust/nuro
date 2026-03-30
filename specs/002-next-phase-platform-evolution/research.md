# Phase 2 Research: Event Replay Deterministic Strategy

## 决策

采用两级回放策略：

1. **Strict 模式**
   - 任一事件 `schema_version` 非 `v1` 直接失败
   - 时间戳逆序直接失败
2. **Lenient 模式**
   - 保留事件并记录 warnings
   - 允许在排障场景下先观察再修复

## 原因

- 当前 Nuro 处于工程化闭环阶段，回放需要先覆盖核心链路。
- 先定义可执行的最小确定性规则（版本 + 时间顺序），避免过早引入复杂的 side-effect 仿真。

## 约束

- 回放输入以 `session_id` 为主，可选 `run_id` 过滤。
- 当前 store 默认按插入顺序回放。
- 外部 provider/tool 的非确定性副作用暂不在 v1 范围内。

## 后续演进

- 引入 causation/correlation 完整链路一致性检查。
- 引入 side-effect policy（跳过/模拟/强制失败）。
- 在 Runtime Server 中暴露回放 API 与回放报告。
