#!/bin/sh
set -eu
case "${1:-focused}" in
cheap)
    make fmt lint ratchets
    ;;
generate)
    make schema-generate
    tar -czf cache-error-schemas.tar.gz crates/biomcp-types/schemas
    ;;
focused)
    cargo test --locked -p biomcp --test cases cache-resilience/
    cargo test --locked -p biomcp-query --test article_search
    cargo test --locked -p biomcp-fetch --test cache
    cargo test --locked -p biomcp-fetch --lib cache::contention
    unset UPDATE_SCHEMAS
    cargo test --locked -p biomcp-types --features schema --test schemas checked_in_schemas_match_types -- --exact
    ;;
cases)
    cargo test --locked -p biomcp --test cases
    unset UPDATE_SCHEMAS
    cargo test --locked -p biomcp-types --features schema --test schemas checked_in_schemas_match_types -- --exact
    ;;
*)
    echo "expected cheap, generate, focused or cases" >&2
    exit 2
    ;;
esac
