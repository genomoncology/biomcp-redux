#!/bin/sh
set -eu
export PATH="/Users/ian/.local/share/uv/python/cpython-3.13.17-macos-aarch64-none/bin:$HOME/.cargo/bin:$PATH"
cargo +1.95.0 fmt --all -- --check
cargo +1.95.0 build --locked -p biomcp --bin biomcp
if [ "${1:-}" = refresh ]; then
    python3 scripts/refresh_help_cases.py "${CARGO_TARGET_DIR:-target}/debug/biomcp"
    make docs-generate
fi
cargo +1.95.0 test --locked -p biomcp --test ratchets front_map
python3 scripts/check_front_map.py "${CARGO_TARGET_DIR:-target}/debug/biomcp" --baseline /Users/ian/workspace/t/biomcp-redux-0586-benchmark-baseline-27ead/installed/bin/biomcp --output "$HOME/workspace/t/biomcp-redux-0586-proof-$(git rev-parse HEAD)"
if [ "${1:-}" = gate ]; then
    make landing-gate
fi
