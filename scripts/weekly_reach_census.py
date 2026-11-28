"""Measure live source reach, then compare the sample with the checked-in floor.

Use --measure to print baseline candidates. Each line contains only aggregate
counts. Native responses and BioMCP recordings are temporary and stay out of Git.
Re-use the sampling seed across runs to reduce variation from the draw itself.
"""
import argparse
from collections import defaultdict
import json
from pathlib import Path
import re
import subprocess
import tempfile
from urllib.error import HTTPError, URLError
from xml.etree.ElementTree import ParseError

from nightly_live_check import build
from reach_census_common import Census, CensusError, Measurement, offsets, read, source_page
import reach_census_providers as providers
from chembl_reach_census import child_only_parents

NCBI = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi"
GWAS = "https://www.ebi.ac.uk/gwas/rest/api/v2"
CHEMBL = "https://www.ebi.ac.uk/chembl/api/data"
FLOORS = Path(__file__).with_name("weekly_reach_floors.json")
CLINVAR_TYPES = (
    "single nucleotide variant", "deletion", "duplication", "insertion", "indel",
    "microsatellite", "copy number gain", "copy number loss", "inversion",
    "translocation", "complex", "fusion", "haplotype", "diplotype", "protein only",
    "Variation", "tandem duplication",
)
CLINVAR_REVIEWS = (
    "no assertion criteria provided", "criteria provided, single submitter",
    "criteria provided, multiple submitters, no conflicts",
    "criteria provided, conflicting classifications", "reviewed by expert panel",
    "practice guideline", "no classification provided",
    "no classification for the individual variant",
)


def clinvar(census):
    # Use the documented E-Search fields so a term cannot match free text.
    # https://www.ncbi.nlm.nih.gov/clinvar/docs/help/
    for label_prefix, field, classes in (("variation type", "vartype", CLINVAR_TYPES),
                                         ("review status", "revstat", CLINVAR_REVIEWS)):
        for member in classes:
            label = f"{label_prefix}/{member}"

            def fetch(offset):
                params = {"db": "clinvar", "retmode": "json", "retmax": 1,
                          "retstart": offset, "term": f'"{member}"[{field}]'}
                result = census.native(NCBI, params)[0]["esearchresult"]
                if (result.get("errorlist") or {}).get("fieldsnotfound"):
                    raise CensusError("ClinVar did not recognize the census field")
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
            identifiers = {value for value in native if re.fullmatch(r"rs\d+", value)}
            other = set(native) - identifiers
            group = "non-rsid" if other else ("rsid" if identifiers else "no-allele")
            groups[group].append((row["association_id"], identifiers, other))

    reached_by_group, sampled_by_group = [], []
    for group, rows in sorted(groups.items()):
        reached = 0
        for index in offsets(group, len(rows), 20):
            native_id, identifiers, other = rows[index]
            identifier = f"association:{native_id}"
            reply = census.reply(("get", "gwas", identifier))
            association = read(reply, "result", "association") or {}
            mapped = {row["identifier"] for row in association.get("catalog_alleles", [])}
            rsids = {row["rs_id"] for row in association.get("alleles", [])}
            reached += association.get("id") == identifier and identifiers <= rsids and other <= mapped
        reached_by_group.append(reached)
        sampled_by_group.append(min(20, len(rows)))
        # The population for the stratum is the sampled native frame, not an extrapolation.
        yield f"association/{group}", Measurement(len(rows), reached, min(20, len(rows)))
    yield "associations", Measurement(total, sum(reached_by_group), sum(sampled_by_group))


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
                reply = census.reply(("get", "drug", row["pref_name"]), allow_usage=True)
                header = read(reply, "result", "header") or {}
                names = [header.get("name") or "", *(read(reply, "result", "alternate_names") or [])]
                return bool(header.get("id")) and row["pref_name"].casefold() in {
                    name.casefold() for name in names
                }
            reply = census.reply(("get", "drug", identifier, "--source", "chembl"))
            return read(reply, "result", "header", "id") == identifier

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

    families = child_only_parents()
    reached = 0
    sample = offsets("child-only mechanisms", len(families))
    parents = sorted(families)
    for index in sample:
        parent = parents[index]
        reply = census.reply(("get", "drug", parent, "targets", "--source",
                              "chembl", "--section-limit", "1"))
        records = read(reply, "result", "targets", "data", "records") or []
        reached += any(read(row, "drug", "id") in families[parent] for row in records)
    yield "child-only mechanisms", Measurement(len(parents), reached, len(sample))


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
        ("discontinued-application", ("get", "drug", "NDA021335", "markets", "--source", "drugsfda"),
         ("result", "markets", 0, "record")),
        ("biosimilar-name", ("search", "drug", "Zarxio", "--source", "purple-book",
                             "--limit", "1"), ("result", "items")),
        ("biosimilar-reference", ("search", "drug", "Neupogen", "--source", "purple-book",
                                  "--limit", "1"), ("result", "items")),
        ("large-pma", ("get", "diagnostic", "P890003", "--source", "fda-pma"),
         ("result", "header", "pma_number")),
    ):
        reply = census.reply(command, record=True)
        reached = int(bool(read(reply, *path)))
        yield label, Measurement(1, reached, 1)


SOURCES = {
    "clinvar": clinvar, "gwas": gwas, "chembl": chembl, "mychem": mychem, "fda": fda,
    "ctgov": providers.ctgov, "cpic": providers.cpic, "gene-validity": providers.gene_validity,
    "erepo": providers.erepo, "wikipathways": providers.wikipathways, "uniprot": providers.uniprot,
    "interpro": providers.interpro, "opentargets": providers.opentargets,
    "reactome": providers.reactome, "clinpgx": providers.clinpgx, "gnomad": providers.gnomad,
    "orphadata": providers.orphadata,
}


def compare(label, measured, floor):
    if floor is None:
        return f"{label}: new source or record class has no recorded floor"
    reached, sampled = floor
    if not 0 <= reached <= sampled:
        raise ValueError("invalid recorded floor")
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
        except (OSError, ValueError, KeyError, TypeError, IndexError, ParseError,
                subprocess.SubprocessError) as error:
            # Provider text and exception arguments can contain URLs or credentials.
            detail = type(error).__name__
            if isinstance(error, HTTPError):
                detail = f"HTTP {error.code}"
            elif isinstance(error, URLError):
                detail = f"network {type(error.reason).__name__}"
            elif isinstance(error, CensusError):
                detail = str(error)
            failures.append(f"{source}: census incomplete ({detail})")
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
