#!/bin/sh
# Check the dependency switch in a physical checkout with job-owned output.
set -eu
phase=${1:-check}
root=${2:-$(mktemp -d /private/tmp/biomcp-0459.XXXXXX)}
case "$root" in /private/tmp/biomcp-0459.*) ;; *) echo 'unexpected qualification root' >&2; exit 1;; esac
source="$root/source-$(git rev-parse HEAD)"
mkdir -p "$source"
git archive HEAD | /usr/bin/tar -xf - -C "$source"
cd "$source"
export CARGO_TARGET_DIR="$root/target"
printf 'Qualification root: %s\n' "$root"
rustc --version
case "$phase" in
cheap)
    cargo fmt --all -- --check
    make lint ratchets
    ;;
vendor)
    python3 release/vendor.py -- cargo metadata --frozen --format-version 1 > "$root/offline-metadata.json"
    ;;
check)
    cargo check --workspace --all-targets --locked
    ;;
*) echo 'unknown switch check phase' >&2; exit 1;;
esac
