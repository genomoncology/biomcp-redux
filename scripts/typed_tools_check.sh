#!/bin/sh
set -eu
cargo build --locked -p biomcp --bin biomcp
python3 scripts/trials.py --installed mcp-parity/mcp-typed-tools
