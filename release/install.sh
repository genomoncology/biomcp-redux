#!/bin/bash
# Install a local release archive. Compatible with stock macOS bash.
set -euo pipefail
if [ "$#" -ne 3 ]; then
  echo 'Usage: install.sh ARCHIVE SHA256 INSTALL_DIR' >&2
  exit 2
fi
archive=$1
expected=$2
install_dir=$3
if command -v sha256sum >/dev/null 2>&1; then
  actual=$(sha256sum "$archive")
else
  actual=$(shasum -a 256 "$archive")
fi
actual=${actual%% *}
[ "$actual" = "$expected" ] || { echo 'Archive checksum mismatch' >&2; exit 1; }
[ "$(tar -tzf "$archive")" = 'biomcp' ] || { echo 'Unexpected archive contents' >&2; exit 1; }
mkdir -p "$install_dir"
tar -xzf "$archive" -C "$install_dir"
chmod 755 "$install_dir/biomcp"
