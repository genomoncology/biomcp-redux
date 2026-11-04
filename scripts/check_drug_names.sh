#!/bin/sh
# Focused installed drug name proof; routine reuses the configured gate launcher.
set -eu
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  routine) exec sh scripts/check_nightly_tests.sh routine ;;
  prepare)
    cargo fmt --all
    UPDATE_SCHEMAS=1 cargo test --locked -p biomcp-types --features schema --test schemas checked_in_schemas_match_types -- --exact
    git diff --binary > target/drug-names-prepare.patch
    cargo check --locked -p biomcp
    ;;
  focused)
    cargo install --locked --debug --path crates/biomcp --root target/drug-names-installed --force
    export BIOMCP_TEST_EXECUTABLE="$PWD/target/drug-names-installed/bin/biomcp"
    export BIOMCP_TRANSCRIPT_DIR="$PWD/target/drug-names-proof/transcripts"
    export BIOMCP_EVIDENCE_DIR="$PWD/target/drug-names-proof/evidence"
    mkdir -p target/drug-names-proof
    git rev-parse HEAD > target/drug-names-proof/source.txt
    cargo test --locked -p biomcp --test cases drug/ -- --nocapture
    cargo test --locked -p biomcp-types --test drug_contracts
    ;;
  *) exit 2 ;;
esac
