# Tasks: Nuro Runtime & Workflow Repositioning

**Input**: Design documents from `/specs/001-runtime-workflow-reposition/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: This feature requires contract and integration validation tasks because the spec defines measurable acceptance and governance compliance outcomes.

**Organization**: Tasks are grouped by user story so each story can be implemented and validated independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Task can run in parallel (different files, no blocking dependency)
- **[Story]**: User story label (`[US1]`, `[US2]`, `[US3]`) for story-phase tasks
- Every task includes an exact file path

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare artifacts and validation scaffolding for layer repositioning work.

- [X] T001 Create rollout workspace notes in `specs/001-runtime-workflow-reposition/implementation-notes.md`
- [X] T002 [P] Create ownership artifact directory `specs/001-runtime-workflow-reposition/artifacts/`
- [X] T003 [P] Create contract fixture directory `specs/001-runtime-workflow-reposition/fixtures/`
- [X] T004 [P] Create validation script directory `.specify/scripts/bash/`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish shared contracts and governance plumbing required by all user stories.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T005 Create module-layer ownership baseline in `specs/001-runtime-workflow-reposition/artifacts/module-ownership.json`
- [X] T006 [P] Create adoption path baseline in `specs/001-runtime-workflow-reposition/artifacts/adoption-paths.json`
- [X] T007 [P] Create proposal review baseline in `specs/001-runtime-workflow-reposition/artifacts/proposal-review-template.json`
- [X] T008 Implement contract validation script for both schemas in `.specify/scripts/bash/validate-layer-contracts.sh`
- [X] T009 [P] Add valid classification fixture in `specs/001-runtime-workflow-reposition/fixtures/layer-classification.valid.json`
- [X] T010 [P] Add valid proposal fixture in `specs/001-runtime-workflow-reposition/fixtures/proposal-review.valid.json`
- [X] T011 [P] Add invalid proposal fixture for breaking-migration rule in `specs/001-runtime-workflow-reposition/fixtures/proposal-review.breaking.invalid.json`

**Checkpoint**: Shared artifacts and schema validation are ready for story-specific delivery.

---

## Phase 3: User Story 1 - 分层定位统一发布 (Priority: P1) 🎯 MVP

**Goal**: Publish authoritative four-layer model and primary ownership mapping for existing modules.

**Independent Test**: A maintainer can answer layer ownership and collaboration boundary for every mapped core module using repository documentation and ownership artifacts.

### Tests for User Story 1

- [X] T012 [P] [US1] Add ownership contract validation test script in `.specify/scripts/bash/test-layer-ownership.sh`
- [X] T013 [P] [US1] Add ownership test fixtures coverage notes in `specs/001-runtime-workflow-reposition/fixtures/layer-classification.test-cases.md`

### Implementation for User Story 1

- [X] T014 [US1] Update project positioning statement in `README.md`
- [X] T015 [US1] Add four-layer model and boundaries doc in `docs/architecture/layer-model.md`
- [X] T016 [US1] Add module-to-layer mapping table in `docs/architecture/module-layer-mapping.md`
- [X] T017 [US1] Link ownership artifact and governance contract in `specs/001-runtime-workflow-reposition/quickstart.md`
- [X] T018 [US1] Record non-breaking classification for repositioning in `CHANGELOG.md`

**Checkpoint**: Maintainer-facing positioning and ownership rules are published and verifiable.

---

## Phase 4: User Story 2 - 用户按层选择能力 (Priority: P2)

**Goal**: Provide layer-based onboarding path so users can start small and scale by complexity.

**Independent Test**: A new user can select a starting layer and find runnable examples for that layer within 10 minutes using docs only.

### Tests for User Story 2

- [X] T019 [P] [US2] Add onboarding navigation validation checklist in `specs/001-runtime-workflow-reposition/checklists/user-navigation.md`
- [X] T020 [P] [US2] Add quickstart scenario assertions for layer-example mapping in `specs/001-runtime-workflow-reposition/checklists/quickstart-validation.md`

### Implementation for User Story 2

- [X] T021 [US2] Add layer-first getting started section in `README.md`
- [X] T022 [US2] Add progressive adoption guide in `docs/quickstart-layered.md`
- [X] T023 [US2] Add example-to-layer index in `docs/examples/layer-index.md`
- [X] T024 [US2] Add advanced quickstart cross-links to layer model in `docs/QUICKSTART-ADVANCED.md`

**Checkpoint**: User-facing docs enable layer-first discovery and progressive adoption.

---

## Phase 5: User Story 3 - 贡献者按层扩展不破坏稳定性 (Priority: P3)

**Goal**: Standardize proposal/review flow to enforce layer ownership and compatibility-first delivery.

**Independent Test**: A contributor can submit a proposal record that passes schema validation and includes owned layer, compatibility class, test plan, and doc plan.

### Tests for User Story 3

- [X] T025 [P] [US3] Add proposal review contract validation test script in `.specify/scripts/bash/test-proposal-review.sh`
- [X] T026 [P] [US3] Add proposal metadata pass/fail examples in `specs/001-runtime-workflow-reposition/fixtures/proposal-review.test-cases.md`

### Implementation for User Story 3

- [X] T027 [US3] Add layer-aware proposal checklist template in `specs/001-runtime-workflow-reposition/checklists/proposal-review.md`
- [X] T028 [US3] Update planning template with layer ownership gate in `.specify/templates/plan-template.md`
- [X] T029 [US3] Update specification template with owned-layer field in `.specify/templates/spec-template.md`
- [X] T030 [US3] Update task template with compatibility/migration task requirement in `.specify/templates/tasks-template.md`

**Checkpoint**: Contributor workflow enforces layer ownership and compatibility classification.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final consistency, verification, and release readiness across stories.

- [X] T031 [P] Run full quickstart verification and capture evidence in `specs/001-runtime-workflow-reposition/validation-report.md`
- [X] T032 [P] Normalize terminology across docs in `docs/Nuro_—_Rust_Agent_SDK_技术架构设计文档.lark.md`
- [X] T033 Run schema validation and checklist aggregation in `specs/001-runtime-workflow-reposition/checklists/requirements.md`
- [X] T034 Finalize release notes and compatibility statement in `CHANGELOG.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: Can start immediately.
- **Phase 2 (Foundational)**: Depends on Phase 1 and blocks all user stories.
- **Phase 3 (US1)**: Starts after Phase 2; delivers MVP.
- **Phase 4 (US2)**: Starts after Phase 2; can run in parallel with US1 if staffed, but usually follows US1 for narrative coherence.
- **Phase 5 (US3)**: Starts after Phase 2; can run in parallel with US2.
- **Phase 6 (Polish)**: Starts after selected user stories are complete.

### User Story Dependencies

- **US1 (P1)**: No dependency on other user stories.
- **US2 (P2)**: Depends on US1 positioning language for consistent user-facing navigation.
- **US3 (P3)**: Depends on US1 ownership model and contracts; independent from US2 content work.

### Within Each User Story

- Test tasks MUST be written before implementation tasks.
- Contract and checklist updates MUST happen before final story sign-off.
- Story documentation updates MUST include direct links to relevant contracts/artifacts.

### Parallel Opportunities

- T002, T003, T004 can run in parallel.
- T006, T007, T009, T010, T011 can run in parallel after T005.
- In US1, T012 and T013 can run in parallel.
- In US2, T019 and T020 can run in parallel.
- In US3, T025 and T026 can run in parallel.
- T031 and T032 can run in parallel in final phase.

---

## Parallel Example: User Story 1

```bash
# Launch US1 tests together:
Task: "T012 [US1] in .specify/scripts/bash/test-layer-ownership.sh"
Task: "T013 [US1] in specs/001-runtime-workflow-reposition/fixtures/layer-classification.test-cases.md"

# Then implement docs in sequence:
Task: "T014 [US1] in README.md"
Task: "T015 [US1] in docs/architecture/layer-model.md"
Task: "T016 [US1] in docs/architecture/module-layer-mapping.md"
```

## Parallel Example: User Story 2

```bash
# Launch US2 validation assets together:
Task: "T019 [US2] in specs/001-runtime-workflow-reposition/checklists/user-navigation.md"
Task: "T020 [US2] in specs/001-runtime-workflow-reposition/checklists/quickstart-validation.md"

# Then publish user-facing docs:
Task: "T021 [US2] in README.md"
Task: "T022 [US2] in docs/quickstart-layered.md"
Task: "T023 [US2] in docs/examples/layer-index.md"
```

## Parallel Example: User Story 3

```bash
# Launch US3 contract tests together:
Task: "T025 [US3] in .specify/scripts/bash/test-proposal-review.sh"
Task: "T026 [US3] in specs/001-runtime-workflow-reposition/fixtures/proposal-review.test-cases.md"

# Then update governance templates:
Task: "T028 [US3] in .specify/templates/plan-template.md"
Task: "T029 [US3] in .specify/templates/spec-template.md"
Task: "T030 [US3] in .specify/templates/tasks-template.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 and Phase 2.
2. Deliver US1 tasks (T012-T018).
3. Validate maintainer flow from quickstart and ownership artifacts.
4. Release as non-breaking positioning baseline.

### Incremental Delivery

1. Ship US1 for governance and ownership clarity.
2. Ship US2 for user onboarding and example discoverability.
3. Ship US3 for contributor proposal/review enforcement.
4. Run polish phase and finalize release notes.

### Parallel Team Strategy

1. Team A: Foundational contracts and validation scripts (Phase 2).
2. Team B: US1 + US2 documentation and onboarding paths.
3. Team C: US3 governance templates and proposal workflow checks.
4. Merge in story-complete increments with independent validation evidence.

---

## Notes

- All task lines follow required checklist format: checkbox, task ID, optional `[P]`, required `[USx]` for story phases, and explicit file path.
- MVP scope recommendation: **US1 only** (Phase 3) after Setup + Foundational.
- This task plan intentionally keeps changes incremental and compatibility-first.
