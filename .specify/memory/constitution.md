<!--
Sync Impact Report
- Version change: 1.0.0 -> 2.0.0
- Modified principles:
  - I. Rust-Native Performance First -> I. Incremental Evolution and Compatibility First
  - II. Extension-First Architecture (Plugin + Event) -> II. Rust-Native Performance and Type Safety
  - III. Quality Gates Are Non-Negotiable -> III. Stable Core Abstractions and Modular Architecture
  - IV. Documentation and Demo Are Product Surface -> IV. Operational Reliability and Observability by Default
  - V. Operational Readiness by Default -> V. Documentation, Examples, and Developer Experience as Release Gates
- Added sections:
  - Engineering Scope and Mandatory Capability Baseline
  - Iteration Workflow and Release Discipline
- Removed sections:
  - Engineering Standards and Constraints
  - Development Workflow and Quality Gates
- Templates requiring updates:
  - ✅ updated: .specify/templates/plan-template.md
  - ✅ updated: .specify/templates/spec-template.md
  - ✅ updated: .specify/templates/tasks-template.md
  - ⚠ pending: .specify/templates/commands/*.md (directory not present; no action required now)
  - ✅ updated: README.md
- Follow-up TODOs:
  - None
-->

# Nuro Constitution

## Core Principles

### I. Incremental Evolution and Compatibility First
Nuro MUST evolve from the existing repository baseline through incremental,
backward-compatible changes whenever possible. New work MUST begin with a
current-state assessment of crate boundaries, API surface, examples, tests, CI,
and docs, followed by an explicit iteration roadmap. Breaking changes MUST be
treated as exceptional and MUST include deprecation path, migration guide, and
release notes rationale.

Rationale: Nuro is a long-lived SDK, not a one-off project; trust depends on
smooth upgrades and continuity for existing users.

### II. Rust-Native Performance and Type Safety
Core SDK capabilities MUST be implemented in stable Rust and MUST preserve
zero-cost abstractions in critical paths. Public APIs MUST favor explicit trait
contracts, strong typing, and predictable async behavior while minimizing
unnecessary clone, allocation, boxing, and dynamic dispatch overhead. Hot-path
changes MUST include benchmark or profiling evidence.

Rationale: Nuro must provide Rust-native performance and safety advantages over
frameworks in dynamic-language ecosystems.

### III. Stable Core Abstractions and Modular Architecture
The following abstractions MUST remain clear, low-coupled, and independently
evolvable: agent, model/provider, tool, memory, workflow/graph,
runtime/context, event/observer, config, and tracing/telemetry. High-level
features MUST be built on these stable abstractions, not by cross-module
tight coupling. Modules MUST maintain single responsibility, clear entry points,
tests, examples, and module-level documentation.

Rationale: A composable SDK requires stable foundations before feature breadth.

### IV. Operational Reliability and Observability by Default
All externally relevant paths MUST provide consistent error taxonomy, structured
logging, tracing correlation, and configuration layering (file + env + code/CLI).
Provider and tool integrations MUST include timeout, retry, and failure isolation
mechanisms where applicable. Event models MUST remain stable and extensible to
support debugging, monitoring, and audit subscribers with low intrusion.

Rationale: Production-grade agent systems require diagnosable behavior under
failure, scale, and multi-component orchestration.

### V. Documentation, Examples, and Developer Experience as Release Gates
Every new feature MUST ship with tests, documentation, and runnable examples in
the same iteration. The documentation system MUST include README, quickstart,
core concepts, architecture docs, module docs, migration guidance, and FAQ.
Public APIs MUST include rustdoc, and examples MUST remain executable and
validation-ready in CI. The default developer journey MUST enable a first working
agent in about 10 minutes.

Rationale: Adoption depends on discoverability, clarity, and confidence, not
only on internal code quality.

## Engineering Scope and Mandatory Capability Baseline

- Nuro is a Rust ecosystem Agent SDK targeting single-agent and multi-agent,
  workflow orchestration, tool usage, memory, event-driven execution,
  observability, plugin extension, and production deployment.
- The project MUST evolve the existing workspace; redesign-from-scratch proposals
  are non-compliant unless explicitly approved as a governance exception.
- Core capability tracks MUST be planned and implemented across:
  agent, provider, tool, workflow/graph, memory, runtime/context,
  event/observer, observability, configuration, and compatibility/versioning.
- Async-first design is required, but heavyweight runtime coupling MUST be
  optional where feasible through feature gates or sync/async layering.
- Added dependencies MUST be justified with necessity and impact.
- Feature flags MUST be used to prevent over-heavy default builds.

## Iteration Workflow and Release Discipline

Before implementation of any new feature, teams MUST produce a short
"Current State Assessment + Iteration Roadmap" covering existing capability,
gaps, duplicate design risks, and potential breaking-change impact.

Each implementation spec MUST explicitly include:

1. Background
2. Goal
3. Non-goals
4. API design
5. Module changes
6. Compatibility impact
7. Test plan
8. Documentation plan
9. Acceptance criteria

Merge and release quality gates MUST include:

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -D warnings`
- `cargo test --all --all-features`
- `cargo doc --no-deps` for public API documentation validation
- Coverage evidence showing core module target remains above 90%
- Changelog update for every release-oriented change
- Migration guide update for deprecations and breaking changes

## Governance

This constitution overrides conflicting local practices for the Nuro repository.

- Amendment process: changes MUST be proposed in a documented PR that includes
  impact analysis, compatibility impact, migration notes (if applicable), and
  updates to affected templates and guidance docs.
- Approval policy: at least one maintainer review is required, and constitution
  changes MUST not be merged with unresolved governance TODOs.
- Versioning policy: constitution versions follow semantic versioning.
  - MAJOR: backward-incompatible governance or principle redefinition/removal.
  - MINOR: new principle/section or materially expanded obligations.
  - PATCH: wording clarifications or non-semantic refinements.
- Compliance review: every implementation plan, spec, and task list MUST include
  an explicit constitution alignment check before execution and before merge.

Release governance for the SDK MUST follow semver, deprecation notices, and
changelog discipline. If a release includes breaking API changes, maintainers
MUST publish migration guidance before or with the release.

**Version**: 2.0.0 | **Ratified**: 2026-03-25 | **Last Amended**: 2026-03-25
