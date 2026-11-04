#!/bin/sh
# Exercise installed TLS diagnostics against local fixtures; never contact a provider.
set -eu
export PATH="/Users/ian/.local/share/uv/python/cpython-3.13.17-macos-aarch64-none/bin:$HOME/.cargo/bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
proofs="$PWD/target/0582-proofs"
mkdir -p "$proofs"
git check-ignore "$proofs/probe"
case "${1:-focused}" in
    red)
        export BIOMCP_TRANSCRIPT_DIR="$proofs/red/transcripts"
        export BIOMCP_EVIDENCE_DIR="$proofs/red/fixtures"
        python3 scripts/trials.py --installed --executable "$2" corporate-network/additive-ca-tls-and-proxy-isolation
        ;;
    focused)
        export BIOMCP_TRANSCRIPT_DIR="$proofs/green/transcripts"
        export BIOMCP_EVIDENCE_DIR="$proofs/green/fixtures"
        python3 scripts/trials.py --installed corporate-network/additive-ca-tls-and-proxy-isolation
        ;;
    scan)
        scanner=/Users/ian/.npm/_npx/f96b6f8e572bd88e/node_modules/.bin/jscpd
        "$scanner" --version
        baseline="$proofs/baseline-source"
        mkdir -p "$baseline"
        git archive 974d11223 crates/biomcp-fetch/src crates/biomcp-query/src/status.rs \
            crates/biomcp/src/command/sync.rs crates/biomcp/tests/runner/corporate.rs \
            crates/biomcp/tests/runner/launch.rs crates/biomcp-test-support/src/network_stub.rs | tar -xf - -C "$baseline"
        for kind in baseline-product product baseline-tests tests; do
            case "$kind" in baseline-*) prefix="$baseline/" ;; *) prefix= ;; esac
            case "$kind" in
                *product) set -- "${prefix}crates/biomcp-fetch/src" "${prefix}crates/biomcp-query/src/status.rs" "${prefix}crates/biomcp/src/command/sync.rs" ;;
                *tests) set -- "${prefix}crates/biomcp/tests/runner/corporate.rs" "${prefix}crates/biomcp/tests/runner/launch.rs" "${prefix}crates/biomcp-test-support/src/network_stub.rs" ;;
            esac
            "$scanner" "$@" --format rust --max-size 10mb --reporters json --output "$proofs/jscpd-$kind"
        done
        ;;
    gate)
        python3 --version
        cargo machete --version
        make landing-gate
        ;;
    *) exit 2 ;;
esac
