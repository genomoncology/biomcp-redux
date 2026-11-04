#!/bin/sh
# Check native cell-line filters through the installed CLI and MCP replay surfaces.
set -eu
cd "$(dirname "$0")/.."
case "${1:-focused}" in
  generate)
    cargo fmt --all
    UPDATE_SCHEMAS=1 cargo test --locked -p biomcp-types --features schema --test schemas checked_in_schemas_match_types -- --exact
    make docs-generate
    mkdir -p target/cell-line-filters
    git diff > target/cell-line-filters/generated.patch
    ;;
  focused)
    python3 scripts/trials.py --list
    python3 scripts/trials.py --installed cell-line/filters cell-line/examples cell-line/help
    cargo test --locked -p biomcp-types --features schema --test cell_line_contracts
    ;;
  *) echo 'usage: check_cell_line_filters.sh [generate|focused]' >&2; exit 2 ;;
esac
