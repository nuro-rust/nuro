# Feature Specification: [FEATURE NAME]

**Feature Branch**: `[###-feature-name]`  
**Created**: [DATE]  
**Status**: Draft  
**Input**: User description: "$ARGUMENTS"

## Iteration Brief *(mandatory)*

### Background

[Describe the current repository context and why this iteration exists now]

### Goal

[Define the concrete outcome for this iteration]

### Non-Goals

[List explicitly out-of-scope items to prevent over-design]

### Current State Assessment

- **Touched Modules/Crates**: [List existing modules]
- **Owned Layer**: [core | capability | runtime | platform]
- **Existing API/Behavior Baseline**: [What exists today]
- **Known Gaps/Risks**: [Missing capability, duplicate design, compatibility risk]

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.
  
  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - [Brief Title] (Priority: P1)

[Describe this user journey in plain language]

**Why this priority**: [Explain the value and why it has this priority level]

**Independent Test**: [Describe how this can be tested independently - e.g., "Can be fully tested by [specific action] and delivers [specific value]"]

**Acceptance Scenarios**:

1. **Given** [initial state], **When** [action], **Then** [expected outcome]
2. **Given** [initial state], **When** [action], **Then** [expected outcome]

---

### User Story 2 - [Brief Title] (Priority: P2)

[Describe this user journey in plain language]

**Why this priority**: [Explain the value and why it has this priority level]

**Independent Test**: [Describe how this can be tested independently]

**Acceptance Scenarios**:

1. **Given** [initial state], **When** [action], **Then** [expected outcome]

---

### User Story 3 - [Brief Title] (Priority: P3)

[Describe this user journey in plain language]

**Why this priority**: [Explain the value and why it has this priority level]

**Independent Test**: [Describe how this can be tested independently]

**Acceptance Scenarios**:

1. **Given** [initial state], **When** [action], **Then** [expected outcome]

---

[Add more user stories as needed, each with an assigned priority]

### Edge Cases

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right edge cases.
-->

- What happens when [boundary condition]?
- How does system handle [error scenario]?

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001**: System MUST [specific capability, e.g., "allow users to create accounts"]
- **FR-002**: System MUST [specific capability, e.g., "validate email addresses"]  
- **FR-003**: Users MUST be able to [key interaction, e.g., "reset their password"]
- **FR-004**: System MUST [data requirement, e.g., "persist user preferences"]
- **FR-005**: System MUST [behavior, e.g., "log all security events"]

*Example of marking unclear requirements:*

- **FR-006**: System MUST authenticate users via [NEEDS CLARIFICATION: auth method not specified - email/password, SSO, OAuth?]
- **FR-007**: System MUST retain user data for [NEEDS CLARIFICATION: retention period not specified]

### API Design *(mandatory)*

- Public API additions/changes: [functions, traits, structs, builders]
- Naming and discoverability considerations: [consistency notes]
- Async/sync layering or feature-gate behavior: [design notes]

### Module Changes *(mandatory)*

- [module/crate path]: [change summary]
- [module/crate path]: [change summary]

### Compatibility and Versioning Impact *(mandatory)*

- Change classification:
  - [ ] Non-breaking enhancement
  - [ ] Deprecation but compatible
  - [ ] Breaking change
- Deprecation markers and replacement APIs: [required if deprecating]
- Migration strategy/guide updates: [required for breaking or major deprecation]
- Changelog impact: [entry scope]
- Layer collaboration notes: [which other layers are touched and why]

### Key Entities *(include if feature involves data)*

- **[Entity 1]**: [What it represents, key attributes without implementation]
- **[Entity 2]**: [What it represents, relationships to other entities]

## Constitution Alignment *(mandatory)*

- **CA-001 Rust-Native**: Describe how this feature preserves Rust-native,
  zero-cost abstraction expectations for critical paths.
- **CA-002 Extensibility**: Define plugin and/or event extension points and how
  business customization avoids modifying unrelated core modules.
- **CA-003 Quality**: Define testing strategy and how repository coverage stays
  above 90% after this feature.
- **CA-004 Operability**: Define error handling (including codes), logging
  strategy, and layered configuration (config file + env + CLI args).
- **CA-005 Documentation & Demo**: List required API docs, user/developer guide
  updates, and runnable basic/advanced demo coverage.
- **CA-006 Compatibility**: Explain semver impact and deprecation/migration plan.
- **CA-007 Reliability**: Define timeout/retry/error-isolation behavior for
  external provider/tool interactions.
- **CA-008 Layer Ownership**: Feature declares one owning layer and explicit
  cross-layer collaboration boundaries.

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: [Measurable metric, e.g., "Users can complete account creation in under 2 minutes"]
- **SC-002**: [Measurable metric, e.g., "System handles 1000 concurrent users without degradation"]
- **SC-003**: [User satisfaction metric, e.g., "90% of users successfully complete primary task on first attempt"]
- **SC-004**: [Business metric, e.g., "Reduce support tickets related to [X] by 50%"]
