#!/bin/sh
# Build the released public source offline, then invoke only its help tree.
set -eu
public_repo=${1:?public repository path required}
source_revision=5641de7d7b79b53872f8599dbb4dba22e611dcaf
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT HUP INT TERM
mkdir "$scratch/source"
git -C "$public_repo" archive "$source_revision" | tar -x -C "$scratch/source"
(
    cd "$scratch/source"
    CARGO_TARGET_DIR="$scratch/target" cargo build --offline --locked --bin biomcp >&2
)
python3 scripts/parity_inventory.py "$scratch/target/debug/biomcp"
