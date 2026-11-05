#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
gate_python_bin=$(python3.13 -c 'from pathlib import Path; import sys; print(Path(sys.executable).resolve().parent)')
export PATH="$gate_python_bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
cargo test --locked -p biomcp --test cases source-status -- --nocapture
