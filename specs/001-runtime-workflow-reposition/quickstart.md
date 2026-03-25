# Quickstart: Four-Layer Positioning Rollout

This quickstart verifies that maintainers, users, and contributors can use the new Nuro framework positioning end to end.

## 1) Maintainer flow: publish layer model and ownership

1. Open `README.md` and architecture docs.
2. Confirm Nuro positioning is stated as: `Rust Native Agent Runtime & Workflow Framework`.
3. Confirm the four layers are documented with goals and boundaries.
4. Confirm each core module/crate appears once with one primary ownership layer.
5. Confirm cross-layer behavior is written as collaboration edges, not dual ownership.
6. Confirm ownership artifact exists at `specs/001-runtime-workflow-reposition/artifacts/module-ownership.json` and aligns with `specs/001-runtime-workflow-reposition/contracts/layer-classification.schema.json`.

Expected result:

- A maintainer can answer "which layer owns this module?" for every listed core module.

## 2) User flow: start minimal and scale by layer

1. Start with Core/Capability examples:
   - `cargo run -p simple_chatbot`
   - `cargo run -p graph_demo`
2. Move to Runtime/Platform-oriented examples:
   - `cargo run -p http_server`
   - `cargo run -p mcp_demo`
   - `cargo run -p a2a_demo`
3. Verify docs explain why each example belongs to its layer and when to adopt next-layer capabilities.

Expected result:

- A new user can pick a starting layer and locate a matching runnable example within 10 minutes.

## 3) Contributor flow: submit a layer-aware proposal

1. Create a proposal/spec update containing:
   - background and goal
   - non-goals
   - owned layer and boundary
   - API impact
   - compatibility classification
   - test/doc plans
   - acceptance criteria
2. Validate proposal metadata against contracts in `specs/001-runtime-workflow-reposition/contracts/`.
3. Ensure classification is `non_breaking` unless migration/deprecation details are included.

Expected result:

- Reviewers can accept/reject with objective checks, not interpretation-only discussion.

## 4) Quality gate checks

Run:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -D warnings
cargo test --all --all-features
cargo doc --no-deps
```

Expected result:

- No regressions from framework repositioning work.

## 5) Release checklist

- Changelog entry states this as a non-breaking framework positioning enhancement.
- Docs include migration-by-concept guidance (no forced code migration in this iteration).
- Governance templates/contracts are published and linked from contributor workflow docs.
