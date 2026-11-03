#!/bin/sh
# Exercise the installed article view with the shared pre-change binary.
set -eu
export PATH="/Users/ian/.local/share/uv/python/cpython-3.13.17-macos-aarch64-none/bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
phase=${1:-focused}
output="/Users/ian/workspace/t/biomcp-redux-0590-${PM_JOB:?}"
mkdir -p "$output/installed/bin"
if [ "$phase" = gate ]; then
    make landing-gate
fi
cargo fmt --all
git diff > "$output/format.patch"
cargo build --locked -p biomcp --bin biomcp
cp "${CARGO_TARGET_DIR:-target}/debug/biomcp" "$output/installed/bin/biomcp"
export BIOMCP_TEST_EXECUTABLE="$output/installed/bin/biomcp"
export BIOMCP_LEGACY_EXECUTABLE="/Users/ian/workspace/t/biomcp-redux-0586-benchmark-baseline-27ead/installed/bin/biomcp"
export BIOMCP_TRANSCRIPT_DIR="$output/replies"
cargo test --locked -p biomcp --test cases article-fulltext/compact-outline -- --nocapture
cargo test --locked -p biomcp --test cases article/fulltext/ -- --test-threads=4
cargo test --locked -p biomcp --test cases article/preprint/ -- --test-threads=4
