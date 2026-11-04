#!/bin/sh
# Test only the runner's interrupted-read repair and the affected installed case.
set -eu
cd "$(dirname "$0")/.."
case "${1:-focused}" in
  routine)
    exec python3.13 -c 'import json,os; os.execlp("sh", "sh", "-c", json.load(open("sdlc/pm.json"))["ticketsGate"])' ;;
  focused)
    export RUSTUP_TOOLCHAIN="$(python3.13 release/toolchain.py get rust)"
    mkdir -p target/http-read
    rustc --edition 2024 --test crates/biomcp/tests/runner/socket_read.rs -o target/http-read/read-tests
    target/http-read/read-tests
    cargo fmt --all -- --check
    cargo clippy --locked -p biomcp --test cases --test tools -- -D warnings
    ;;
  container)
    [ "$#" -eq 4 ] || { echo 'container ARTIFACTS CACHE REVISION' >&2; exit 2; }
    artifacts=$2
    cache=$3
    revision=$4
    python3.13 -c 'from pathlib import Path; import sys; from release.artifacts import candidate_record; print(candidate_record(Path(sys.argv[1]), sys.argv[2]))' "$artifacts" "$revision"
    image=$(python3.13 release/toolchain.py image manylinux-x86_64)
    mkdir -p target/http-read-linux/build
    if [ "$cache" != - ]; then cp -a "$cache/." target/http-read-linux/build/; fi
    docker run --rm ${JOB_LABEL_FILE:+--label-file "$JOB_LABEL_FILE"} \
      --platform linux/amd64 --memory 8g --cpus 2 \
      -v "$PWD:/source" -v "$artifacts:/artifacts:ro" -w /source \
      -e HOME=/source/target/http-read-linux/home \
      -e CARGO_TARGET_DIR=/source/target/http-read-linux/build \
      -e CARGO_BUILD_JOBS=2 -e RUST_TEST_THREADS=2 \
      -e "BIOMCP_BUILD_REVISION=$revision" \
      "$image" /bin/bash scripts/linux_fixture_cache.sh /source/target/http-read-linux/evidence \
      /bin/bash scripts/check_http_read.sh inside
    ;;
  inside)
    python_series=$(python3 release/toolchain.py get python | tr -d .)
    export PATH="$HOME/.cargo/bin:/opt/python/cp$python_series-cp$python_series/bin:$PATH"
    mkdir -p "$HOME"
    python release/toolchain.py install-rust
    rustc --edition 2024 --test crates/biomcp/tests/runner/socket_read.rs -o "$CARGO_TARGET_DIR/read-tests"
    "$CARGO_TARGET_DIR/read-tests"
    cargo test --locked -p biomcp --test cases --no-run --message-format=json > target/http-read-linux/runner.jsonl
    mkdir -p target/http-read-linux/installed
    tar -xzf /artifacts/biomcp-linux-x86_64.tar.gz -C target/http-read-linux/installed
    python - <<'PYTHON'
import json, os, subprocess
from pathlib import Path
rows = [json.loads(line) for line in Path('target/http-read-linux/runner.jsonl').read_text().splitlines()]
runner, = [row['executable'] for row in rows if row.get('reason') == 'compiler-artifact' and row.get('target', {}).get('name') == 'cases' and row.get('executable')]
env = {**os.environ, 'BIOMCP_TEST_EXECUTABLE': str(Path('target/http-read-linux/installed/biomcp').resolve()),
       'BIOMCP_TEST_FIXTURES': str(Path('tests').resolve())}
subprocess.run([runner, 'mcp-http/framing-resource-deadlines-and-shutdown', '--exact', '--test-threads', '1'], env=env, check=True)
PYTHON
    ;;
  *) echo 'usage: check_http_read.sh [focused|routine|container ARTIFACTS CACHE REVISION]' >&2; exit 2 ;;
esac
