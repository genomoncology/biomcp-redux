#!/bin/sh
# Run through pm job. Docker executes both architectures at the declared floor.
set -eu
scratch=$(mktemp -d)
cleanup() {
  docker image rm biomcp-local-0058:amd64 biomcp-local-0058:arm64 >/dev/null 2>&1 || true
  rm -rf "$scratch"
}
trap cleanup EXIT HUP INT TERM
revision=$(git rev-parse HEAD)
version=$(cargo metadata --locked --no-deps --format-version 1 | python3 -c 'import json,sys; print(next(p["version"] for p in json.load(sys.stdin)["packages"] if p["name"] == "biomcp"))')
mkdir "$scratch/source"
git archive HEAD | tar -x -C "$scratch/source"
python3 release/artifacts.py build --output "$scratch/native"
python3 release/gate.py --artifacts "$scratch/native" --scratch "$scratch/native-check"
BIOMCP_CASE_RUNNER="$scratch/native-check/case-runner" python3 -m unittest release.test_artifacts
for arch in amd64 arm64; do
  case "$arch" in amd64) wheel_arch=x86_64;; arm64) wheel_arch=aarch64;; esac
  docker run --rm --label pm.repo=biomcp-redux --label pm.ticket=0058 \
    --platform "linux/$arch" -v "$scratch/source:/source:ro" -v "$scratch:/output" \
    "quay.io/pypa/manylinux_2_28_$wheel_arch" /bin/bash -c '
      set -euo pipefail
      cp -a /source /tmp/build
      cd /tmp/build
      export CARGO_HOME=/tmp/cargo RUSTUP_HOME=/tmp/rustup
      curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/rustup.sh
      sh /tmp/rustup.sh -y --profile minimal --default-toolchain 1.95.0
      export PATH=/tmp/cargo/bin:/opt/python/cp311-cp311/bin:$PATH
      python release/artifacts.py build --revision "'"$revision"'" --output /output/linux-'"$arch"'
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
  python3 release/container_check.py "biomcp-local-0058:$arch" "$arch" "$scratch/container-$arch" "$version"
  docker image rm "biomcp-local-0058:$arch"
done
