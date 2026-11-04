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
