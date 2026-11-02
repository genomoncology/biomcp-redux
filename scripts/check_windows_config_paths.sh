#!/bin/sh
# Keep config-path development checks on the normal source-owned gate.
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
export PATH="$HOME/.local/bin:$PATH"
python3.13 --version
case "${1:-cheap}" in
    cheap) make check-cheap ;;
    focused) cargo test --locked -p biomcp --test cases housekeeping/ ;;
    routine) make landing-gate ;;
    *) exit 2 ;;
esac
