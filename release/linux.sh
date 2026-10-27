#!/bin/sh
# Build once at the wheel floor and qualify the same native and container bytes.
set -eu
test -n "$BUILD_IMAGE"
revision=$(git rev-parse HEAD)
mkdir -p .release-cache/cargo dist
docker run --rm "$@" --user "$(id -u):$(id -g)" -e HOME=/tmp \
  -v "$PWD:/source" -w /source \
  -v "$PWD/.release-cache/cargo:/cargo" -e CARGO_HOME=/cargo \
  -e CARGO_BUILD_JOBS=2 -e "BIOMCP_BUILD_REVISION=$revision" \
  "$BUILD_IMAGE" /bin/bash -c '
    set -euo pipefail
    export PATH=/cargo/bin:/opt/python/cp312-cp312/bin:$PATH
    curl --proto "=https" --tlsv1.2 -sSf https://static.rust-lang.org/rustup/archive/1.28.2/$(uname -m)-unknown-linux-gnu/rustup-init -o /tmp/rustup-init
    curl --proto "=https" --tlsv1.2 -sSf https://static.rust-lang.org/rustup/archive/1.28.2/$(uname -m)-unknown-linux-gnu/rustup-init.sha256 -o /tmp/rustup-init.sha256
    cd /tmp
    sha256sum -c rustup-init.sha256
    chmod +x rustup-init
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain 1.95.0
    cd /source
    python release/artifacts.py build --manylinux --revision "$BIOMCP_BUILD_REVISION" --output dist
    python release/gate.py --prepare-only --artifacts dist --scratch /source/proof
  '
# The install process sees the artifacts and copied proof, without the checkout.
docker run --rm "$@" --user "$(id -u):$(id -g)" -e HOME=/tmp \
  -v "$PWD/dist:/artifacts:ro" -v "$PWD/proof:/proof:ro" \
  -v "$PWD/dist:/output" \
  "$BUILD_IMAGE" /bin/bash -c '
    set -euo pipefail
    export PATH=/opt/python/cp312-cp312/bin:$PATH
    cp -a /proof /tmp/check
    python /tmp/check/tools/release/gate.py --prepared --python python \
      --artifacts /artifacts --scratch /tmp/check --report /output/install-report.json
  ' > install-report.log
cat install-report.log
archive=$(find dist -name 'biomcp-linux-*.tar.gz')
case "$archive" in
  *x86_64*) arch=amd64;;
  *arm64*) arch=arm64;;
  *) echo 'unsupported Linux archive' >&2; exit 1;;
esac
mkdir container
tar -xzf "$archive" -C container
cp proof/case-runner container/case-runner
cp -R proof/fixtures container/fixtures
cp release/Dockerfile container/Dockerfile
image="biomcp-candidate:$arch"
docker build --platform "linux/$arch" -t "$image" container
version=$(python3 -c 'import json; print(json.load(open("dist/release.json"))["version"])')
python3 release/container_check.py "$image" "$arch" container "$version" "$@"
docker save "$image" -o "dist/biomcp-container-$arch.tar"
docker image rm "$image"
