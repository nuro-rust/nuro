# Data Model: Nuro Runtime & Workflow Repositioning

## Entity: LayerDefinition

- Purpose: Defines each of the four framework layers and their intent.
- Fields:
  - `layer_id` (string, enum): `core`, `capability`, `runtime`, `platform`
  - `display_name` (string): Human-readable layer name
  - `goal` (string): Value proposition of the layer
  - `entry_criteria` (array[string]): Conditions for modules/features to belong primarily to this layer
  - `out_of_scope` (array[string]): Boundaries that must remain outside this layer
  - `stability_level` (string, enum): `stable`, `evolving`, `experimental`
- Validation rules:
  - `layer_id` MUST be unique.
  - `goal` MUST be non-empty.
  - `entry_criteria` MUST contain at least one item.

## Entity: ModuleOwnershipRecord

- Purpose: Captures the primary layer ownership and collaboration edges for each module.
- Fields:
  - `module_name` (string): crate/module identifier, e.g. `nuro-runtime`
  - `module_path` (string): repository-relative path
  - `primary_layer` (enum LayerDefinition.layer_id)
  - `responsibilities` (array[string]): canonical responsibilities
  - `collaborates_with_layers` (array[enum LayerDefinition.layer_id])
  - `public_surface_notes` (string): discoverability/API positioning notes
  - `compatibility_classification` (enum): `non_breaking`, `deprecation_compatible`, `breaking`
- Validation rules:
  - Every module MUST map to exactly one `primary_layer`.
  - `collaborates_with_layers` MUST NOT include `primary_layer`.
  - `compatibility_classification` MUST match release notes labeling.

## Entity: CompatibilityClassification

- Purpose: Normalized compatibility impact used by specs, PRs, and changelog entries.
- Fields:
  - `classification` (enum): `non_breaking`, `deprecation_compatible`, `breaking`
  - `behavior_change_summary` (string)
  - `requires_migration_guide` (boolean)
  - `replacement_path` (string, optional)
- Validation rules:
  - If `classification = breaking`, then `requires_migration_guide = true`.
  - If `classification = deprecation_compatible`, `replacement_path` MUST be provided.

## Entity: AdoptionPath

- Purpose: Guides users from minimal setup to production capability.
- Fields:
  - `persona` (enum): `new_user`, `sdk_integrator`, `platform_operator`, `contributor`
  - `start_layer` (enum LayerDefinition.layer_id)
  - `target_layer` (enum LayerDefinition.layer_id)
  - `steps` (array[string])
  - `linked_examples` (array[string])
  - `success_check` (string)
- Validation rules:
  - `steps` MUST contain at least 2 items.
  - `target_layer` MUST be the same as or later than `start_layer` in documented progression.

## Entity: ProposalReviewRecord

- Purpose: Required metadata block for future feature proposals/reviews.
- Fields:
  - `proposal_id` (string)
  - `feature_title` (string)
  - `owned_layer` (enum LayerDefinition.layer_id)
  - `scope_boundary` (string)
  - `api_impact` (string)
  - `compatibility_classification` (enum CompatibilityClassification.classification)
  - `test_plan` (array[string])
  - `doc_plan` (array[string])
  - `acceptance_criteria` (array[string])
- Validation rules:
  - `owned_layer`, `scope_boundary`, and `compatibility_classification` are mandatory.
  - `test_plan`, `doc_plan`, and `acceptance_criteria` MUST be non-empty arrays.

## Relationships

- LayerDefinition `1 -> many` ModuleOwnershipRecord
- ModuleOwnershipRecord `many -> many` LayerDefinition (collaboration only)
- ModuleOwnershipRecord `many -> 1` CompatibilityClassification
- AdoptionPath references LayerDefinition for progression constraints
- ProposalReviewRecord references LayerDefinition and CompatibilityClassification

## State Transitions

### ModuleOwnershipRecord lifecycle

`draft` -> `reviewed` -> `published`

- `draft -> reviewed`: Requires maintainer review with ownership/boundary checks.
- `reviewed -> published`: Requires docs update merged and changelog impact classification confirmed.

### ProposalReviewRecord lifecycle

`submitted` -> `triaged` -> `approved` -> `implemented` -> `released`

- `submitted -> triaged`: Layer ownership assigned.
- `triaged -> approved`: Compatibility/test/doc plans complete.
- `approved -> implemented`: Tasks created and traceable.
- `implemented -> released`: Quality gates and release notes complete.
