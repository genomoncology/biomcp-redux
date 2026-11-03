#!/bin/sh
set -eu
export PATH="/Users/ian/.local/share/uv/python/cpython-3.13.17-macos-aarch64-none/bin:$HOME/.cargo/bin:$PATH"
cargo +1.95.0 fmt --all
cargo +1.95.0 build --locked -p biomcp --bin biomcp
