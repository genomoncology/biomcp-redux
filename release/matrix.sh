#!/bin/sh
# Run through pm job. Docker executes both architectures at the declared floor.
set -eu
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT HUP INT TERM
python3 release/artifacts.py build --output "$scratch/native"
python3 release/gate.py --artifacts "$scratch/native" --scratch "$scratch/native-check"
for arch in amd64 arm64; do
  case "$arch" in amd64) wheel_arch=x86_64;; arm64) wheel_arch=aarch64;; esac
  docker run --rm --label pm.repo=biomcp-redux --label pm.ticket=0058 \
    --platform "linux/$arch" -v "$PWD:/source:ro" -v "$scratch:/output" \
    "quay.io/pypa/manylinux_2_28_$wheel_arch" /bin/bash -c '
      set -euo pipefail
      cp -a /source /tmp/build
      cd /tmp/build
      export CARGO_HOME=/tmp/cargo RUSTUP_HOME=/tmp/rustup
      curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/rustup.sh
      sh /tmp/rustup.sh -y --profile minimal --default-toolchain 1.95.0
      export PATH=/tmp/cargo/bin:/opt/python/cp311-cp311/bin:$PATH
      python release/artifacts.py build --output /output/linux-'"$arch"'
      python release/gate.py --python /opt/python/cp311-cp311/bin/python --artifacts /output/linux-'"$arch"' --scratch /tmp/check
      cp /tmp/check/report.json /output/linux-'"$arch"'/report.json
      mkdir -p /output/container-'"$arch"'
      cp /tmp/check/installer-bin/biomcp /output/container-'"$arch"'/biomcp
      cp /tmp/check/case-runner /output/container-'"$arch"'/case-runner
      cp -a /tmp/check/fixtures /output/container-'"$arch"'/fixtures
    '
  # Build and execute the actual runtime image for this architecture.
  cp release/Dockerfile "$scratch/container-$arch/Dockerfile"
  docker build --platform "linux/$arch" -t "biomcp-local-0058:$arch" "$scratch/container-$arch"
  docker run --rm --label pm.repo=biomcp-redux --label pm.ticket=0058 \
    --platform "linux/$arch" "biomcp-local-0058:$arch" schema biodata.problem/1
  docker image rm "biomcp-local-0058:$arch"
done
