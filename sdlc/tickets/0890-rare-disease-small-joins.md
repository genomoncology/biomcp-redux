# 0890 — rare-disease-small-joins

Status: OPEN.

Milestone: 7

## Outcome

Three small joins the rare-disease agent spent calls on work in one: disease hierarchy rows carry parent labels; disease synonyms are not repeated; a `clinvar:` variant's population section uses the allele it already resolved.

## Why

In the rare-disease persona run of 2026-11-13 (biomed-agent-bench `personas/rare-disease/runs/2026-11-13/log.md`, analysis beside it; code-mode run in `runs/2026-11-13-raw/`): `get disease MONDO:0016512 hierarchy` gave bare parent IDs and the agent made 3 more calls to name them (log.md:212-218); synonyms came back 7 with repeats (log.md:210); `get variant clinvar:5020581 population` said `absent` while the same allele by ClinGen allele ID had gnomAD counts (log.md:355).

## Proof

- Proof: recorded cases for each: parent labels from the recorded source reply, distinct synonyms, and the clinvar ID's population matching its allele ID's.

## Starts from

- Starts from: `crates/biomcp-adapters/src/mydisease/`, `crates/biomcp-query/src/variant/`.

## Keeps

- Keeps: source attribution per row.

## Defers

- Defers: transitive HPO ancestors.

## Changes

- Changes: `crates/biomcp-adapters/src/mydisease/`, `crates/biomcp-query/`, `tests/cases/`, `docs/`

## Progress

- 2026-11-13, main db0a736b3, live job j39133: all three gaps reproduce. `get disease MONDO:0016512 synonyms` returned 7 rows; "KMS" and "Niikawa-Kuroki syndrome" appear once from MONDO and again from Disease Ontology. `hierarchy` returned 4 rows with bare object IDs. `get variant clinvar:5020581 population` returned `absent` from MyVariant (MyVariant has no row for this 2026 ClinVar record), while `CA59804330 population` returned gnomAD r4 exome AC 1 / AN 1,429,918.
- Source checks: MyDisease `POST /v1/disease` with `ids` and `fields=mondo.mondo,mondo.label,disease_ontology.doid,disease_ontology.name` names each parent in one call; a DOID query can return the MONDO document, so a label is kept only when the parent's own component ID matches. ClinGen Allele Registry `GET /alleles?ClinVar.variationId=5020581` returns CA59804330, whose `externalRecords.ClinVarVariations` states 5020581.
- Changed: synonyms keep one row per distinct text (case-insensitive), first component and scope kept, so MONDO wins over Disease Ontology. Hierarchy reads the page's parent labels in one MyDisease batch; a parent without a self-stated label keeps its bare ID, and a failed lookup adds a problem but keeps the rows. A `clinvar:` ID that MyVariant does not resolve now falls back to the registry's ClinVar cross-reference; rows that do not state that variation are dropped, the card names `car` as its resolver, and population joins through the resolved allele as for a CAid.
