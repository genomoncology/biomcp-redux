"""Map public publisher fields to the existing BioData values, without inference."""
import csv
import json
import re

from public_dump_json import bundle_records


def item(kind, value):
    return {"kind": kind, "value": value}


def parts(value, delimiter=","):
    return [part.strip() for part in value.split(delimiter) if part.strip()]


def concept(kind, label, identity=None):
    value = {"label": label}
    if identity:
        value["id"] = identity
    return item(kind, value)


def civic(kind, row):
    values = []
    if kind == "civic_variants":
        values += [item("identifier", "civic:" + row["variant_id"]), item("text", row["variant"])]
        if row["gene"]:
            values += [item("gene", gene) for gene in parts(row["gene"])]
        values += [item("hgvs", hgvs) for hgvs in parts(row["hgvs_descriptions"])]
        if row.get("representative_transcript"):
            values.append(item("accession", row["representative_transcript"]))
    elif kind == "civic_profiles":
        values.append(item("profile", {"level": "profile", "record": "civic:" + row["molecular_profile_id"],
                                       "label": row["molecular_profile"], "members": []}))
        values += [item("identifier", "civic:" + identity) for identity in parts(row["variant_ids"])]
    else:
        values.append(item("identifier", "civic.evidence:" + row["evidence_id"]))
        values.append(item("identifier", "civic:" + row["molecular_profile_id"]))
        if row.get("disease"):
            doid = row.get("doid", "")
            values.append(concept("disease", row["disease"], "DOID:" + doid if doid else None))
        for drug in parts(row.get("therapies", "")):
            values.append(concept("drug", drug))
        if row.get("citation_id"):
            citation_type = {"PubMed": "citation", "DOI": "doi"}.get(row["source_type"], "text")
            values.append(item(citation_type, row["citation_id"]))
        for field in ("evidence_statement", "citation"):
            if row.get(field):
                values.append(item("text", row[field]))
    return values


def clinvar(row):
    values = [item("clinvar_allele", row["#AlleleID"]),
              item("clinvar_variation", "clinvar:" + row["VariationID"])]
    for gene in parts(row.get("GeneSymbol", ""), ";"):
        if gene != "-":
            values.append(item("gene", gene))
    for gene in parts(row.get("GeneID", ""), ";"):
        if gene != "-1":
            values.append(item("gene_id", gene))
    for accession in parts(row.get("RCVaccession", ""), ";"):
        if accession != "-":
            values.append(item("rcv", accession))
    rs = row.get("RS# (dbSNP)", "")
    if rs and rs != "-1":
        values.append(item("rs", "rs" + rs))
    # The preferred title may append a protein description after the transcript HGVS.
    name = row.get("Name", "")
    if ":" in name and re.search(r":[gcnrpmo]\.", name):
        values.append(item("hgvs", name.split(" ", 1)[0]))
    elif name and name != "-":
        values.append(item("text", name))
    for identity in parts(row.get("PhenotypeIDS", ""), "|"):
        if identity != "-":
            values.append(item("identifier", identity))
    for label in parts(row.get("PhenotypeList", ""), "|"):
        if label != "-":
            values.append(concept("disease", label))
    return values


def clingen(row):
    values = [item("gene", row["HGNC Gene Symbol"]),
              concept("disease", row["Disease"], row["Mondo Id"] or None)]
    if row.get("Allele Registry Id"):
        values.append(item("ca", row["Allele Registry Id"]))
    if row.get("ClinVar Variation Id"):
        values.append(item("clinvar_variation", "clinvar:" + row["ClinVar Variation Id"]))
    values += [item("hgvs", hgvs) for hgvs in re.split(r"[,;]\s*", row["HGVS Expressions"]) if hgvs]
    if row.get("Variation"):
        values.append(item("text", row["Variation"]))
    return values


REQUIRED = {
    "civic_variants": {"variant_id", "variant", "gene", "hgvs_descriptions"},
    "civic_profiles": {"molecular_profile_id", "molecular_profile", "variant_ids"},
    "civic_evidence": {"evidence_id", "molecular_profile_id", "source_type", "citation_id"},
    "clinvar_summary": {"#AlleleID", "VariationID", "Name"},
    "clingen_summary": {"HGNC Gene Symbol", "Disease", "Mondo Id", "HGVS Expressions"},
}


def table_records(handle, kind):
    reader = csv.DictReader(handle, delimiter="\t")
    missing = REQUIRED[kind] - set(reader.fieldnames or [])
    if missing:
        raise ValueError("missing_columns: " + ",".join(sorted(missing)))
    for index, row in enumerate(reader, 1):
        if None in row or any(value is None for value in row.values()):
            raise ValueError(f"malformed_tsv_record: {index}")
        if kind.startswith("civic_"):
            values = civic(kind, row)
        elif kind == "clinvar_summary":
            values = clinvar(row)
        else:
            values = clingen(row)
        genes = parts(row.get("gene", "")) if kind == "civic_variants" else []
        yield index, values, genes, None


SECTIONS = {
    "sequenceReference", "location", "allele", "copyNumberCount", "copyNumberChange",
    "gene", "variation", "condition", "conditionSet", "therapy", "therapyGroup", "submitter",
    "propositionPathogenicity", "propositionOncogenicity", "propositionSomaticClinicalImpact",
    "propositionClinvarCustom", "evidenceLine", "scv", "vcv", "rcv",
}


def identity_value(value):
    for prefix, kind in (("clinvar:", "clinvar_variation"), ("VCV", "vcv"), ("RCV", "rcv"),
                         ("SCV", "scv"), ("rs", "rs"), ("CA", "ca"), ("ga4gh:VA.", "variant"),
                         ("MONDO:", "mondo"), ("NCIT:", "ncit")):
        if value.startswith(prefix):
            return item(kind, value)
    if value.startswith(("pmid:", "PMID:")):
        return item("citation", value.split(":", 1)[1])
    if ":" in value and not value.startswith(("https:", "http:", "#/")):
        return item("identifier", value)
    return None


def nested_values(record):
    """Visit only publisher identity, expression, label and citation fields."""
    if isinstance(record, list):
        for entry in record:
            yield from nested_values(entry)
    elif isinstance(record, dict):
        syntax = record.get("syntax", "")
        for key, value in record.items():
            if isinstance(value, (list, dict)):
                yield from nested_values(value)
            elif isinstance(value, str):
                if key == "value" and syntax.startswith("hgvs"):
                    yield item("hgvs", value)
                elif key in {"id", "primaryCode", "code", "accession", "identifier"}:
                    parsed = identity_value(value)
                    if parsed:
                        yield parsed
                    elif value:
                        yield item("text", value)
                elif key in {"name", "label", "title"}:
                    yield item("text", value)


def records(handle, kind):
    if kind == "clingen_evidence":
        record = json.load(handle)
        if not isinstance(record, dict) or not isinstance(record.get("evidenceLine"), list):
            raise ValueError("clingen_missing_evidence_tree")
        values = list(nested_values(record))
        condition = record.get("condition", {})
        if condition.get("label"):
            values.append(concept("disease", condition["label"], condition.get("id")))
        values.extend(criterion_citations(record))
        yield record.get("uuid", "interpretation"), values, [], None
        return
    if kind != "clinvar_gks":
        yield from table_records(handle, kind)
        return
    for section, identity, record in bundle_records(handle):
        values = list(nested_values(record))
        primary = identity_value(identity)
        if primary and record.get("id") != identity:
            values.insert(0, primary)
        if section in {"condition", "therapy"} and record.get("name"):
            values.append(concept("disease" if section == "condition" else "drug", record["name"]))
        yield f"{section}/{identity}", values, [], None if section in SECTIONS else section


def criterion_citations(node, criterion=False):
    """Only explicit PubMed references within source criterion assessments."""
    if isinstance(node, dict):
        criterion = criterion or node.get("type") == "CriterionAssessment"
        for value in node.values():
            yield from criterion_citations(value, criterion)
    elif isinstance(node, list):
        for value in node:
            yield from criterion_citations(value, criterion)
    elif criterion and isinstance(node, str):
        for match in re.finditer(r"(?:pubmed\.ncbi\.nlm\.nih\.gov/|/pubmed/)([0-9]+)", node):
            yield item("citation", match[1])
        for match in re.finditer(r"\bPMIDs?\s*:?\s*([0-9]+(?:\s*[,;]\s*[0-9]+)*)", node, re.I):
            for pmid in re.findall(r"[0-9]+", match[1]):
                yield item("citation", pmid)
