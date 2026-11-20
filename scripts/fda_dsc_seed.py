"""Remake the FDA Drug Safety Communications seed that BioMCP ships, with BioMCP's own reader.

Usage: python3 scripts/fda_dsc_seed.py BIOMCP_EXECUTABLE [SEED_FILE]

It lists FDA's index with `search drug --source fda-dsc`, then reads each communication's page
with `get drug ID --source fda-dsc`, 30 seconds apart as FDA's robots.txt asks (Crawl-Delay: 30),
and writes each page's row, without its text, to the seed file. A full run reads about 45 pages
and takes about 23 minutes. It uses a throwaway cache, so every page is read fresh.
"""

import datetime
import json
import os
import subprocess
import sys
import tempfile
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
            header = (got.get("result") or {}).get("header")
            if got.get("status") != "complete" or not header:
                sys.stderr.write(f"{row['id']}: page not read: {json.dumps(got.get('sources'))}\n")
                return 1
            items.append(header)
            sys.stderr.write(f"{number + 1}/{len(rows)} {row['id']}\n")
    seed = {"date": datetime.date.today().isoformat(), "items": items}
    with open(target, "w", encoding="utf-8") as handle:
        json.dump(seed, handle, indent=1, ensure_ascii=False)
        handle.write("\n")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
