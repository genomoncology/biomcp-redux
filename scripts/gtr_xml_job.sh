#!/usr/bin/env bash
# 0931: regenerate declarations, run the GTR cases, or time one live GTR sync with the XML part.
set -euo pipefail
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-cases}" in
  generate)
    make schema-generate docs-generate
    cargo build --locked -p biomcp --bin biomcp
    python3 scripts/refresh_help_cases.py target/debug/biomcp
    echo '----- BEGIN PATCH -----'
    git diff --binary
    echo '----- END PATCH -----'
    ;;
  cases)
    cargo test --locked -p biomcp-fetch --test bundle
    cargo test --locked -p biomcp --test cases diagnostic/
    ;;
  live)
    # An optional revision measures that revision's code the same way, as a baseline.
    if [ -n "${2:-}" ]; then git checkout "$2" -- crates Cargo.toml Cargo.lock; fi
    cargo build --locked --release -p biomcp --bin biomcp
    home=$(mktemp -d)
    export XDG_CACHE_HOME="$home/cache" XDG_CONFIG_HOME="$home/config" XDG_DATA_HOME="$home/data"
    /usr/bin/time -l target/release/biomcp sync gtr
    target/release/biomcp get diagnostic GTR000560508.11 --source gtr
    target/release/biomcp get diagnostic GTR000575663.2 --source gtr
    /usr/bin/time -l target/release/biomcp search diagnostic --source gtr --gene EGFR --limit 50 > "$home/page.json"
    wc -c "$home/page.json"
    du -sh "$home"
    ;;
  *) exit 2 ;;
esac
