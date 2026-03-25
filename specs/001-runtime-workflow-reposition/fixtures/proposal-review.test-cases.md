# Proposal Review Test Cases

## Positive

- [x] `proposal-review.valid.json` passes required fields checks.
- [x] `non_breaking` classification passes without migration guide.

## Negative

- [x] `proposal-review.breaking.invalid.json` fails because `requires_migration_guide` is false.
- [x] Missing required arrays (`test_plan` / `doc_plan` / `acceptance_criteria`) fails validation.
