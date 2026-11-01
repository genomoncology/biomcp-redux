#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
export PATH="$HOME/.local/bin:$PATH"
python3.13 --version
case "${1:-linux}" in
  linux)
    image=rust:1.95.0-slim
    scratch=$(mktemp -d)
    trap 'chmod -R u+w "$scratch"; rm -rf "$scratch"' EXIT HUP INT TERM
    docker run --rm ${JOB_LABEL_FILE:+--label-file "$JOB_LABEL_FILE"} \
      --user "$(id -u):$(id -g)" -e CARGO_MANIFEST_DIR=/source/crates/biomcp-test-support \
      -v "$PWD:/input:ro" -v "$scratch:/output" "$image" \
      rustc --edition 2024 /input/release/evidence_probe.rs -o /output/evidence-probe
    mkdir "$scratch/fixtures"
    chmod 555 "$scratch/fixtures"
    docker run --rm ${JOB_LABEL_FILE:+--label-file "$JOB_LABEL_FILE"} \
      --user "$(id -u):$(id -g)" \
      -v "$scratch:/proof:ro" "$image" /bin/sh -eu -c '
        cp -a /proof /tmp/check
        test ! -e /source
        test ! -w /tmp/check/fixtures
        mkdir /tmp/fixture-scratch
        export BIOMCP_TEST_FIXTURES=/tmp/check/fixtures TMPDIR=/tmp/fixture-scratch
        /tmp/check/evidence-probe /tmp/check/target/fixture-evidence
      '
    ;;
  focused)
    cargo fmt --all -- --check
    python3.13 scripts/check_shell.py
    cargo test --locked -p biomcp --test tools schema-discovery/retains-failed-local-peer-state -- --exact
    cargo test --locked -p biomcp --test tools schema-discovery/persistent-fixture-evidence -- --exact
    cargo test --locked -p biomcp --test tools schema-discovery/persistent-native-evidence -- --exact
    ;;
  *) exit 2 ;;
esac
