#!/bin/sh
# Run source-use checks with the installed binary and inherited remote tool paths.
set -eu
export PATH="/Users/ian/.local/share/uv/python/cpython-3.13.17-macos-aarch64-none/bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
phase=${1:-focused}
case "$phase" in
  format)
    cargo fmt --all
    UPDATE_SCHEMAS=1 cargo test --locked -p biomcp-types --features schema --test schemas checked_in_schemas_match_types -- --exact
    tar -czf "$PM_JOB_DIR/declared-use-generated.tar.gz" crates/biomcp-types/schemas
    ;;
  focused)
    cargo build --locked -p biomcp --bin biomcp
    cargo test --locked -p biomcp --test cases --no-run
    mkdir -p "$PM_JOB_DIR/installed/bin"
    cp "${CARGO_TARGET_DIR:-target}/debug/biomcp" "$PM_JOB_DIR/installed/bin/biomcp"
    export BIOMCP_TEST_EXECUTABLE="$PM_JOB_DIR/installed/bin/biomcp"
    cargo test --locked -p biomcp --test cases mcp-hosted/use/ -- --test-threads=4
    ;;
  gate) exec make landing-gate ;;
  *) exit 2 ;;
esac
