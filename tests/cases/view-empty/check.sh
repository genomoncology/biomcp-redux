#!/bin/sh
# Run installed Markdown cases, retaining their complete sanitized replies.
set -eu
python="$(uv python find --offline 3.13.17)"
export PATH="$(dirname "$python"):$HOME/.cargo/bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
export BIOMCP_TRANSCRIPT_DIR="$PWD/target/view-empty/transcripts"
export BIOMCP_EVIDENCE_DIR="$PWD/target/view-empty/fixtures"
mkdir -p "$PWD/target/view-empty"
cargo fmt --all -- --check
python3 scripts/trials.py --installed view-empty/empty-section view-empty/failed-section view-empty/all-absent
