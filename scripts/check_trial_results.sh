#!/bin/sh
# Posted trial results: generated contracts and installed, source-backed cases.
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  prepare)
    python3.13 scripts/schema_release.py --generate
    make docs-generate
    cargo build --locked -p biomcp --bin biomcp
    python3.13 scripts/refresh_help_cases.py "${CARGO_TARGET_DIR:-target}/debug/biomcp"
    output="$HOME/workspace/t/biomcp-redux-0607-$(git rev-parse HEAD)"
    mkdir -p "$output"
    git diff --binary > "$output/generated.patch"
    printf 'Generated patch: %s/generated.patch\n' "$output"
    ;;
  focused)
    python3.13 scripts/trials.py --installed trial-results/posted-values-and-event-pages trial-results/no-posted-results
    cargo test --locked -p biomcp --test cases trial/get/
    cargo test --locked -p biomcp --test cases trial/sites/
    cargo test --locked -p biomcp --test cases trial/references
    cargo test --locked -p biomcp --test cases trial/help
    make docs-check
    ;;
  *) echo 'usage: check_trial_results.sh [prepare|focused]' >&2; exit 2 ;;
esac
