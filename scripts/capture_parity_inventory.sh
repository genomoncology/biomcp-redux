#!/bin/sh
# Build released public source offline and retain its executable and raw help.
set -eu
public_repo=${1:?public repository path required}
source_revision=5641de7d7b79b53872f8599dbb4dba22e611dcaf
staging=$(mktemp -d "${TMPDIR:-/tmp}/biomcp-parity-0276.XXXXXX")
printf 'Retained inventory staging (ticket 0276 owns cleanup): %s\n' "$staging" >&2
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT HUP INT TERM
mkdir "$scratch/source"
git -C "$public_repo" archive "$source_revision" | tar -x -C "$scratch/source"
(
    cd "$scratch/source"
    CARGO_TARGET_DIR="$scratch/target" cargo build --jobs 2 --offline --locked --bin biomcp >&2
)
mv "$scratch/target/debug/biomcp" "$staging/biomcp"
printf '%s\n' "$source_revision" > "$staging/source-revision"
python3 scripts/parity_inventory.py "$staging/biomcp" --capture-dir "$staging/help"
