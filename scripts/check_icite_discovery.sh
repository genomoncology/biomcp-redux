#!/bin/sh
# Run source-owned citation discovery checks before the routine gate.
set -eu
cd "$(dirname "$0")/.."
export RUSTUP_TOOLCHAIN=1.95.0
case "${1:-focused}" in
  focused)
    make fmt lint ratchets
    cargo test --locked -p biomcp --test tools -- --proofs live-source-checks/retained-drift-witnesses
    cargo test --locked -p biomcp --test cases -- article/citations/
    ;;
  routine) make landing-gate ;;
  fixtures)
    python3.13 -m unittest release.test_gate
    ;;
  installed)
    python3 - <<'PYTHON'
from pathlib import Path
import os
import shutil
import subprocess
import sys
import tempfile

sys.path.insert(0, str(Path('release').resolve()))
from artifacts import cargo
from gate import stage_fixtures

binary = Path(cargo('dev')['executable'])
runner = Path(cargo('dev', tests='tools')['executable'])
with tempfile.TemporaryDirectory(prefix='installed-icite-') as folder:
    root = Path(folder)
    stage_fixtures(Path.cwd(), root / 'fixtures')
    shutil.copy2(binary, root / 'biomcp')
    shutil.copy2(runner, root / 'tool-runner')
    env = {**os.environ, 'BIOMCP_TEST_EXECUTABLE': str(root / 'biomcp'),
           'BIOMCP_TEST_FIXTURES': str(root / 'fixtures')}
    subprocess.run([str(root / 'tool-runner'), '--proofs',
                    'live-source-checks/retained-drift-witnesses'],
                   cwd=root, env=env, check=True)
PYTHON
    ;;
  linux)
    image=$(python3.13 release/toolchain.py image manylinux-aarch64)
    mkdir -p .release-cache/icite-cargo .release-cache/icite-rustup .release-cache/icite-target
    docker run --rm ${JOB_LABEL_FILE:+--label-file "$JOB_LABEL_FILE"} \
      -v "$PWD:/source:ro" -w /source \
      -v "$PWD/.release-cache/icite-cargo:/cargo" \
      -v "$PWD/.release-cache/icite-rustup:/rustup" \
      -v "$PWD/.release-cache/icite-target:/target" \
      -e CARGO_HOME=/cargo -e RUSTUP_HOME=/rustup -e CARGO_TARGET_DIR=/target \
      -e CARGO_BUILD_JOBS=2 -e RUST_TEST_THREADS=2 "$image" /bin/bash -c '
        set -eu
        export PATH=/cargo/bin:/opt/python/cp313-cp313/bin:$PATH
        python3 release/toolchain.py install-rust
        sh scripts/check_icite_discovery.sh installed
      '
    ;;
  *) echo 'usage: check_icite_discovery.sh [focused|routine|fixtures|installed|linux]' >&2; exit 2 ;;
esac
