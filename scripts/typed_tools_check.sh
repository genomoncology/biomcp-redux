#!/bin/sh
set -eu
if [ "$#" -gt 0 ]; then
    typed_scratch=$(mktemp -d)
    trap 'rm -rf "$typed_scratch"' EXIT HUP INT TERM
    cp "$1" "$typed_scratch/baseline"
    test "$(shasum -a 256 "$typed_scratch/baseline" | cut -d ' ' -f 1)" = "$2"
    export BIOMCP_TYPED_BASELINE="$typed_scratch/baseline"
fi
cargo build --locked -p biomcp --bin biomcp
python3 scripts/trials.py --installed mcp-parity/mcp-typed-tools
