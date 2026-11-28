"""Native sample frames for the remaining live reach providers."""
from collections import defaultdict
import json
from pathlib import Path
import re
from xml.etree import ElementTree

from gene_validity_reach import probes as validity_probes
from reach_census_common import MAX_SOURCE_BYTES, CensusError, read, sample_rows, source_page

CTGOV = "https://clinicaltrials.gov/api/v2"
CPIC = "https://api.cpicpgx.org/v1"
EREPO = "https://erepo.clinicalgenome.org/evrepo/api/summary/classifications"
UNIPROT = "https://rest.uniprot.org/uniprotkb/search"
INTERPRO = "https://www.ebi.ac.uk/interpro/api"
OPEN_TARGETS = "https://api.platform.opentargets.org/api/v4/graphql"

TRIAL_CLASSES = {
    "OverallStatus": ("NOT_YET_RECRUITING", "RECRUITING", "ENROLLING_BY_INVITATION",
                      "ACTIVE_NOT_RECRUITING", "COMPLETED", "SUSPENDED", "TERMINATED",
                      "WITHDRAWN", "UNKNOWN", "AVAILABLE", "NO_LONGER_AVAILABLE",
                      "TEMPORARILY_NOT_AVAILABLE", "APPROVED_FOR_MARKETING", "WITHHELD"),
    "StudyType": ("INTERVENTIONAL", "OBSERVATIONAL", "EXPANDED_ACCESS"),
    "Phase": ("EARLY_PHASE1", "PHASE1", "PHASE2", "PHASE3", "PHASE4", "NA"),
    "InterventionType": ("BEHAVIORAL", "BIOLOGICAL", "COMBINATION_PRODUCT", "DEVICE",
                         "DIAGNOSTIC_TEST", "DIETARY_SUPPLEMENT", "DRUG", "GENETIC",
                         "PROCEDURE", "RADIATION", "OTHER"),
}


def ctgov(census):
    for field, classes in TRIAL_CLASSES.items():
        for member in classes:
            label = f"{field}/{member}"
            params = {"countTotal": "true", "pageSize": 100, "fields": "NCTId",
                      "filter.advanced": f"AREA[{field}]{member}"}
            body, _ = census.native(f"{CTGOV}/studies", params)
            rows = [row["protocolSection"]["identificationModule"]["nctId"]
                    for row in body["studies"]]

            def probe(identifier):
                args = ("get", "trial", identifier)
                if field == "InterventionType":
                    args += ("arms", "--section-limit", "50")
                items = read(census.reply(args), "result", "items") or []
                for item in items:
                    card = item.get("card", {})
                    if card.get("nct_id") != identifier:
                        continue
                    if field == "OverallStatus":
                        return card.get("status") == member
                    if field == "StudyType":
                        return card.get("study_type") == member
                    if field == "Phase":
                        return member in card.get("phases", [])
                    arms = read(item, "sections", "arms", "data", "items") or []
                    return any(read(arm, "data", "kind") == member for arm in arms)
                return False

            # The registry uses sequential cursors. Sample from its first native page.
            yield label, sample_rows(label, rows, probe, body["totalCount"])


def cpic(census):
    for table, order in (("pair", "pairid"), ("diplotype", "genesymbol,diplotype")):
        def fetch(offset):
            body, headers = census.native(
                f"{CPIC}/{table}", {"limit": 1, "offset": offset, "order": order},
                {"Prefer": "count=exact"},
            )
            return int(headers["Content-Range"].split("/")[-1]), next(iter(body), None)

        def probe(row):
            gene = row["genesymbol"]
            if table == "diplotype":
                spelling = row["diplotype"]
                items = read(census.reply(("get", "pgx", "diplotype", "--gene", gene,
                                           "--diplotype", spelling, "--source", "cpic")),
                             "result", "diplotypes") or []
                return any(item["diplotype"] == spelling for item in items)
            offset = 0
            while offset < 1000:
                reply = census.reply(("search", "pgx", "--gene", gene, "--source", "cpic",
                                      "--offset", str(offset), "--limit", "50"))
                items = read(reply, "result", "items") or []
                if any(read(item, "pairid", "id") == row["pairid"] for item in items):
                    return True
                following = read(reply, "result", "page", "next_offset")
                if following is None:
                    return False
                offset = following
            raise CensusError("CPIC pair lookup exceeds its paging window")

        yield table, source_page(census, table, fetch, probe)


def gene_validity(census):
    # Use the same recorder and source-row parser as the disease-first reach check.
    census.reply(("sync", "gencc"), record="gencc")
    census.reply(("get", "disease", "MONDO:0008426", "gene-validity",
                  "--source", "clingen"), record="clingen")
    for (source, namespace), rows in sorted(validity_probes(census.home).items()):
        def probe(row):
            disease, key = row
            if not disease:
                return False
            offset = 0
            while offset <= 500:
                reply = census.reply(("get", "disease", disease, "gene-validity",
                                      "--source", source, "--section-limit", "50",
                                      "--section-offset", str(offset)), allow_usage=True)
                items = read(reply, "result", "items") or []
                for item in items:
                    assertion = item["assertion"]
                    identity = ((assertion.get("report_url"),) if source == "clingen"
                                else (assertion.get("id"), assertion.get("version")))
                    if identity == key:
                        return True
                offset += len(items)
                if not items or offset >= read(reply, "page", "total"):
                    return False
            raise CensusError("validity sample exceeds paging window")

        label = f"{source}/{namespace}"
        yield label, sample_rows(label, rows, probe)


def erepo(census):
    groups = defaultdict(list)
    for page in range(1, 21):
        body, _ = census.native(EREPO, {"pgSize": 1000, "pg": page})
        rows = body["data"]
        for row in rows:
            caid = row.get("caId") or ""
            group = ("no-allele-id" if not re.fullmatch(r"CA\d+", caid) else
                     "with-document-id" if row.get("PCERDocID") else "no-document-id")
            groups[group].append(row)
        if len(rows) < 1000:
            break
    else:
        raise CensusError("evidence repository exceeds sampling window")

    def probe(row):
        caid = row.get("caId") or ""
        if not re.fullmatch(r"CA\d+", caid):
            return False
        reply = census.reply(("variant", "assertions", caid, "--source", "erepo"))
        for item in read(reply, "result", "assertions") or []:
            detail = item.get("statement") or item.get("unmapped") or {}
            attribution = detail.get("attribution") or {}
            if (attribution.get("record") == row.get("PCERDocID")
                    and attribution.get("version") == row["docVersion"]):
                return True
        return False

    for label, rows in sorted(groups.items()):
        yield label, sample_rows(label, rows, probe)


def recorded_bodies(census, source):
    for path in (census.home / source / source).glob("*.json"):
        capture = json.loads(path.read_text())
        for exchange in capture["exchanges"]:
            if exchange["status"] != 200:
                continue
            body = exchange["body"]
            content = body["value"]
            if body["kind"] == "sidecar":
                if content != Path(content).name:
                    raise CensusError("invalid bundle recording sidecar")
                with path.with_name(content).open("rb") as stream:
                    content = stream.read(MAX_SOURCE_BYTES + 1)
                if len(content) > MAX_SOURCE_BYTES:
                    raise CensusError("bundle recording exceeds the source capture bound")
            yield capture["url"], content


def wikipathways(census):
    census.reply(("sync", "wikipathways"), record="wikipathways")
    rows = defaultdict(list)
    for url, content in recorded_bodies(census, "wikipathways"):
        if not url.endswith("/findPathwaysByText.json"):
            continue
        catalog = content if isinstance(content, dict) else json.loads(content)
        for row in catalog["pathwayInfo"]:
            rows[row["species"]].append(row["id"])
    if not rows:
        raise CensusError("WikiPathways recording contains no catalog")

    def probe(identifier):
        reply = census.reply(("get", "pathway", identifier), allow_usage=True)
        return read(reply, "result", "pathway", "id", "id") == identifier

    for label, identifiers in sorted(rows.items()):
        yield label, sample_rows(label, identifiers, probe)


def protein_sample(census, query, label, probe):
    # UniProt uses cursors. Sample a bounded native page and retain the full native count.
    body, headers = census.native(UNIPROT, {"query": query, "fields": "accession", "size": 250})
    return sample_rows(label, body["results"], lambda row: probe(row["primaryAccession"]),
                       int(headers["x-total-results"]))


def uniprot(census):
    def probe(accession):
        items = read(census.reply(("get", "protein", accession)), "result", "items") or []
        return any(read(item, "header", "protein", "accession") == accession for item in items)

    for label, query in (
        ("reviewed-human", "reviewed:true AND organism_id:9606"),
        ("unreviewed-human", "reviewed:false AND organism_id:9606"),
        ("mouse", "organism_id:10090"),
        ("reviewed-human-no-gene", "reviewed:true AND organism_id:9606 AND NOT gene:*"),
    ):
        yield label, protein_sample(census, query, label, probe)


def interpro(census):
    for database in ("interpro", "unintegrated"):
        def probe(accession):
            native, _ = census.native(f"{INTERPRO}/entry/{database}/protein/uniprot/{accession}/",
                                      {"page_size": 1})
            total = native["count"] if native is not None else 0
            reply = census.reply(("protein", "domains", accession, "--limit", "1"))
            mapped = read(reply, "result", "native_total")
            if database == "interpro":
                return mapped == total or (total == 0 and reply["status"] == "complete")
            if total == 0:
                return True
            # A member signature is reachable only when the tool returns its own identity.
            identifier = native["results"][0]["metadata"]["accession"]
            return any(item.get("id") == identifier
                       for item in (read(reply, "result", "items") or []))

        yield database, protein_sample(census, "reviewed:true AND organism_id:9606",
                                       database, probe)


def opentargets(census):
    groups = defaultdict(list)
    for query in ("antibody", "inhibitor", "acetate", "insulin"):
        payload = json.dumps({"query": """query Drugs($text: String!) {
          search(queryString: $text, entityNames: ["drug"], page: {index: 0, size: 50}) {
            total hits { id }
          }
        }""", "variables": {"text": query}}).encode()
        native, _ = census.native(OPEN_TARGETS, {}, {"Content-Type": "application/json"}, payload)
        rows = native["data"]["search"]
        groups[query] = rows

    for label, frame in groups.items():
        def probe(row):
            identifier = row["id"]
            reply = census.reply(("get", "drug", identifier, "indications",
                                  "--source", "opentargets", "--section-limit", "1"))
            return read(reply, "result", "identity", "id") == identifier

        yield label, sample_rows(label, frame["hits"], probe, frame["total"])


def reactome(census):
    data, _ = census.native("https://reactome.org/download/current/ReactomePathways.txt",
                            {}, raw=True)
    groups = defaultdict(list)
    for line in data.decode().splitlines():
        identifier, _, species = line.split("\t")
        groups[species].append(identifier)

    def probe(identifier):
        reply = census.reply(("get", "pathway", identifier), allow_usage=True)
        return read(reply, "result", "pathway", "id", "id") == identifier

    for species, identifiers in sorted(groups.items()):
        yield species, sample_rows(species, identifiers, probe)


def clinpgx(census):
    base = "https://api.clinpgx.org/v1/data/summaryAnnotation"
    for level in ("1A", "1B", "2A", "2B", "3", "4"):
        body, _ = census.native(base, {"levelOfEvidence.term": level, "view": "min"})
        if body["status"] != "success":
            raise CensusError("ClinPGx did not return the level collection")
        rows = body["data"]
        groups = defaultdict(list)
        for row in rows:
            genes = read(row, "location", "genes") or []
            groups["with-gene" if genes else "without-gene"].append(row)

        def probe(row):
            identifier = row["accessionId"]
            reply = census.reply(("get", "pgx", "--annotation-id", identifier,
                                  "--source", "clinpgx"))
            return read(reply, "result", "header", "id") == identifier

        for group, records in sorted(groups.items()):
            label = f"clinical/{level}/{group}"
            yield label, sample_rows(label, records, probe)


def gnomad(census):
    groups = defaultdict(set)
    # Source totals here are for the selected native gene frame.
    for gene in ("ENSG00000141510", "ENSG00000157764", "ENSG00000146648",
                 "ENSG00000133703", "ENSG00000139618", "ENSG00000101986"):
        payload = json.dumps({"query": """query Variants($id: String!) {
          gene(gene_id: $id, reference_genome: GRCh38) {
            variants(dataset: gnomad_r4) { variant_id exome { ac } genome { ac } }
          }
        }""", "variables": {"id": gene}}).encode()
        body, _ = census.native("https://gnomad.broadinstitute.org/api", {},
                                {"Content-Type": "application/json"}, payload)
        for row in body["data"]["gene"]["variants"]:
            identifier = row["variant_id"]
            chromosome, _, reference, alternate = identifier.split("-")
            kind = "snv" if len(reference) == len(alternate) == 1 else "other-small-variant"
            dataset = ("joint" if row["exome"] and row["genome"] else
                       "exome-only" if row["exome"] else "genome-only")
            region = "sex-chromosome" if chromosome in {"X", "Y"} else "autosome"
            groups[f"{region}/{kind}/{dataset}"].add(identifier)

    def probe(identifier):
        reply = census.reply(("variant", "population", identifier))
        return read(reply, "result", "population", "data", "variant") == identifier

    for label, identifiers in sorted(groups.items()):
        yield label, sample_rows(label, sorted(identifiers), probe)


def orphadata(census):
    census.reply(("sync", "orphadata"), record="orphadata")
    groups = defaultdict(set)
    for _, content in recorded_bodies(census, "orphadata"):
        root = ElementTree.fromstring(content)
        for disorder in root.findall("./DisorderList/Disorder"):
            code = disorder.findtext("OrphaCode")
            kind = disorder.findtext("DisorderType/Name") or "unspecified"
            if disorder.find("./PrevalenceList/Prevalence") is not None:
                groups["prevalence/" + kind].add(code)
            if disorder.find("./AverageAgeOfOnsetList/AverageAgeOfOnset") is not None:
                groups["onset/" + kind].add(code)
    if not groups:
        raise CensusError("Orphadata recording contains no disorder classes")
    for label, codes in sorted(groups.items()):
        field = label.partition("/")[0]

        def probe(code):
            reply = census.reply(("get", "disease", f"ORPHA:{code}", "prevalence",
                                  "--source", "orphadata"), allow_usage=True)
            return any(item.get(field) for item in (read(reply, "result", "items") or []))

        yield label, sample_rows(label, sorted(codes), probe)
