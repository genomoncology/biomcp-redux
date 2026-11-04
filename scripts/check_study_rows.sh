#!/bin/sh
# Build and exercise source-scoped study rows with the installed outside-in runner.
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  generate)
    cargo check --locked --workspace
    make schema-generate docs-generate
    tar -czf /tmp/biomcp-study-rows-generated.tar.gz crates/biomcp-types/schemas docs/reference/commands.md
    pwd
    ;;
  list)
    python3 scripts/trials.py --list
    ;;
  focused)
    python3 scripts/trials.py --installed study/rows study/help study/examples study/retired
    ;;
  routine)
    exec sh scripts/check_nightly_tests.sh routine
    ;;
  *) echo 'usage: check_study_rows.sh [generate|list|focused|routine]' >&2; exit 2 ;;
esac
