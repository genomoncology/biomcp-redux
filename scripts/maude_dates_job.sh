#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"
case "${1:-focused}" in
  generate)
    cargo fmt --all
    make schema-generate docs-generate
    ;;
  scans)
    baseline=${2:?scans requires baseline commit}
    mkdir -p target/0606-scans/baseline
    git archive "$baseline" crates/biomcp-types/src/adverse_event crates/biomcp-adapters/src/maude | tar -xf - -C target/0606-scans/baseline
    for side in baseline current; do
      prefix=.
      test "$side" != baseline || prefix=target/0606-scans/baseline
      npm exec --offline --yes --package=jscpd@4.0.5 -- jscpd \
        "$prefix/crates/biomcp-types/src/adverse_event" "$prefix/crates/biomcp-adapters/src/maude" \
        --format rust --max-size 10mb --reporters json --output "target/0606-scans/$side"
    done
    cargo clippy --locked -p biomcp-types -p biomcp-adapters --all-targets --features biomcp-types/schema -- -D warnings
    ruff check --select F401,F841,ERA,BLE,E722,S110 scripts/maude_dates_proof.py
    ;;
  live)
    python3.13 scripts/maude_dates_proof.py "${CARGO_TARGET_DIR:-target}/debug/biomcp" "$2" "target/0606-live"
    ;;
  list)
    python3.13 scripts/trials.py --target cases --list
    ;;
  focused)
    cargo fmt --all -- --check
    cargo test --locked -p biomcp-types --features schema --test adverse_event_contracts
    cargo test --locked -p biomcp-types --features schema --test schemas
    make current-schema-check schema-release-check
    shift
    python3.13 scripts/trials.py --target cases --installed "$@"
    ;;
  *) exit 2 ;;
esac
