#!/bin/sh
# Test only the runner's interrupted-read repair and the affected installed case.
set -eu
cd "$(dirname "$0")/.."
case "${1:-focused}" in
  routine)
    exec python3.13 -c 'import json,os; os.execlp("sh", "sh", "-c", json.load(open("sdlc/pm.json"))["ticketsGate"])' ;;
  focused)
    export RUSTUP_TOOLCHAIN="$(python3.13 release/toolchain.py get rust)"
    mkdir -p target/http-read
    rustc --edition 2024 --test crates/biomcp/tests/runner/socket_read.rs -o target/http-read/read-tests
    target/http-read/read-tests
    cargo fmt --all -- --check
    cargo clippy --locked -p biomcp --test cases --test tools -- -D warnings
    ;;
  *) echo 'usage: check_http_read.sh [focused|routine]' >&2; exit 2 ;;
esac
