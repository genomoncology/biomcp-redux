#!/bin/sh
# Focused outside-in cases and generated contracts for shared drug-name expansion.
set -eu
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  focused)
    export BIOMCP_TRANSCRIPT_DIR="$PWD/target/drug-search-proof/transcripts"
    export BIOMCP_EVIDENCE_DIR="$PWD/target/drug-search-proof/evidence"
    mkdir -p target/drug-search-proof
    git rev-parse HEAD > target/drug-search-proof/source.txt
    cargo fmt --all -- --check
    make plain-cli-build
    python3 scripts/trials.py --installed --executable target/debug/biomcp --case-file search/drug-names/trial.yaml --case-file search/drug-names/article.yaml --case-file search/drug-names/ambiguous.yaml --case-file search/drug-names/policy.yaml --case-file search/drug-names/budget.yaml drug-name-search/legacy-literal-pool
    ;;
  focused-check)
    sh "$0" focused
    make check-cheap
    ;;
  prepare-focused)
    sh "$0" prepare
    sh "$0" focused
    ;;
  prepare)
    make schema-generate docs-generate
    mkdir -p dist
    COPYFILE_DISABLE=1 tar -czf dist/drug-search-generated.tar.gz crates/biomcp-types/schemas docs/reference/commands.md
    ;;
  routine) exec sh scripts/check_nightly_tests.sh routine ;;
  *) exit 2 ;;
esac
