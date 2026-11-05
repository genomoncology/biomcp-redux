#!/bin/sh
# Synthetic recorder preflight only. No compiler, product, client or provider.
set -eu
python="$(uv python find --offline 3.13.17)"
export PATH="$(dirname "$python"):$HOME/.cargo/bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
export PYTHONDONTWRITEBYTECODE=1
exec "$python" scripts/typed_harness_preflight.py check
