#!/bin/sh
# Build and exercise source-scoped study rows with the installed outside-in runner.
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  generate)
    cargo check --locked --workspace
    make schema-generate docs-generate
    tar -czf /tmp/biomcp-study-rows-generated.tar.gz crates/biomcp-types/schemas docs/reference/commands.md docs/recipes.md skills/SKILL.md docs/reference/source-pacing.md
    pwd
    ;;
  list)
    python3 scripts/trials.py --list | rg study
    python3 scripts/trials.py --target tools --proofs --list | rg study
    ;;
  focused)
    shift
    python3 scripts/trials.py --installed "$@"
    ;;
  tools)
    shift
    python3 scripts/trials.py --target tools --proofs --installed "$@"
    ;;
  scans)
    cargo fmt --all -- --check
    cargo clippy --locked -p biomcp-adapters -p biomcp-query -p biomcp-types --all-targets -- -D warnings
    ;;
  routine)
    exec sh scripts/check_nightly_tests.sh routine
    ;;
  *) echo 'usage: check_study_rows.sh [generate|list|focused NAMES...|tools NAMES...|scans|routine]' >&2; exit 2 ;;
esac
