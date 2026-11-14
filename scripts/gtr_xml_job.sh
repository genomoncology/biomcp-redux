#!/usr/bin/env bash
# 0931: regenerate declarations, run the gate or the GTR cases, or time one live GTR sync.
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
  gate)
    make --keep-going landing-gate docs-check schema-release-check
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
  rss)
    # Sample the sync's resident memory every 0.25 s to show which phase sets the peak.
    if [ -n "${2:-}" ]; then git checkout "$2" -- crates Cargo.toml Cargo.lock; fi
    cargo build --locked --release -p biomcp --bin biomcp
    home=$(mktemp -d)
    export XDG_CACHE_HOME="$home/cache" XDG_CONFIG_HOME="$home/config" XDG_DATA_HOME="$home/data"
    target/release/biomcp sync gtr > "$home/sync.json" &
    pid=$!
    start=$(date +%s)
    while kill -0 "$pid" 2>/dev/null; do
      rss=$(ps -o rss= -p "$pid" 2>/dev/null || true)
      net=$(nettop -P -L 1 -p "$pid" -J bytes_in 2>/dev/null | tail -1 || true)
      echo "t=$(( $(date +%s) - start )) rss_kib=${rss:-gone} net=${net:-?}"
      sleep 0.25
    done
    wait "$pid"
    cat "$home/sync.json"
    ;;
  *) exit 2 ;;
esac
