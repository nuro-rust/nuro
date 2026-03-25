# Implementation Plan: Nuro Runtime & Workflow Repositioning

**Branch**: `001-runtime-workflow-reposition` | **Date**: 2026-03-25 | **Spec**: `/Users/bytedance/workspace/nuro/specs/001-runtime-workflow-reposition/spec.md`
**Input**: Feature specification from `/Users/bytedance/workspace/nuro/specs/001-runtime-workflow-reposition/spec.md`

## Summary

Reposition Nuro from a "Rust Agent SDK" to a "Rust Native Agent Runtime & Workflow Framework" by introducing a strict four-layer model (Core, Capability, Runtime, Platform), mapping existing crates/modules to one primary layer, and codifying compatibility-first governance for future proposals. Implementation stays incremental and documentation-first: no forced API renames, no rewrite, and no unannounced breaking behavior.

## Current State Assessment (Mandatory)

- Existing crates/modules touched: `nuro/`, `crates/nuro-core`, `crates/nuro-runtime`, `crates/nuro-graph`, `crates/nuro-llm`, `crates/nuro-tools`, `crates/nuro-memory`, `crates/nuro-rag`, `crates/nuro-server`, `crates/nuro-mcp`, `crates/nuro-a2a`, plus docs/spec governance assets.
- Current API surface impacted: top-level positioning and discoverability in README/docs; contribution/review templates; module ownership metadata. No mandatory signature-level API break in this iteration.
- Existing examples/tests/docs reviewed: `README.md`, `docs/QUICKSTART-ADVANCED.md`, `docs/Nuro_—_Rust_Agent_SDK_技术架构设计文档.lark.md`, and demo crates under `examples/`.
- Gaps and duplicate-design risks: role boundaries between capability and runtime are currently easy to blur; platform wording can be mistaken as fully productized; existing module descriptions are not enforced by a single ownership contract.
- Breaking-change risk and compatibility level:
  - [x] Non-breaking enhancement
  - [ ] Deprecation with compatibility
  - [ ] Breaking change (requires migration plan)

## Technical Context

**Language/Version**: Rust workspace (Edition 2024), multi-crate architecture  
**Primary Dependencies**: `tokio`, `serde`, `serde_json`, `tracing`, `async-trait`, `axum`, `tower`, `reqwest`, `thiserror`, `anyhow`  
**Storage**: N/A for this iteration (positioning/governance/documentation feature; no new persistent data store)  
**Testing**: `cargo fmt --check`, `cargo clippy --all-targets --all-features -D warnings`, `cargo test --all --all-features`, `cargo doc --no-deps`, plus docs/structure acceptance checks  
**Target Platform**: Rust developer environments (macOS/Linux/CI) and existing runtime deployment targets  
**Project Type**: Rust SDK/framework workspace with documentation and example applications  
**Performance Goals**: Zero runtime regression on agent loop/graph/tool hot paths; no additional dynamic-dispatch overhead introduced by this iteration  
**Constraints**: Compatibility-first rollout, single primary ownership per module, no rewrite-from-scratch, explicit cross-layer collaboration labels  
**Scale/Scope**: 1 root crate + 11 core crates + examples/docs/spec templates, with framework-wide taxonomy and governance updates

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Rust-native scope: PASS. Plan is Rust-first and mainly taxonomy/governance; no critical-path runtime overhead added.
- Evolution discipline: PASS. Uses incremental remapping of existing crates and docs, explicitly avoids redesign-from-scratch.
- Extension architecture: PASS. Preserves event/tool/provider extension points and formalizes cross-layer collaboration boundaries.
- Core abstraction alignment: PASS. Keeps agent/provider/tool/memory/workflow/runtime/event/config/telemetry roles explicit in layer ownership records.
- Quality gates: PASS. Keeps full workspace checks and coverage expectations from constitution in release validation.
- Operational readiness: PASS. Runtime/platform documentation requires structured logging, tracing, and layered configuration coverage.
- Reliability controls: PASS. Runtime-facing guidance maintains timeout/retry/failure isolation expectations for providers/tools.
- Delivery completeness: PASS. Scope includes docs, examples index mapping, governance templates, and quickstart-based adoption flow.
- Release discipline: PASS. Classified as non-breaking enhancement with changelog-level positioning update.

## Project Structure

### Documentation (this feature)

```text
specs/001-runtime-workflow-reposition/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── layer-classification.schema.json
│   └── proposal-review.schema.json
└── tasks.md
```

### Source Code (repository root)

```text
nuro/
crates/
├── nuro-core/
├── nuro-runtime/
├── nuro-graph/
├── nuro-llm/
├── nuro-tools/
├── nuro-memory/
├── nuro-rag/
├── nuro-server/
├── nuro-mcp/
├── nuro-a2a/
└── nuro-macros/

docs/
examples/
specs/
```

**Structure Decision**: Use the existing Rust workspace structure and deliver this feature through docs/spec/governance artifacts anchored in `specs/001-runtime-workflow-reposition/`, then propagate approved mapping language into `README.md`, `docs/`, and contributor workflows in follow-up implementation tasks.

## Complexity Tracking

No constitution violations requiring justification.
