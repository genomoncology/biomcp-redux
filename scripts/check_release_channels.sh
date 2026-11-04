#!/bin/sh
# Focused proof-tooling checks; no compilation or real qualification.
set -eu
cd "$(dirname "$0")/.."
BIOMCP_TEST_PINNED_INSTALLER=1 BIOMCP_TEST_CLAUDE_INSTALL=1 python3.13 -m unittest release.test_installer release.test_promotion release.test_workflow release.test_windows
python3.13 scripts/test_windows_vm_lock.py
python3.13 scripts/check_shell.py
