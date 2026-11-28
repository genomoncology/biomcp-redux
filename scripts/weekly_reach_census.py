"""Measure live source reach, then compare the sample with the checked-in floor.

Use --measure to print baseline candidates. Each line contains only aggregate
counts. Native responses and BioMCP recordings are temporary and stay out of Git.
Re-use the sampling seed across runs to reduce variation from the draw itself.
"""
import argparse
from collections import defaultdict
from dataclasses import dataclass
import json
import os
from pathlib import Path
import random
import re
import subprocess
import tempfile
import time
from urllib.parse import urlencode
from urllib.request import Request, urlopen

from nightly_live_check import PASSED_ENVIRONMENT, build

NCBI = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi"
GWAS = "https://www.ebi.ac.uk/gwas/rest/api/v2"
CHEMBL = "https://www.ebi.ac.uk/chembl/api/data"
FLOORS = Path(__file__).with_name("weekly_reach_floors.json")
MAX_SOURCE_BYTES = 64 * 1024 * 1024
SAMPLE_SIZE = 8
CLINVAR_TYPES = (
    "single nucleotide variant", "deletion", "duplication", "insertion", "indel",
    "microsatellite", "copy number gain", "copy number loss", "inversion",
    "translocation", "complex", "fusion", "haplotype", "diplotype", "protein only",
    "Variation", "tandem duplication",
)
CLINVAR_REVIEWS = (
    "no assertion criteria provided", "single submitter",
    "multiple submitters no conflicts", "conflicting classifications",
    "expert panel", "practice guideline", "no classification provided",
)


@dataclass(frozen=True)
class Measurement:
    total: int
    reached: int
    sampled: int

    def __post_init__(self):
        if not 0 <= self.reached <= self.sampled <= self.total:
            raise ValueError("inconsistent measurement")


class Census:
    def __init__(self, executable, home):
        self.executable = executable
        self.home = Path(home)
        self.environment = {name: os.environ[name] for name in PASSED_ENVIRONMENT
                            if name in os.environ}
        self.environment.update(HOME=home, XDG_CACHE_HOME=home + "/cache",
                                XDG_CONFIG_HOME=home + "/config")
        self.last_call = None
        self.recording = 0

    def pace(self):
        if self.last_call is not None:
            remaining = 1 - (time.monotonic() - self.last_call)
            if remaining > 0:
                time.sleep(remaining)
        self.last_call = time.monotonic()

    def native(self, url, params, headers=None):
        self.pace()
        request = Request(url + "?" + urlencode(params), headers=headers or {})
        with urlopen(request, timeout=30) as response:
            content = response.read(MAX_SOURCE_BYTES + 1)
            if len(content) > MAX_SOURCE_BYTES:
                raise ValueError("native reply exceeds the source capture bound")
            return json.loads(content), response.headers

    def reply(self, args, record=False):
        self.pace()
        command = [self.executable, "--format", "json"]
        if record:
            self.recording += 1
            command += ["--record", str(self.home / f"recording-{self.recording}")]
        process = subprocess.run(command + list(args), env=self.environment,
                                 capture_output=True, timeout=120, check=False)
        reply = json.loads(process.stdout)
        if reply.get("status") == "usage" or process.returncode == 2:
            raise ValueError("BioMCP rejected census arguments")
        for source in reply.get("sources", []):
            if source.get("reason") in {
                "deadline-exceeded", "rate-limited", "unavailable", "transport",
                "disabled", "offline", "missing-key",
            }:
                raise ValueError("BioMCP could not observe the source")
        return reply


def offsets(label, total, count=SAMPLE_SIZE):
    return sorted(random.Random(label).sample(range(total), min(total, count)))


def read(document, *path):
    for key in path:
        if not isinstance(document, dict):
            return None
        document = document.get(key)
    return document


def source_page(census, label, fetch, probe):
    """Sample exact native offsets while checking that the reported total is stable."""
    total, _ = fetch(0)
    reached = 0
    sample = offsets(label, total)
    for offset in sample:
        observed, row = fetch(offset)
        if observed != total or row is None:
            raise ValueError("native population changed during its sample")
        reached += bool(probe(row))
    return Measurement(total, reached, len(sample))


def clinvar(census):
    for field, classes in (("Variation Type", CLINVAR_TYPES),
                           ("Review Status", CLINVAR_REVIEWS)):
        for member in classes:
            label = f"{field.lower()}/{member}"

            def fetch(offset):
                params = {"db": "clinvar", "retmode": "json", "retmax": 1,
                          "retstart": offset, "term": f'"{member}"[{field}]'}
                result = census.native(NCBI, params)[0]["esearchresult"]
                return int(result["count"]), next(iter(result["idlist"]), None)

            def probe(variant):
                reply = census.reply(("variant", "assertions", f"clinvar:{variant}",
                                     "--source", "clinvar", "--kind", "scv", "--limit", "1"))
                return bool(read(reply, "result", "assertions"))

            yield label, source_page(census, label, fetch, probe)


def gwas(census):
    for endpoint in ("studies", "unpublished-studies"):
        def fetch(offset):
            body, _ = census.native(f"{GWAS}/{endpoint}", {"page": offset, "size": 1})
            rows = body.get("_embedded", {})
            return body["page"]["totalElements"], next(iter(next(iter(rows.values()), [])), None)

        def probe(row):
            identifier = row["accession_id"]
            reply = census.reply(("get", "gwas", identifier))
            return read(reply, "result", "id") == identifier

        yield endpoint, source_page(census, endpoint, fetch, probe)

    first, _ = census.native(f"{GWAS}/associations", {"page": 0, "size": 50})
    total = first["page"]["totalElements"]
    groups = defaultdict(list)
    for page in offsets("gwas associations", first["page"]["totalPages"], 20):
        body, _ = census.native(f"{GWAS}/associations", {"page": page, "size": 50})
        if body["page"]["totalElements"] != total:
            raise ValueError("association total changed during sampling")
        for row in body["_embedded"]["associations"]:
            native = [allele["rs_id"] for allele in row.get("snp_allele") or []]
            identifiers = [value for value in native if not re.fullmatch(r"rs\d+", value)]
            group = "non-rsid" if identifiers else ("rsid" if native else "no-allele")
            groups[group].append((row["association_id"], identifiers))

    frame = sum(map(len, groups.values()))
    for group, rows in sorted(groups.items()):
        reached = 0
        for index in offsets(group, len(rows), 20):
            native_id, identifiers = rows[index]
            identifier = f"association:{native_id}"
            reply = census.reply(("get", "gwas", identifier))
            association = read(reply, "result", "association") or {}
            mapped = {row["identifier"] for row in association.get("catalog_alleles", [])}
            reached += association.get("id") == identifier and set(identifiers) <= mapped
        # The population for the stratum is the sampled native frame, not an extrapolation.
        yield f"association/{group}", Measurement(len(rows), reached, min(20, len(rows)))
    yield "associations", Measurement(total, frame, frame)


CHEMBL_CLASSES = {
    "approved-small-molecule": {"max_phase": 4, "molecule_type": "Small molecule"},
    "approved-antibody": {"max_phase": 4, "molecule_type": "Antibody"},
    "approved-protein": {"max_phase": 4, "molecule_type": "Protein"},
    "phase-3": {"max_phase": 3},
    "phase-2": {"max_phase": 2},
    "phase-1": {"max_phase": 1},
    "phase-0.5": {"max_phase": 0.5},
    "withdrawn": {"withdrawn_flag": "true"},
    "natural-product": {"natural_product": 1},
    "unnamed": {"pref_name__isnull": "true"},
}


def chembl_molecules(census, by_name):
    for label, filters in CHEMBL_CLASSES.items():
        if by_name and label == "unnamed":
            continue

        def fetch(offset):
            params = dict(filters, limit=1, offset=offset, order_by="molecule_chembl_id")
            if by_name:
                params["pref_name__isnull"] = "false"
            reply, _ = census.native(f"{CHEMBL}/molecule.json", params)
            return reply["page_meta"]["total_count"], next(iter(reply["molecules"]), None)

        def probe(row):
            identifier = row["molecule_chembl_id"]
            if by_name:
                reply = census.reply(("get", "drug", row["pref_name"]))
                return bool(read(reply, "result", "identity", "id"))
            reply = census.reply(("get", "drug", identifier, "--source", "chembl"))
            return read(reply, "result", "identity", "id") == identifier

        yield label, source_page(census, label, fetch, probe)


def chembl(census):
    yield from chembl_molecules(census, False)
    for endpoint, section in (("mechanism", "targets"), ("drug_indication", "indications")):
        def fetch(offset):
            body, _ = census.native(f"{CHEMBL}/{endpoint}.json", {"offset": offset, "limit": 1})
            return body["page_meta"]["total_count"], next(iter(body[endpoint + "s"]), None)

        def probe(row):
            identifier = row["molecule_chembl_id"]
            reply = census.reply(("get", "drug", identifier, section, "--source",
                                  "chembl", "--section-limit", "1"))
            return read(reply, "result", section, "state") == "data"

        yield endpoint, source_page(census, endpoint, fetch, probe)


def mychem(census):
    yield from chembl_molecules(census, True)


def fda(census):
    # The FDA sample uses fixed witnesses from the census. Only BioMCP can reach FDA.
    for label, command, path in (
        ("prescription-label", ("search", "drug", "imatinib", "--source", "openfda-label",
                                "--limit", "1"), ("result", "items")),
        ("vaccine-label", ("search", "drug", "Shingrix", "--source", "openfda-label",
                           "--limit", "1"), ("result", "items")),
        ("cell-therapy-label", ("get", "drug", "Kymriah", "label",
                                "--label-section", "boxed_warning"), ("result", "label", "data")),
        ("discontinued-application", ("get", "drug", "NDA021335", "markets"),
         ("result", "markets", "data")),
        ("biosimilar-name", ("search", "drug", "Zarxio", "--source", "purple-book",
                             "--limit", "1"), ("result", "items")),
        ("biosimilar-reference", ("search", "drug", "Neupogen", "--source", "purple-book",
                                  "--limit", "1"), ("result", "items")),
        ("large-pma", ("get", "diagnostic", "P890003", "--source", "fda-pma"),
         ("result", "header", "id")),
    ):
        reply = census.reply(command, record=True)
        reached = int(bool(read(reply, *path)))
        yield label, Measurement(1, reached, 1)


SOURCES = {"clinvar": clinvar, "gwas": gwas, "chembl": chembl, "mychem": mychem, "fda": fda}


def compare(label, measured, floor):
    if floor is None:
        return f"{label}: new source or record class has no recorded floor"
    reached, sampled = floor
    if measured.reached * sampled < reached * measured.sampled or (
        sampled > 0 and measured.sampled == 0
    ):
        return (f"{label}: reached {measured.reached}/{measured.sampled}, "
                f"below floor {reached}/{sampled}")
    return None


def run(census, sources, floors):
    failures = []
    for source in sources:
        label = source
        seen = set()
        try:
            for group, measured in SOURCES[source](census):
                label = f"{source}/{group}"
                seen.add(group)
                print(json.dumps({"source": source, "class": group, **vars(measured)}), flush=True)
                if floors is not None:
                    failure = compare(label, measured, floors.get(source, {}).get(group))
                    if failure:
                        failures.append(failure)
        except (OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
            # Provider text and exception arguments can contain URLs or credentials.
            failures.append(f"{label}: census incomplete ({type(error).__name__})")
        if floors is not None:
            failures.extend(f"{source}/{group}: recorded class was not measured"
                            for group in sorted(floors.get(source, {}).keys() - seen))
    for failure in failures:
        print(f"FAIL {failure}", flush=True)
    return int(bool(failures))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--measure", action="store_true", help="Print baseline candidates")
    parser.add_argument("--source", action="append", choices=SOURCES,
                        help="Run only the selected source (repeatable)")
    args = parser.parse_args()
    try:
        floors = None if args.measure else json.loads(FLOORS.read_text())
        executable = build()
    except (OSError, ValueError) as error:
        print(f"FAIL census setup ({type(error).__name__})")
        return 1
    with tempfile.TemporaryDirectory(prefix="biomcp-weekly-reach-") as home:
        return run(Census(executable, home), args.source or SOURCES, floors)


if __name__ == "__main__":
    raise SystemExit(main())
