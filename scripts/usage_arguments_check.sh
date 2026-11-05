#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
gate_python_bin=$(python3.13 -c 'from pathlib import Path; import sys; print(Path(sys.executable).resolve().parent)')
export PATH="$gate_python_bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
export CARGO_BUILD_JOBS=2
cargo build --locked -p biomcp --bin biomcp
exec python3 scripts/test_argument_recovery.py "${CARGO_TARGET_DIR:-target}/debug/biomcp" "${1:-check}"
