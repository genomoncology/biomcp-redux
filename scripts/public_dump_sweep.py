"""Sweep pinned local public releases through BioData; never download data.

Build the test bridge with cargo build -p biomcp-types --example public_dump_values.
Pass a JSON manifest with sources: [{kind, path, release, url, rights, sha256}].
Paths resolve against the manifest directory and dumps must remain outside Git.
Kinds: civic_variants, civic_profiles, civic_evidence, clinvar_summary,
clingen_summary, clingen_evidence, clinvar_gks. ClinGen evidence accepts one
original public interpretation response per file; GKS accepts the keyed bundle.
Only CC0-1.0 and public-domain inputs are accepted. Pin the compressed file hash.
This tool checks mapped source fields. Profile member IDs are checked separately without
inferring allele identities or reconstructing their source logical expression.
Use --observed with saved CLI envelopes as JSONL: {source, release, revision,
retrieved, record, reply}. Source is civic or clinvar; ClinVar record is its
clinvar: Variation ID. Release must exactly match the source manifest. Civic
replies must cover every source gene. --values-only explicitly omits comparison.
Exit 0 means complete with no refusals or mapping gaps; 1 means findings or a
bounded partial run; 2 means an input, transport or tool failure.
"""
import argparse
from collections import Counter, defaultdict
import gzip
import hashlib
import json
from pathlib import Path
import subprocess
import sys

from public_dump_formats import REQUIRED, records
from public_dump_compare import Comparison

EXAMPLE_LIMIT = 10
GENE_LIMIT = 20


def arguments():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("manifest", type=Path)
    parser.add_argument("--values", type=Path, required=True, help="built public_dump_values bridge")
    parser.add_argument("--max-records", type=int, help="bound records per file; marks report incomplete")
    parser.add_argument("--output", type=Path, help="save full gene counts; stdout is bounded")
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--observed", type=Path, help="saved real CLI replies with source/release identity")
    mode.add_argument("--values-only", action="store_true", help="parse values without claiming reply equality")
    return parser.parse_args()


def pinned_sources(manifest):
    sources = json.loads(manifest.read_text())["sources"]
    if not isinstance(sources, list) or not sources:
        raise ValueError("manifest_empty_sources")
    for source in sources:
        required = {"kind", "path", "release", "url", "rights", "sha256"}
        if not required <= source.keys() or any(not source[key] for key in required):
            raise ValueError("manifest_missing_pin")
        if source["kind"] not in {*REQUIRED, "clinvar_gks", "clingen_evidence"}:
            raise ValueError("manifest_unknown_source_kind")
        if source["rights"] not in {"CC0-1.0", "public-domain"}:
            raise ValueError("manifest_unapproved_rights")
        path = (manifest.parent / source["path"]).resolve()
        if any((parent / ".git").exists() for parent in path.parents):
            raise ValueError("dump_must_be_outside_git")
        digest = hashlib.sha256()
        with path.open("rb") as handle:
            for chunk in iter(lambda: handle.read(1024 * 1024), b""):
                digest.update(chunk)
        if digest.hexdigest() != source["sha256"]:
            raise ValueError(f"checksum_mismatch: {path}")
        yield source, path


class Sweep:
    def __init__(self, bridge):
        self.bridge = bridge
        self.types = defaultdict(Counter)
        self.reasons = Counter()
        self.refusals = []
        self.genes = Counter()
        self.unmapped = Counter()
        self.sources = []
        self.spelling_mismatches = 0
        self.complete = True
        self.comparison = Comparison()

    def parse(self, values, source, identity):
        self.bridge.stdin.write(json.dumps(values) + "\n")
        self.bridge.stdin.flush()
        line = self.bridge.stdout.readline()
        if not line:
            raise ValueError("value_bridge_closed")
        outputs = json.loads(line)
        if len(outputs) != len(values):
            raise ValueError("value_bridge_count_mismatch")
        refused = False
        for original, result in zip(values, outputs):
            if result["kind"] != original["kind"] or result["value"] != original["value"]:
                raise ValueError("value_bridge_identity_mismatch")
            kind = result["kind"]
            reason = result.get("reason")
            if not reason and kind in {"variant", "clinvar_variation", "clinvar_allele", "vcv", "rcv", "scv"}:
                if result["parsed"] != result["value"]:
                    reason = "source_identifier_spelling_changed"
                    self.spelling_mismatches += 1
            self.types[kind]["refused" if reason else "parsed"] += 1
            if reason:
                refused = True
                self.reasons[f"{kind}: {reason}"] += 1
                if len(self.refusals) < EXAMPLE_LIMIT:
                    self.refusals.append({"source": source, "record": identity, "type": kind,
                                          "value": result["value"], "reason": reason})
        return refused

    def read(self, source, path, maximum):
        count = refused = empty = 0
        opener = gzip.open if path.suffix == ".gz" else open
        with opener(path, "rt", encoding="utf-8", newline="") as handle:
            for identity, values, genes, unmapped, context in records(handle, source["kind"]):
                if maximum is not None and count >= maximum:
                    self.complete = False
                    break
                count += 1
                refused += self.parse(values, source["kind"], identity)
                empty += not values
                self.genes.update(genes)
                self.comparison.add(context, source["kind"], source["release"])
                if unmapped:
                    self.unmapped[unmapped] += 1
        self.sources.append({**source, "records": count, "parsed_records": count - refused - empty,
                             "refused_records": refused, "records_without_values": empty})

    def report(self):
        return {"complete": self.complete, "sources": self.sources,
                "types": {kind: {"parsed": counts["parsed"], "refused": counts["refused"]}
                          for kind, counts in sorted(self.types.items())},
                "refusal_reasons": dict(self.reasons), "refusals": self.refusals,
                "gene_counts": dict(sorted(self.genes.items())),
                "unmapped_sections": dict(self.unmapped),
                "spelling_mismatches": self.spelling_mismatches}


def run(args):
    if args.max_records is not None and args.max_records <= 0:
        raise ValueError("max_records_must_be_positive")
    # Validate every file before starting the bridge or reporting partial success.
    sources = list(pinned_sources(args.manifest.resolve()))
    with subprocess.Popen([str(args.values.resolve())], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, text=True) as bridge:
        try:
            sweep = Sweep(bridge)
            for source, path in sources:
                sweep.read(source, path, args.max_records)
            bridge.stdin.close()
            if bridge.wait() != 0:
                raise ValueError("value_bridge_failed")
        except BaseException:
            bridge.kill()
            bridge.wait()
            raise
    report = sweep.report()
    try:
        report["comparisons"] = sweep.comparison.compare(args.observed, sweep.genes) if args.observed else None
    finally:
        sweep.comparison.close()
    if args.output:
        with args.output.open("x", encoding="utf-8") as output:
            json.dump(report, output, indent=2, sort_keys=True)
            output.write("\n")
    report["gene_counts_omitted"] = max(0, len(report["gene_counts"]) - GENE_LIMIT)
    report["gene_counts"] = dict(list(report["gene_counts"].items())[:GENE_LIMIT])
    reasons = report["refusal_reasons"]
    report["refusal_reasons_omitted"] = max(0, len(reasons) - EXAMPLE_LIMIT)
    report["refusal_reasons"] = dict(list(reasons.items())[:EXAMPLE_LIMIT])
    report["refusals"] = [{**refusal, "value": str(refusal["value"])[:512],
                           "reason": refusal["reason"][:512]} for refusal in report["refusals"]]
    print(json.dumps(report, indent=2, sort_keys=True))
    comparisons_failed = report["comparisons"] and report["comparisons"]["failures"]
    failures = sweep.reasons or sweep.unmapped or comparisons_failed or any(row["records_without_values"] for row in sweep.sources)
    return int(bool(failures) or not sweep.complete)


def main():
    try:
        return run(arguments())
    except (OSError, ValueError, KeyError, TypeError, BrokenPipeError) as error:
        print(f"public_dump_sweep: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
