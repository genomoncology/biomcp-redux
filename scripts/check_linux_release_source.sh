#!/bin/sh
# Inventory source changes and check-only disk use in the pinned Linux image.
set -eu
cd "$(dirname "$0")/.."
case "${1:-container}" in
  container)
    image=$(python3.13 release/toolchain.py image manylinux-aarch64)
    mkdir -p target/linux-source-check
    docker run --rm ${JOB_LABEL_FILE:+--label-file "$JOB_LABEL_FILE"} \
      --platform linux/arm64 --memory 8g --cpus 2 \
      -v "$PWD:/source" -w /source \
      -e HOME=/source/target/linux-source-check/home \
      -e CARGO_BUILD_JOBS=2 -e RUST_TEST_THREADS=2 \
      "$image" /bin/bash scripts/check_linux_release_source.sh checks
    ;;
  checks)
    export PATH="$HOME/.cargo/bin:/opt/python/cp313-cp313/bin:$PATH"
    mkdir -p "$HOME"
    git config --global --add safe.directory /source
    python3.13 release/toolchain.py install-rust
    cargo --version
    python3.13 --version
    uname -sm
    cargo install cargo-deny --version "$(python3.13 release/toolchain.py get cargo-deny)" --locked
    snapshot() {
      echo "SOURCE STATUS $1"
      git status --porcelain=v1 --untracked-files=all
      echo "DISK $1"
      df -k . /tmp
      du -sk target
    }
    snapshot before
    test -z "$(git status --porcelain=v1 --untracked-files=all)"
    status=0
    for stage in fmt lint test docs-check; do
      if make "$stage"; then code=0; else code=$?; fi
      echo "CHECK RESULT $stage $code"
      snapshot "$stage"
      if [ "$code" != 0 ]; then status=$code; fi
    done
    echo 'CHECK BUILD OUTPUT'
    du -sk target/* target/debug/* 2>/dev/null || true
    exit "$status"
    ;;
  *) exit 2 ;;
esac
