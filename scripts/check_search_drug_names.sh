#!/bin/sh
# Focused outside-in cases and generated contracts for shared drug-name expansion.
set -eu
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  focused)
    cargo fmt --all -- --check
    make plain-cli-build
    python3 scripts/trials.py --installed --executable target/debug/biomcp --case-file drug-name-search.yaml
    ;;
  prepare)
    make schema-generate docs-generate
    mkdir -p dist
    tar -czf dist/drug-search-generated.tar.gz crates/biomcp-types/schemas docs/reference/commands.md
    ;;
  routine) exec sh scripts/check_nightly_tests.sh routine ;;
  *) exit 2 ;;
esac
