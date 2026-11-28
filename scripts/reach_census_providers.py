"""Native sample frames for the remaining live reach providers."""
from collections import defaultdict
import json
import re
from urllib.error import HTTPError

from gene_validity_reach import probes as validity_probes
from reach_census_common import Measurement, read, sample_rows, source_page

CTGOV = "https://clinicaltrials.gov/api/v2"
CPIC = "https://api.cpicpgx.org/v1"
EREPO = "https://erepo.clinicalgenome.org/evrepo/api/summary/classifications"
WP = "https://www.wikipathways.org/json/findPathwaysByText.json"
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
            reply = census.reply(("search", "pgx", "--gene", gene, "--source", "cpic",
                                  "--drug-id", row["drugid"], "--limit", "50"))
            return any(item["pairid"] == row["pairid"]
                       for item in (read(reply, "result", "items") or []))

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
            raise ValueError("validity sample exceeds paging window")

        label = f"{source}/{namespace}"
        yield label, sample_rows(label, rows, probe)


def erepo(census):
    groups = defaultdict(list)
    for page in range(1, 21):
        body, _ = census.native(EREPO, {"pgSize": 1000, "pg": page})
        rows = body["data"]
        for row in rows:
            group = ("no-allele-id" if not row.get("caId") else
                     "with-document-id" if row.get("PCERDocID") else "no-document-id")
            groups[group].append(row)
        if len(rows) < 1000:
            break
    else:
        raise ValueError("evidence repository exceeds sampling window")

    def probe(row):
        if not row.get("caId"):
            return False
        reply = census.reply(("variant", "assertions", row["caId"], "--source", "erepo"))
        return any(item.get("record") == row.get("PCERDocID")
                   and item.get("version") == row["docVersion"]
                   for item in (read(reply, "result", "assertions") or []))

    for label, rows in sorted(groups.items()):
        yield label, sample_rows(label, rows, probe)


def wikipathways(census):
    catalog, _ = census.native(WP, {})
    rows = defaultdict(list)
    for row in catalog["result"]:
        rows[row["species"]].append(row["id"])
    census.reply(("sync", "wikipathways"))

    def probe(identifier):
        reply = census.reply(("get", "pathway", identifier))
        return read(reply, "result", "header", "id") == identifier

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
            return total == (read(reply, "result", "native_total") or 0)

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
