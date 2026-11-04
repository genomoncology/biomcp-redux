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
    UPDATE_SCHEMAS=1 cargo test --locked -p biomcp-types --features schema --test schemas checked_in_schemas_match_types -- --exact
    make docs-generate
    ;;
  focused)
    make docs-generate
    python3 scripts/trials.py --target cases --list
    python3 scripts/trials.py --target cases --installed gwas-search/gwas/examples.yaml gwas-get/gwas/get-examples.yaml gwas-search/gwas/help.yaml gwas-search/gwas/studies.yaml
    ;;
  installed) python3 scripts/check_gwas_studies_installed.py ;;
  scans)
    cargo fmt --all -- --check
    cargo clippy --locked -p biomcp-adapters -p biomcp-query -p biomcp-types --all-targets -- -D warnings
    ;;

  routine) exec sh scripts/check_nightly_tests.sh routine ;;
  *) echo 'usage: check_gwas_studies.sh [prepare|focused|installed|scans|routine]' >&2; exit 2 ;;
esac
