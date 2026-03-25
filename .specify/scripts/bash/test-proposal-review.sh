#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
FEATURE_DIR="$ROOT_DIR/specs/001-runtime-workflow-reposition"

"$ROOT_DIR/.specify/scripts/bash/validate-layer-contracts.sh" \
  "$FEATURE_DIR/fixtures/layer-classification.valid.json" \
  "$FEATURE_DIR/fixtures/proposal-review.valid.json" \
  "$FEATURE_DIR/fixtures/proposal-review.breaking.invalid.json"

echo "OK: proposal review contract tests passed"
