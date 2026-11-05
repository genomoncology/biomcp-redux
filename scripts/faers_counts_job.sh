#!/bin/sh
# Exercise FAERS reaction counts at the caller boundary.
set -eu
cd "$(dirname "$0")/.."
faers_python_bin=$(python3.13 -c 'from pathlib import Path; import sys; print(Path(sys.executable).resolve().parent)')
export PATH="$faers_python_bin:$HOME/.cargo/bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  generate)
    cargo fmt --all
    make schema-generate docs-generate
    ;;
  focused)
    cargo test --locked -p biomcp-types --features schema --test adverse_event_contracts
    python3.13 scripts/trials.py --case-file adverse-event/faers-reaction-counts.yaml --case-file adverse-event/examples.yaml --case-file adverse-event/faers-several-names.yaml --case-file adverse-event/help.yaml
    PYTHONPATH=. python3.13 scripts/faers_counts_mcp.py "${CARGO_TARGET_DIR:-target}/debug/biomcp"
    ;;
  routine) make landing-gate ;;
  *) exit 2 ;;
esac
