"""Remake the FDA Drug Safety Communications seed that BioMCP ships, with BioMCP's own reader.

Usage: python3 scripts/fda_dsc_seed.py BIOMCP_EXECUTABLE [SEED_FILE]

It lists FDA's index with `search drug --source fda-dsc`, then reads each communication's page
with `get drug ID --source fda-dsc`, 30 seconds apart as FDA's robots.txt asks (Crawl-Delay: 30),
and writes each page's row and visible text. If FDA refuses a request or the seed exceeds
1.5 MB, the script reports the failure and leaves the previous seed in place. It uses a
throwaway cache. A full run reads about 45 pages and takes about 23 minutes.
"""

import datetime
import json
import os
import subprocess
import sys
import tempfile
import textwrap
import time

SEED = os.path.join(
    os.path.dirname(os.path.abspath(__file__)),
    "..",
    "crates",
    "biomcp-adapters",
    "src",
    "fda_dsc",
    "seed.json",
)
SPACING_SECONDS = 30
MAX_BYTES = 1_500_000


def run(executable, cache, *args):
    env = dict(os.environ, XDG_CACHE_HOME=cache, XDG_CONFIG_HOME=cache)
    out = subprocess.run([executable, *args], env=env, capture_output=True, text=True, check=False)
    return json.loads(out.stdout)


def main(argv):
    if len(argv) not in (2, 3):
        sys.stderr.write(__doc__)
        return 2
    executable, target = argv[1], argv[2] if len(argv) == 3 else SEED
    with tempfile.TemporaryDirectory() as cache:
        rows = []
        while True:
            listed = run(executable, cache, "search", "drug", "--source", "fda-dsc", "--limit", "50", "--offset", str(len(rows)))
            if listed.get("status") != "complete":
                sys.stderr.write(f"index not read: {json.dumps(listed)[:2000]}\n")
                return 1
            rows.extend(listed["result"].get("items", []))
            if not listed["result"].get("items") or len(rows) >= listed["result"]["total"]:
                break
        items = []
        for number, row in enumerate(rows):
            time.sleep(SPACING_SECONDS)
            got = run(executable, cache, "get", "drug", row["id"], "--source", "fda-dsc")
            result = got.get("result") or {}
            header = result.get("header")
            if got.get("status") != "complete" or not header or not result.get("text"):
                sys.stderr.write(f"{row['id']}: page not read: {json.dumps(got.get('sources'))}\n")
                return 1
            header["text"] = textwrap.wrap(result["text"], width=120, break_long_words=False, break_on_hyphens=False)
            items.append(header)
            sys.stderr.write(f"{number + 1}/{len(rows)} {row['id']}\n")
    seed = {"date": datetime.date.today().isoformat(), "items": items}
    content = (json.dumps(seed, indent=1, ensure_ascii=False) + "\n").encode()
    if len(content) > MAX_BYTES:
        sys.stderr.write(f"Seed would be {len(content):,} bytes (limit {MAX_BYTES:,}).\n")
        return 1
    with open(target, "wb") as handle:
        handle.write(content)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
