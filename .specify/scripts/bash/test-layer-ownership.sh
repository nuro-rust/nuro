#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
FEATURE_DIR="$ROOT_DIR/specs/001-runtime-workflow-reposition"

"$ROOT_DIR/.specify/scripts/bash/validate-layer-contracts.sh" \
  "$FEATURE_DIR/fixtures/layer-classification.valid.json" \
  "$FEATURE_DIR/fixtures/proposal-review.valid.json" \
  "$FEATURE_DIR/fixtures/proposal-review.breaking.invalid.json"

python3 - "$FEATURE_DIR/artifacts/module-ownership.json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, "r", encoding="utf-8") as f:
    records = json.load(f)

if not isinstance(records, list) or not records:
    raise SystemExit("module-ownership.json must be a non-empty array")

names = set()
for rec in records:
    name = rec.get("module_name")
    layer = rec.get("primary_layer")
    collab = rec.get("collaborates_with_layers", [])
    if not name or not layer:
        raise SystemExit("record missing module_name/primary_layer")
    if name in names:
        raise SystemExit(f"duplicate module_name: {name}")
    names.add(name)
    if layer in collab:
        raise SystemExit(f"module {name} has primary layer inside collaboration list")

print("OK: module ownership records validated")
PY
