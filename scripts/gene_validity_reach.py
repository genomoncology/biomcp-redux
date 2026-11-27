"""Sample disease-first reach against the public ClinGen and GenCC validity exports."""

import argparse
from collections import defaultdict
import csv
import io
import json
import os
from pathlib import Path
import random
import subprocess
import tempfile
import time

from nightly_live_check import PASSED_ENVIRONMENT


def invoke(executable, env, *args):
    result = subprocess.run([executable, *args], env=env, capture_output=True, timeout=120)
    if not result.stdout:
        raise SystemExit(f"BioMCP did not return JSON for {' '.join(args[-4:])}")
    return json.loads(result.stdout)


def export_rows(capture, source):
    path, = (capture / source).glob("*.json")
    body = json.loads(path.read_text())["exchanges"][0]["body"]["value"]
    lines = io.StringIO(body)
    if source == "clingen":
        for line in lines:
            if "GENE ID (HGNC)" in line:
                lines.seek(lines.tell() - len(line))
                break
    return csv.DictReader(lines)


def probes(capture):
    groups = defaultdict(list)
    for row in export_rows(capture, "clingen"):
        disease = row["DISEASE ID (MONDO)"]
        if disease.startswith("++"):
            continue
        namespace = disease.partition(":")[0] or "missing"
        groups["clingen", namespace].append((disease, (row["ONLINE REPORT"],)))
    for row in export_rows(capture, "gencc"):
        submitted = row["submitted_as_disease_id"]
        namespace = submitted.partition(":")[0].upper() or "missing"
        if namespace == "ORPHA":
            namespace = "ORPHANET"
        groups["gencc", namespace].append(
            (submitted or row["disease_curie"], (row["sgc_id"], row["version_number"]))
        )
    return groups


def covering(executable, env, source, disease):
    if not disease:
        return set()
    offset = 0
    found = set()
    if disease.upper().startswith(("OMIM:", "ORPHA:", "ORPHANET:")):
        time.sleep(1)
    while True:
        reply = invoke(
            executable, env, "get", "disease", disease, "gene-validity", "--source", source,
            "--section-limit", "50", "--section-offset", str(offset),
        )
        for item in reply.get("result", {}).get("items", []):
            assertion = item["assertion"]
            if source == "clingen":
                found.add((assertion.get("report_url"),))
            else:
                found.add((assertion["id"], assertion["version"]))
        page = reply.get("page", {})
        returned = page.get("returned", 0)
        offset += returned
        if returned == 0 or offset >= page["total"]:
            return found


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("executable", type=lambda path: Path(path).resolve())
    args = parser.parse_args()
    rng = random.Random(1212)
    with tempfile.TemporaryDirectory(prefix="biomcp-validity-") as home:
        env = {name: os.environ[name] for name in PASSED_ENVIRONMENT if name in os.environ}
        env.update(HOME=home, XDG_CACHE_HOME=home + "/cache", XDG_CONFIG_HOME=home + "/config")
        capture = Path(home) / "capture"
        synced = invoke(args.executable, env, "--record", capture, "sync", "gencc")
        if synced["status"] != "complete":
            raise SystemExit(f"GenCC export capture returned {synced['status']}")
        # Read the validity export alone. Explicit ClinGen sync downloads dosage as well.
        warm = invoke(args.executable, env, "--record", capture, "get", "disease",
                      "MONDO:0008426", "gene-validity", "--source", "clingen")
        if warm["status"] != "complete":
            raise SystemExit(f"ClinGen export capture returned {warm['status']}")
        answers = {}
        for (source, namespace), rows in sorted(probes(capture).items()):
            sample = rng.sample(rows, min(16, len(rows)))
            reached = 0
            for disease, row in sample:
                key = (source, disease)
                if key not in answers:
                    answers[key] = covering(args.executable, env, source, disease)
                reached += row in answers[key]
            print(f"{source} {namespace}: {len(rows)} export rows, "
                  f"{reached}/{len(sample)} sampled rows reached ({reached / len(sample):.1%})",
                  flush=True)


if __name__ == "__main__":
    main()
