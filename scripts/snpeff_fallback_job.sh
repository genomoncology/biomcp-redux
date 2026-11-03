#!/bin/sh
set -eu
export PATH="/Users/ian/.local/share/uv/python/cpython-3.13.17-macos-aarch64-none/bin:$HOME/.cargo/bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
mkdir -p target/0595-proofs
if [ "${1:-}" = format ]; then
    cargo fmt --all
    git diff --binary > target/0595-proofs/format.patch
    cargo check --locked -p biomcp-query
    exit
fi
if [ "${1:-}" = schemas ]; then
    UPDATE_SCHEMAS=1 cargo test --locked -p biomcp-types --features schema --test schemas checked_in_schemas_match_types -- --exact
    git diff --binary > target/0595-proofs/schemas.patch
    exit
fi
if [ "${1:-}" = gate ]; then
    make landing-gate
    exit
fi
cargo fmt --all -- --check
cargo install --locked --debug --path crates/biomcp --root target/0595-proofs/installed --force
export BIOMCP_TEST_EXECUTABLE="$PWD/target/0595-proofs/installed/bin/biomcp"
export BIOMCP_TRANSCRIPT_DIR="$PWD/target/0595-proofs/replies"
cargo test --locked -p biomcp --test cases variant-search/ -- --nocapture
