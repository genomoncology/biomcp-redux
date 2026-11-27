"""Count accepted evidence addressable by the checkout's CIViC routes."""
import argparse
from collections import Counter
import csv
from email.utils import parsedate_to_datetime
import io
import json
from pathlib import Path
import re
import subprocess
import tempfile
from urllib.request import urlopen

NIGHTLY = "https://civicdb.org/downloads/nightly/nightly-"
KINDS = ("gene_with_caid", "gene_without_caid", "factor", "fusion", "region")
ROUTES = ("variant", "drug", "profile", "any")
BUCKETS = ("all", "with_therapy", "without_therapy") + tuple(
    name for kind in KINDS for name in (kind, f"{kind}/with_therapy", f"{kind}/without_therapy")
)


def nightly(name):
    with urlopen(f"{NIGHTLY}{name}Summaries.tsv", timeout=30) as response:
        date = parsedate_to_datetime(response.headers["Last-Modified"]).date().isoformat()
        with io.TextIOWrapper(response, encoding="utf-8") as text:
            rows = list(csv.DictReader(text, delimiter="\t"))
    if not rows:
        raise ValueError(f"empty CIViC {name} export")
    return date, rows


def index(rows, key):
    result = {int(row[key]): row for row in rows}
    if len(result) != len(rows):
        raise ValueError(f"duplicate {key}")
    return result


def source_route(binary, env, command, *, optional=False):
    result = subprocess.run(
        [binary, "--offline", "--format", "json", "get", *command],
        env=env, capture_output=True, check=False, timeout=30,
    )
    body = json.loads(result.stdout)
    if result.returncode == 2 and body.get("error", {}).get("code") == "usage.arguments":
        if optional:
            return False
    if [row["source"] for row in body.get("sources", [])] != ["civic"]:
        raise ValueError(f"{command}: expected a direct CIViC source route, got {body.get('status')}")
    return True


def route_support(binary):
    with tempfile.TemporaryDirectory() as root:
        env = {"HOME": root, **{
            f"XDG_{kind}_HOME": str(Path(root) / kind.lower())
            for kind in ("CONFIG", "CACHE", "DATA", "STATE")
        }}
        caid = source_route(binary, env, ["variant", "CA123643", "civic"])
        drug = source_route(binary, env, ["drug", "138", "civic", "--source", "civic"])
        # The exact name distinguishes this surface from an old bare ClinVar number.
        profile = source_route(binary, env, ["variant", "MSI High", "civic"], optional=True)
        if profile:
            source_route(binary, env, ["variant", "5118", "civic"])
    return caid, drug, profile


def profile_class(members):
    kinds = set()
    allele = False
    for member in members:
        kind = member["feature_type"].lower()
        caid = re.fullmatch(r"CA[0-9]+", member["allele_registry_id"]) is not None
        if kind == "gene":
            kind = "gene_with_caid" if caid else "gene_without_caid"
        if kind not in KINDS:
            raise ValueError(f"unknown CIViC feature type {kind!r}")
        kinds.add(kind)
        allele |= caid
    return kinds, allele


def census(support):
    releases = [nightly(name) for name in ("ClinicalEvidence", "MolecularProfile", "Variant")]
    dates = {date for date, _ in releases}
    if len(dates) != 1:
        raise ValueError(f"CIViC nightly exports have different dates {sorted(dates)}")
    evidence = index(releases[0][1], "evidence_id")
    profiles = index(releases[1][1], "molecular_profile_id")
    variants = index(releases[2][1], "variant_id")
    counts = {bucket: Counter(total=0, **dict.fromkeys(ROUTES, 0)) for bucket in BUCKETS}
    for row in evidence.values():
        if row["evidence_status"] != "accepted":
            continue
        profile = profiles[int(row["molecular_profile_id"])]
        members = [variants[int(variant)] for variant in profile["variant_ids"].split(",")]
        kinds, allele = profile_class(members)
        therapy = bool(row["therapies"].strip())
        grouping = "with_therapy" if therapy else "without_therapy"
        reached = (support[0] and allele, support[1] and therapy, support[2])
        for bucket in {"all", grouping, *kinds, *(f"{kind}/{grouping}" for kind in kinds)}:
            counts[bucket]["total"] += 1
            counts[bucket].update(route for route, ok in zip(ROUTES, (*reached, any(reached))) if ok)
    if not counts["all"]["total"]:
        raise ValueError("CIViC nightly export contains no accepted evidence")
    return {"release_date": dates.pop(), "buckets": counts}


def regressions(current, floor):
    for bucket in BUCKETS:
        live = current["buckets"][bucket]
        minimum = floor["buckets"][bucket]
        for route in ROUTES:
            if live[route] < minimum[route] or live[route] * minimum["total"] < minimum[route] * live["total"]:
                yield f"{bucket}/{route}: {live[route]}/{live['total']} below {minimum[route]}/{minimum['total']}"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--biomcp", type=Path, required=True)
    parser.add_argument("--report-only", action="store_true", help="measure without applying a floor")
    args = parser.parse_args()
    result = census(route_support(args.biomcp.resolve()))
    print(json.dumps(result, separators=(",", ":")))
    if not args.report_only:
        floor = json.loads(Path(__file__).with_name("civic_reach_floor.json").read_text())
        failures = list(regressions(result, floor))
        if failures:
            parser.exit(1, "\n".join(failures) + "\n")


if __name__ == "__main__":
    main()
