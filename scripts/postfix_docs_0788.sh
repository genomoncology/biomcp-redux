#!/bin/sh
set -eu
mkdir -p "$TMPDIR/python-bin"
ln -s "$(command -v python3.13)" "$TMPDIR/python-bin/python3"
export PATH="$TMPDIR/python-bin:$PATH"
export RUSTUP_TOOLCHAIN=1.95.0
python3 --version
make docs-check
