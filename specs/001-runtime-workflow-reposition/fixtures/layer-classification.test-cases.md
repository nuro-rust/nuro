# Layer Classification Test Cases

## Positive

- [x] Valid object includes all required fields.
- [x] `primary_layer` is one of `core|capability|runtime|platform`.
- [x] `collaborates_with_layers` does not include `primary_layer`.

## Negative

- [x] Missing required field fails validation.
- [x] Invalid layer enum fails validation.
- [x] Self-collaboration fails validation.
