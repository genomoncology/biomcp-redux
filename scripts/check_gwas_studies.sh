#!/bin/sh
# Check native GWAS publication study and bounded association search behavior.
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  prepare)
    cargo fmt --all
    cargo check --locked -p biomcp
    cargo test --locked -p biomcp --test cases -- --list
    make docs-generate
    UPDATE_SCHEMAS=1 cargo test --locked -p biomcp-types --features schema --test schemas checked_in_schemas_match_types -- --exact
    ;;
  focused)
    cargo test --locked -p biomcp --test cases -- gwas/
    ;;
  routine) exec sh scripts/check_nightly_tests.sh routine ;;
  *) echo 'usage: check_gwas_studies.sh [prepare|focused|routine]' >&2; exit 2 ;;
esac
