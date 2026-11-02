#!/bin/sh
# Bound the cache-process diagnosis to twelve cases sharing two CPUs.
set -eu
cd "$(dirname "$0")/.."
case "${1:-container}" in
container|native)
    image=$(python3.13 release/toolchain.py image manylinux-aarch64)
    mkdir -p target/cache-process-load
    mode=checks
    if [ "${1:-container}" = native ]; then mode=native-checks; fi
    docker run --rm ${JOB_LABEL_FILE:+--label-file "$JOB_LABEL_FILE"} \
        --platform linux/arm64 --memory 8g --cpus 2 \
        -v "$PWD:/source" -w /source -e HOME=/source/target/cache-process-load/home \
        -e CARGO_TARGET_DIR=/source/target/cache-process-load/build \
        -e CARGO_BUILD_JOBS=2 -e RUST_TEST_THREADS=2 \
        "$image" /bin/bash scripts/check_cache_process_load.sh "$mode"
    ;;
native-checks)
    export PATH="$HOME/.cargo/bin:/opt/python/cp313-cp313/bin:$PATH"
    export BIOMCP_EVIDENCE_DIR=/tmp/cache-process-native
    sha256sum target/cache-process-load/build/debug/deps/cache-* | head -10
    status=0
    python3.13 scripts/cache_process_load.py target/cache-process-load/build.json || status=$?
    cp -a /tmp/cache-process-native target/cache-process-load/native-fixtures
    python3.13 -c 'import pathlib,sqlite3; [(print(str(p),sqlite3.connect(p).execute("SELECT count(*) FROM reply").fetchone()[0])) for p in pathlib.Path("target/cache-process-load/native-fixtures").rglob("shared.sqlite")]'
    exit "$status"
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
