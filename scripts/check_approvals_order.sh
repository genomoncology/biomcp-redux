#!/bin/sh
# Source-bound submission ordering and installed paging checks.
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
python_bin=$(python3.13 -c 'from pathlib import Path; import sys; print(Path(sys.executable).resolve().parent)')
export PATH="$python_bin:$PATH"
case "${1:-focused}" in
  prepare)
    python3.13 scripts/schema_release.py --generate
    output="$HOME/workspace/t/biomcp-redux-0601-$(git rev-parse HEAD)"
    mkdir -p "$output"
    git diff --binary > "$output/generated.patch"
    printf 'Generated patch: %s/generated.patch\n' "$output"
    ;;
  focused)
    output="$HOME/workspace/t/biomcp-redux-0601-$(git rev-parse HEAD)"
    mkdir -p "$output/transcripts"
    BIOMCP_TRANSCRIPT_DIR="$output/transcripts" python3.13 scripts/trials.py --installed drug-approvals/newest-submission-pages drug-approvals/undated-and-numeric-ties
    ;;
  cheap)
    make check-cheap
    ;;
  *) echo 'usage: check_approvals_order.sh [prepare|focused|cheap]' >&2; exit 2 ;;
esac
