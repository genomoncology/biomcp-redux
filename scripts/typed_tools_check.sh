#!/bin/sh
set -eu
typed_scratch=$(mktemp -d)
trap 'rm -rf "$typed_scratch"' EXIT HUP INT TERM
if [ "$#" -gt 0 ]; then
    cp "$1" "$typed_scratch/baseline"
    test "$(shasum -a 256 "$typed_scratch/baseline" | cut -d ' ' -f 1)" = "$2"
    echo "TYPED_BASELINE_RECEIVED sha256=$2"
    mkdir -p target/typed-tools-baseline
    cp -f "$typed_scratch/baseline" target/typed-tools-baseline/biomcp
    chmod a-w "$typed_scratch/baseline" target/typed-tools-baseline/biomcp
    export BIOMCP_TYPED_BASELINE="$typed_scratch/baseline"
fi
: "${BIOMCP_TYPED_BASELINE:?a verified frozen baseline executable is required}"
test -x "$BIOMCP_TYPED_BASELINE"
cargo build --locked -p biomcp --bin biomcp --message-format=json > "$typed_scratch/product.json" || {
    python3 -c 'import json,sys; [print(v["message"].get("rendered","")) for s in open(sys.argv[1]) if (v:=json.loads(s)).get("reason")=="compiler-message"]' "$typed_scratch/product.json"
    exit 1
}
cargo test --locked -p biomcp --test cases --no-run --message-format=json > "$typed_scratch/runner.json" || {
    python3 -c 'import json,sys; [print(v["message"].get("rendered","")) for s in open(sys.argv[1]) if (v:=json.loads(s)).get("reason")=="compiler-message"]' "$typed_scratch/runner.json"
    exit 1
}
mkdir -p target/typed-tools-transcript
export BIOMCP_TRANSCRIPT_DIR="$(pwd)/target/typed-tools-transcript"
echo "TYPED_EVIDENCE_DIR=$BIOMCP_TRANSCRIPT_DIR"
typed_product=$(python3 -c 'import json,sys; print(next(v["executable"] for s in open(sys.argv[1]) if (v:=json.loads(s)).get("executable") and v["target"]["name"]=="biomcp"))' "$typed_scratch/product.json")
typed_runner=$(python3 -c 'import json,sys; print(next(v["executable"] for s in open(sys.argv[1]) if (v:=json.loads(s)).get("executable") and v["target"]["name"]=="cases"))' "$typed_scratch/runner.json")
if [ "${3:-}" = guards ]; then
    if env -u BIOMCP_TYPED_BASELINE python3 scripts/trials.py --runner "$typed_runner" --executable "$typed_product" --installed mcp-parity/typed-baseline-and-replay-guards > "$typed_scratch/missing-baseline.log" 2>&1; then
        echo 'missing baseline unexpectedly passed' >&2
        exit 1
    fi
    cat "$typed_scratch/missing-baseline.log"
    rg 'BIOMCP_TYPED_BASELINE is required' "$typed_scratch/missing-baseline.log"
    python3 scripts/trials.py --runner "$typed_runner" --executable "$typed_product" --installed mcp-parity/typed-baseline-and-replay-guards
else
    python3 scripts/trials.py --runner "$typed_runner" --executable "$typed_product" --installed mcp-parity/mcp-typed-tools
fi

if [ "${3:-}" = extended ]; then
    make docs-generate
    make mcp-reply-scan JSCPD='npm exec --offline --yes --package=jscpd@4.0.5 -- jscpd'
    npm exec --offline --yes --package=jscpd@4.0.5 -- jscpd crates/biomcp/tests/runner/typed_*.rs --format rust --max-size 10mb --reporters json --output target/typed-tools-jscpd-tests
fi
