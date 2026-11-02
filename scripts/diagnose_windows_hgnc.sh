#!/bin/sh
# One no-compile job, retaining only named output records before releasing the VM.
set -eu
cd "$(dirname "$0")/.."
scratch=$(mktemp -d)
cleanup() {
    result=$?
    rm -rf "$scratch"
    docker stop windows-2022 >/dev/null || true
    exit "$result"
}
trap cleanup EXIT
revision=$(git rev-parse HEAD)
ssh -o BatchMode=yes -o ConnectTimeout=15 yellow true
docker start windows-2022 >/dev/null
ssh -o BatchMode=yes -o ConnectTimeout=3 -o ConnectionAttempts=60 -p 2222 Docker@localhost 'cmd /c ver' >/dev/null
scp -O -o BatchMode=yes -o ConnectTimeout=15 -P 2222 scripts/diagnose_hgnc_baseline.py Docker@localhost:C:/h/biomcp-0566/diagnose_hgnc_baseline.py
set +e
ssh -o BatchMode=yes -o ConnectTimeout=15 -p 2222 Docker@localhost "C:\\Users\\Docker\\AppData\\Roaming\\uv\\python\\cpython-3.13.13-windows-x86_64-none\\python.exe C:\\h\\biomcp-0566\\diagnose_hgnc_baseline.py $revision"
result=$?
set -e
short=$(printf '%.12s' "$revision")
scp -O -o BatchMode=yes -o ConnectTimeout=15 -P 2222 "Docker@localhost:C:/h/biomcp-0566/hgnc-bounded-output-$short.tar.gz" "$scratch/records.tar.gz"
scp -o BatchMode=yes -o ConnectTimeout=15 "$scratch/records.tar.gz" "yellow:/home/ian/workspace/data/biomcp-redux-evidence/0566/windows-bounded-$short.tar.gz"
exit "$result"
