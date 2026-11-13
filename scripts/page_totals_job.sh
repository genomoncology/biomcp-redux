#!/usr/bin/env bash
# Regenerate declarations for page totals, keep the patch, then measure every gate stage.
set -euo pipefail
patch="${2:?persistent patch destination}"
case "${1:-}" in
  measure)
    make schema-generate docs-generate
    cargo build --locked -p biomcp
    python3 scripts/refresh_help_cases.py target/debug/biomcp
    git diff --binary > "$patch"
    git diff --stat
    status=0
    make --keep-going rights audit secrets pre-job first-read-check workflow-lint cases \
      docs-check schema-release-check || status=$?
    exit "$status"
    ;;
  *) exit 2 ;;
esac
