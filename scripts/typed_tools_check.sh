#!/bin/sh
set -eu
cargo build --locked -p biomcp --bin biomcp
cargo test --locked -p biomcp --test cases mcp-typed -- --nocapture
