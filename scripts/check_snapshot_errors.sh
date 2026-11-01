#!/bin/sh
set -eu
case "${1:-focused}" in
focused)
    cargo test --locked -p biomcp --test cases corrupted-snapshot-reports-payload-error
    cargo test --locked -p biomcp --test cases name-search-selects-paper-author
    ;;
cheap)
    make fmt
    cargo clippy --locked -p biomcp --test cases -- -D warnings
    make ratchets
    ;;
generate)
    make schema-generate
    git diff --stat -- crates/biomcp-types/schemas
    tar -czf snapshot-schemas.tar.gz crates/biomcp-types/schemas
    pwd
    ;;
routine)
    make landing-gate
    ;;
esac
