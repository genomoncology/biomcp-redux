#!/bin/sh
# Run inside a tracked Gator job while this lane has Windows VM custody.
set -eu
cd "$(dirname "$0")/.."
archive=$(mktemp)
trap 'rm -f "$archive"' EXIT
revision=$(git rev-parse HEAD)
git archive "$revision" > "$archive"
ssh -o BatchMode=yes -o ConnectTimeout=15 -p 2222 Docker@localhost 'powershell -NoProfile -Command "New-Item -ItemType Directory -Force C:\h\biomcp-0566 | Out-Null"'
scp -O -o BatchMode=yes -o ConnectTimeout=15 -P 2222 "$archive" Docker@localhost:C:/h/biomcp-0566/source.tar
ssh -o BatchMode=yes -o ConnectTimeout=15 -p 2222 Docker@localhost 'tar -xf C:\h\biomcp-0566\source.tar -C C:\h\biomcp-0566'
ssh -o BatchMode=yes -o ConnectTimeout=15 -p 2222 Docker@localhost "cmd /d /c C:\\h\\biomcp-0566\\scripts\\check_windows_hgnc.cmd $revision"
