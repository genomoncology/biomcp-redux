#!/bin/sh
set -eu
export PATH="/Users/ian/.local/share/uv/python/cpython-3.13.17-macos-aarch64-none/bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-target/0594-proofs}"
phase=${1:-focused}
case "$phase" in
    focused)
        mkdir -p "$CARGO_TARGET_DIR"
        cargo fmt --all
        git diff > "$CARGO_TARGET_DIR/format.patch"
        cargo test --locked -p biomcp --test cases --no-run
        export BIOMCP_TEST_EXECUTABLE=/Users/ian/workspace/jobs/biomcp-redux-0587/target/0587-proofs/baseline-installed/biomcp
        if cargo test --locked -p biomcp --test cases gene-sections/bundle- -- --test-threads=1; then
            echo 'Pre-change installed binary unexpectedly passed bundle regressions' >&2
            exit 1
        fi
        cargo install --locked --debug --path crates/biomcp --root "$CARGO_TARGET_DIR/installed" --force
        export BIOMCP_TEST_EXECUTABLE="$PWD/$CARGO_TARGET_DIR/installed/bin/biomcp"
        cargo test --locked -p biomcp --test cases gene-sections/bundle- -- --test-threads=1
        ;;
    gate) make landing-gate ;;
    *) exit 2 ;;
esac
