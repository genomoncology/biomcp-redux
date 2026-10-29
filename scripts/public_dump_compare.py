"""Compare saved CLI replies to source identities using a temporary disk index."""
from collections import Counter
import json
import sqlite3

VARIANT_PREFIXES = ("clinvar:", "VCV", "CA", "PA", "rs", "COSV", "COSM", "COSN", "ga4gh:VA.")


def strings(value):
    if isinstance(value, dict):
        for child in value.values():
            yield from strings(child)
    elif isinstance(value, list):
        for child in value:
            yield from strings(child)
    elif isinstance(value, str):
        yield value


class Comparison:
    """Index source graph identities outside the repository and outside memory."""

    def __init__(self):
        self.db = sqlite3.connect("")
        self.db.executescript("""
            CREATE TABLE ids(node TEXT, id TEXT, PRIMARY KEY(node,id));
            CREATE TABLE edges(node TEXT, target TEXT, PRIMARY KEY(node,target));
        """)
        self.releases = {}
        self.counts = Counter()
        self.examples = []
        self.checked = Counter()

    def add(self, context, kind, release):
        source = "civic" if kind.startswith("civic_") else "clinvar" if kind.startswith("clinvar_") else "clingen"
        self.releases.setdefault(source, set()).add(release)
        if context:
            node, values, edges = context
            self.db.executemany("INSERT OR IGNORE INTO ids VALUES (?,?)", ((node, value) for value in values))
            self.db.executemany("INSERT OR IGNORE INTO edges VALUES (?,?)", ((node, target) for target in edges))

    def refuse(self, reason, identity):
        self.counts[reason] += 1
        if len(self.examples) < 10:
            self.examples.append({"reason": reason, "record": str(identity)[:512]})

    def allowed(self, record):
        root = "variation/" + record
        rows = self.db.execute("""
            WITH RECURSIVE reachable(node) AS (
                SELECT ? UNION SELECT edges.target FROM edges JOIN reachable ON edges.node=reachable.node
            ) SELECT DISTINCT id FROM ids JOIN reachable USING(node)
        """, (root,))
        return {row[0] for row in rows}

    def compare(self, path, genes):
        seen_genes = set()
        with path.open(encoding="utf-8") as handle:
            for number, line in enumerate(handle, 1):
                observation = json.loads(line)
                source = observation["source"]
                if observation["release"] not in self.releases.get(source, set()):
                    self.refuse("source_release_mismatch", number)
                    continue
                if not observation.get("revision") or not observation.get("retrieved"):
                    raise ValueError("observation_missing_revision_or_retrieval_date")
                reply = observation["reply"]
                if not isinstance(reply, dict) or "result" not in reply:
                    raise ValueError("observation_not_cli_envelope")
                if source == "civic":
                    self.civic(reply["result"], genes, seen_genes)
                elif source == "clinvar":
                    self.clinvar(reply["result"], observation["record"])
                else:
                    raise ValueError("observation_unsupported_source")
        for gene in genes.keys() - seen_genes:
            self.refuse("civic_gene_unobserved", gene)
        if "clinvar" in self.releases and not self.checked["clinvar_ids"]:
            self.refuse("clinvar_ids_unobserved", "clinvar")
        return {"checked": dict(self.checked), "failures": dict(self.counts), "examples": self.examples}

    def civic(self, result, genes, seen):
        for row in result.get("items", []):
            gene = row.get("answer", {}).get("gene", {}).get("symbol")
            section = row.get("sections", {}).get("civic", {})
            actual = section.get("data", {}).get("variant_count")
            if gene not in genes or actual is None:
                self.refuse("civic_gene_or_count_missing", gene)
                continue
            seen.add(gene)
            self.checked["civic_genes"] += 1
            if actual != genes[gene]:
                self.refuse("civic_gene_count_mismatch", f"{gene}: dump={genes[gene]} reply={actual}")

    def clinvar(self, result, record):
        allowed = self.allowed(record)
        if not allowed:
            self.refuse("clinvar_record_not_in_dump", record)
            return
        printed = [value for value in strings(result) if value.startswith(VARIANT_PREFIXES)]
        if not printed:
            self.refuse("clinvar_reply_has_no_variant_ids", record)
        for value in printed:
            self.checked["clinvar_ids"] += 1
            if value not in allowed:
                self.refuse("clinvar_identifier_spelling_mismatch", f"{record}: {value}")

    def close(self):
        self.db.close()
