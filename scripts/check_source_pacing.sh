#!/bin/sh
set -eu
case "${1:-focused}" in
red) cargo test --locked -p biomcp --test cases source-pacing/settings-and-ceilings ;;
cheap) make fmt lint ratchets ;;
focused)
    cargo test --locked -p biomcp-fetch --test quotas
    cargo test --locked -p biomcp-adapters --test source_pacing
    cargo test --locked -p biomcp --test cases source-pacing/
    make docs-generate
    cargo test --locked -p biomcp --bin biomcp grammar::reference::reference_matches_grammar
    ;;
*) exit 2 ;;
esac
