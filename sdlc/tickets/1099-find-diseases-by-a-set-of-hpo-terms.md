# 1099 — find-diseases-by-a-set-of-hpo-terms

Status: OPEN.

Milestone: 7

## Outcome

Return diseases whose positive annotations match every requested HPO term or a descendant in a named release, with original disease IDs, the matching terms and release gene links where supplied.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp search disease --phenotype HP:0001083 --phenotype HP:0000098 --phenotype HP:0001634` returned `deferred; no proved native HPO-ID disease filter`. The report had to intersect annotations and descendants outside BioMCP. Monarch association pages did not provide the requested release or coverage.

New source: Human Phenotype Ontology release annotations and gene links, https://github.com/obophenotype/human-phenotype-ontology/releases, Human Phenotype Ontology project

## Proof

- Proof: A saved release includes positive, negated, zero-frequency and excluded annotations, descendants and IDs in two namespaces. Assert intersection, original IDs, release, and the source gene links with missing links identified.

## Starts from

- Starts from: `crates/biomcp-adapters/src/hpo/` and `crates/biomcp-query/src/phenotype/`. Start with `hp.obo`, `phenotype.hpoa` and `genes_to_disease.txt` from one matching release.

## Keeps

- Keeps: Direct phenotype lookup and Monarch reads stay available.

## Defers

- Defers: Orphanet mapping relations and inferred gene links.

## Changes

- Changes: Disease search, HPO release reading, help and cases.
