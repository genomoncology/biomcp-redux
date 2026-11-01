#!/bin/sh
# Bound the cache-process diagnosis to twelve cases sharing two CPUs.
set -eu
cd "$(dirname "$0")/.."
case "${1:-container}" in
container)
    image=$(python3.13 release/toolchain.py image manylinux-aarch64)
    mkdir -p target/cache-process-load
    docker run --rm ${JOB_LABEL_FILE:+--label-file "$JOB_LABEL_FILE"} \
        --platform linux/arm64 --memory 8g --cpus 2 \
        -v "$PWD:/source" -w /source -e HOME=/source/target/cache-process-load/home \
        -e CARGO_TARGET_DIR=/source/target/cache-process-load/build \
        -e CARGO_BUILD_JOBS=2 -e RUST_TEST_THREADS=2 \
        "$image" /bin/bash scripts/check_cache_process_load.sh checks
    ;;
checks)
    export PATH="$HOME/.cargo/bin:/opt/python/cp313-cp313/bin:$PATH"
    mkdir -p "$HOME"
    python3.13 release/toolchain.py install-rust
    cargo --version
    python3.13 --version
    uname -sm
    cargo fmt --all -- --check
    cargo test --locked -p biomcp-fetch --test cache --no-run --message-format=json > target/cache-process-load/build.json
    python3.13 scripts/cache_process_load.py target/cache-process-load/build.json
    ;;
*) exit 2 ;;
esac
