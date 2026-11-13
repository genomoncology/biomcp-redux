#!/usr/bin/env bash
# One-off cold timing of drug label name lookups (0871), run once per revision through pm job.
# Each drug runs cold in a fresh home, then once more from its cache. Two record runs show each
# openFDA page's latency and size. About 75 openFDA requests, inside the keyless daily quota.
set -euo pipefail
cargo build --locked -q -p biomcp --bin biomcp
B="$PWD/target/debug/biomcp"
W="$(mktemp -d)"
TIMEFORMAT='%R s'
summary() {
  python3 -I -c '
import json, sys
try:
    v = json.load(open(sys.argv[1]))
except Exception:
    print(open(sys.argv[1]).read()[:400]); raise SystemExit
s = (v.get("sources") or [{}])[0]
i = (v.get("result") or {}).get("identity") or {}
n = i.get("name_resolution") or {}
print(v.get("status"), s.get("state"), s.get("reason"), i.get("id"), i.get("name"), n.get("total"))
' "$1"
}
for d in atorvastatin metformin lisinopril amlodipine rosuvastatin quetiapine aripiprazole simvastatin sertraline gabapentin; do
  export HOME="$W/home-$d"
  mkdir -p "$HOME"
  for run in cold cached; do
    echo "=== $d $run"
    { time "$B" get drug "$d" label --label-section indications_and_usage > "$W/$d.out" 2>&1 || echo "exit $?"; } 2>&1
    summary "$W/$d.out"
  done
done
for d in metformin rosuvastatin; do
  export HOME="$W/record-$d"
  mkdir -p "$HOME"
  echo "=== $d record"
  { time "$B" --record "$W/rec-$d" get drug "$d" label --label-section indications_and_usage > "$W/rec-$d.out" 2>&1 || echo "exit $?"; } 2>&1
  summary "$W/rec-$d.out"
  python3 -I -c '
import glob, json, os, sys
rows = []
for f in glob.glob(os.path.join(sys.argv[1], "openfda-label", "*.json")):
    r = json.load(open(f))
    for e in r["exchanges"]:
        skip = r["url"].rsplit("skip=", 1)[1] if "skip=" in r["url"] else ("id" if "spl_set_id" in r["url"] else "0")
        rows.append((e["fetched"], skip, e["delay_ms"], os.path.getsize(f)))
for fetched, skip, delay, size in sorted(rows):
    print(f"page skip={skip} fetched={fetched} delay_ms={delay} file_bytes={size}")
' "$W/rec-$d"
done
