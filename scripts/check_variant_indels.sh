#!/bin/sh
# Installed coordinate-form matrix; accepts a source baseline for scoped clone scans.
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  focused)
    cargo fmt --all -- --check
    cargo test --locked -p biomcp-types --test variant_values coordinate_inputs
    cargo install --locked --debug --path crates/biomcp --root target/variant-indels-installed --force
    export BIOMCP_TEST_EXECUTABLE="$PWD/target/variant-indels-installed/bin/biomcp"
    make docs-generate
    git diff --binary > target/variant-indels-generated.patch
    cargo test --locked -p biomcp --test cases variant/id-forms/ -- --test-threads 2
    ;;
  scans)
    baseline=${2:?scans requires the reviewed baseline commit}
    scanner=$(python3 - <<'PY'
import json
from pathlib import Path
for package in (Path.home() / '.npm/_npx').glob('*/node_modules/jscpd/package.json'):
    if json.loads(package.read_text())['version'] == '4.0.5':
        print(package.parent.parent / '.bin/jscpd')
        break
else:
    raise SystemExit('cached jscpd 4.0.5 is required')
PY
    )
    "$scanner" --version
    for side in baseline current; do
      folder="target/variant-indel-scans/$side"
      mkdir -p "$folder/source"
      for file in crates/biomcp-types/src/query/variant_input.rs crates/biomcp-types/src/query/variant.rs crates/biomcp-types/tests/variant_values.rs crates/biomcp-query/src/variant/mod.rs crates/biomcp-adapters/src/allele_registry/map.rs crates/biomcp-adapters/src/allele_registry/raw.rs; do
        output="$folder/source/$(echo "$file" | tr / _)"
        if [ "$side" = baseline ]; then git show "$baseline:$file" > "$output"; else cp "$file" "$output"; fi
      done
      "$scanner" "$folder/source" --format rust --max-size 10mb --reporters console,json --output "$folder/report"
    done
    ;;
  refresh)
    make docs-generate
    git diff --binary > target/variant-indels-generated.patch
    ;;
  *) echo 'usage: check_variant_indels.sh [focused|scans BASE_COMMIT|refresh]' >&2; exit 2 ;;
esac
