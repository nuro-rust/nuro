# Phase 0 Research: Nuro Runtime & Workflow Repositioning

## Decision 1: Adopt a four-layer model as the primary external mental model

- Decision: Standardize external positioning as Core / Capability / Runtime / Platform and require every module to have exactly one primary ownership layer.
- Rationale: A single taxonomy removes ambiguity in roadmap planning, contributor reviews, and user onboarding.
- Alternatives considered: Keep current SDK-centric narrative with ad-hoc module notes; rejected because it cannot prevent ownership drift.

## Decision 2: Keep iteration non-breaking and documentation-first

- Decision: Deliver this iteration through architecture mapping, governance contracts, and navigation updates without forcing public API renames.
- Rationale: This satisfies compatibility-first governance while still establishing a durable framework baseline.
- Alternatives considered: Rename/restructure crates immediately; rejected due to migration overhead and unnecessary churn for existing adopters.

## Decision 3: Treat cross-layer behavior as explicit collaboration, not shared ownership

- Decision: When a module spans concerns, assign one primary layer and capture other interactions as collaboration links.
- Rationale: Single ownership preserves accountability and avoids duplicate abstractions, especially at Capability vs Runtime boundaries.
- Alternatives considered: Multi-home module ownership; rejected because review and release responsibilities become ambiguous.

## Decision 4: Define governance contracts as machine-checkable schemas

- Decision: Add JSON schema contracts for layer classification and proposal review metadata.
- Rationale: Structured contracts make compliance testable and reduce subjective interpretation in PR/spec reviews.
- Alternatives considered: Free-form markdown-only templates; rejected due to weak enforceability and inconsistent reviewer output.

## Decision 5: Anchor user adoption around progressive complexity

- Decision: Document a progressive path from Core to Platform, including layer-to-example mapping.
- Rationale: New users need a fast way to start small and scale without relearning concepts.
- Alternatives considered: Capability-first onboarding only; rejected because it hides foundational execution/runtime responsibilities.

## Decision 6: Preserve Rust-native guarantees as release gates

- Decision: Reuse constitution quality gates (`fmt`, `clippy`, `test`, `doc`) and retain hot-path performance discipline in follow-up tasks.
- Rationale: Repositioning should strengthen, not dilute, Nuro's Rust-native identity.
- Alternatives considered: Restrict this iteration to docs with no engineering gates; rejected because governance change must remain release-grade.
