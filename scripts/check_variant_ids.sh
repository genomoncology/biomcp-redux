#!/bin/sh
set -eu
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  focused)
    cargo check --locked -p biomcp-query -p biomcp-types
    cargo test --locked -p biomcp --test cases variant/id-forms
    ;;
  gate) make landing-gate ;;
  *) exit 2 ;;
esac
