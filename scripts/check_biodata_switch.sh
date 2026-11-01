#!/bin/sh
# Check the dependency switch in a physical checkout with job-owned output.
set -eu
phase=${1:-check}
root=${2:-$(mktemp -d /private/tmp/biomcp-0459.XXXXXX)}
case "$root" in /private/tmp/biomcp-0459.*) ;; *) echo 'unexpected qualification root' >&2; exit 1;; esac
mkdir -p "$root/source"
git archive HEAD | /usr/bin/tar -xf - -C "$root/source"
cd "$root/source"
export CARGO_TARGET_DIR="$root/target"
printf 'Qualification root: %s\n' "$root"
rustc --version
case "$phase" in
cheap)
    cargo fmt --all -- --check
    make lint ratchets
    ;;
check)
    cargo check --workspace --all-targets --locked
    ;;
*) echo 'unknown switch check phase' >&2; exit 1;;
esac
