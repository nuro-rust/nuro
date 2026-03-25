#!/usr/bin/env bash

set -euo pipefail

if [ "$#" -lt 2 ]; then
  echo "Usage: $0 <layer-classification.json> <proposal-review.json> [proposal-review-invalid.json]" >&2
  exit 1
fi

LAYER_JSON="$1"
PROPOSAL_JSON="$2"
PROPOSAL_INVALID_JSON="${3:-}"

python3 - "$LAYER_JSON" "$PROPOSAL_JSON" "$PROPOSAL_INVALID_JSON" <<'PY'
import json
import sys

layer_path = sys.argv[1]
proposal_path = sys.argv[2]
proposal_invalid_path = sys.argv[3] if len(sys.argv) > 3 else ""

LAYERS = {"core", "capability", "runtime", "platform"}
COMPAT = {"non_breaking", "deprecation_compatible", "breaking"}

def load(path):
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)

def ensure(cond, msg):
    if not cond:
        raise ValueError(msg)

def validate_layer(obj):
    required = {
        "module_name", "module_path", "primary_layer", "responsibilities",
        "collaborates_with_layers", "compatibility_classification"
    }
    ensure(required.issubset(obj.keys()), "layer-classification missing required keys")
    ensure(obj["primary_layer"] in LAYERS, "invalid primary_layer")
    ensure(obj["compatibility_classification"] in COMPAT, "invalid compatibility_classification")
    ensure(isinstance(obj["responsibilities"], list) and len(obj["responsibilities"]) > 0, "responsibilities must be non-empty array")
    ensure(isinstance(obj["collaborates_with_layers"], list), "collaborates_with_layers must be array")
    ensure(obj["primary_layer"] not in obj["collaborates_with_layers"], "collaborates_with_layers must not include primary_layer")
    for layer in obj["collaborates_with_layers"]:
        ensure(layer in LAYERS, "invalid collaborate layer value")

def validate_proposal(obj):
    required = {
        "proposal_id", "feature_title", "owned_layer", "scope_boundary",
        "api_impact", "compatibility_classification", "test_plan",
        "doc_plan", "acceptance_criteria"
    }
    ensure(required.issubset(obj.keys()), "proposal-review missing required keys")
    ensure(obj["owned_layer"] in LAYERS, "invalid owned_layer")
    ensure(obj["compatibility_classification"] in COMPAT, "invalid compatibility_classification")
    ensure(isinstance(obj["test_plan"], list) and len(obj["test_plan"]) > 0, "test_plan must be non-empty")
    ensure(isinstance(obj["doc_plan"], list) and len(obj["doc_plan"]) > 0, "doc_plan must be non-empty")
    ensure(isinstance(obj["acceptance_criteria"], list) and len(obj["acceptance_criteria"]) > 0, "acceptance_criteria must be non-empty")

    cls = obj["compatibility_classification"]
    if cls == "breaking":
        ensure(obj.get("requires_migration_guide") is True, "breaking proposal must set requires_migration_guide=true")
    if cls == "deprecation_compatible":
        ensure(isinstance(obj.get("replacement_path"), str) and len(obj.get("replacement_path", "")) > 0, "deprecation_compatible must provide replacement_path")

layer = load(layer_path)
proposal = load(proposal_path)

validate_layer(layer)
validate_proposal(proposal)

if proposal_invalid_path:
    invalid = load(proposal_invalid_path)
    try:
        validate_proposal(invalid)
    except ValueError:
        pass
    else:
        raise ValueError("expected invalid proposal fixture to fail validation")

print("OK: contract validation passed")
PY
