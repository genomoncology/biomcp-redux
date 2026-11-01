#!/bin/sh
set -eu
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-static}" in
  static)
    make fmt
    cargo clippy --workspace --all-targets --locked -- -D warnings
    cargo test --locked -p biomcp --test ratchets
    ;;
  focused)
    cargo test --locked -p biomcp-types --features schema --test queries --test diagnostic_contracts
    cargo test --locked -p biomcp-types --features schema --test schemas checked_in_schemas_match_types
    cargo test --locked -p biomcp --test cases disease/
    cargo test --locked -p biomcp --test cases diagnostic/
    cargo test --locked -p biomcp --test cases drug-recall/
    cargo test --locked -p biomcp --test cases variant/
    cargo test --locked -p biomcp --test cases phenotype/
    ;;
  *) exit 2 ;;
esac
