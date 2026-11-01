#!/bin/sh
# Exercise the release notice graph and schema proof without a release build.
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
export PATH="$HOME/.local/bin:$PATH"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/target}"
case "${1:-focused}" in
  diagnose)
    cargo --version
    python3.13 --version
    for crate in borrow-or-share@0.2.4 foldhash@0.2.0; do
      cargo tree --locked --workspace --target all -i "$crate"
      cargo tree --locked --workspace --target all --edges normal,build -i "$crate"
    done
    cargo deny --version
    cargo deny --locked check licenses
    ;;
  fixture)
    python3.13 -m unittest release.test_licenses.Licensing.test_generation_excludes_dependencies_that_do_not_ship
    ;;
  focused)
    python3.13 -m unittest release.test_licenses release.test_publication
    python3.13 scripts/test_schema_release.py
    ;;
  *) exit 2 ;;
esac
