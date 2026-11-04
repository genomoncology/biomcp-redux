#!/bin/sh
# Focused proof-tooling checks; no compilation or real qualification.
set -eu
cd "$(dirname "$0")/.."
export PATH="$HOME/.local/bin:$PATH"
python3 -m unittest release.test_installer release.test_promotion release.test_workflow release.test_windows
python3 scripts/test_windows_vm_lock.py
python3 scripts/check_shell.py
