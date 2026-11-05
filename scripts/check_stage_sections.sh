#!/bin/sh
set -eu
python3 -m unittest release.test_stage_sections release.test_workflow release.test_workflow_contracts
if command -v actionlint >/dev/null 2>&1; then
  actionlint .github/workflows/release-stage-sections.yml
else
  echo 'actionlint unavailable; workflow boundary checks ran in the focused suite.'
fi
