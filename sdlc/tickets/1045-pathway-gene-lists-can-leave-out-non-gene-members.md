# 1045 — pathway-gene-lists-can-leave-out-non-gene-members

Status: OPEN.

Milestone: 7

## Outcome

`get pathway R-HSA-… genes` can return gene products only: an option keeps the rows whose reference is a gene product (Reactome's UniProt, and Ensembl or NCBI Gene where present) and leaves out small molecules (ChEBI) and other non-gene references. The reply says how many rows it left out and of which namespaces, and its total counts the kept rows. Without the option the reply is unchanged.

## Why

The comp-bio persona built gene sets from pathway "genes" replies in its 2026-11-10 and 2026-11-12 runs, and small molecules came along: 6 of 75 rows in one pathway were ChEBI compounds (biomed-agent-bench `personas/rollup.md`, comp-bio; `personas/comp-bio/runs/2026-11-12/log.md:102`; `records/2026-11-12-persona-review-findings.md`, Waiting). A gene set with compounds in it gives wrong enrichment and overlap answers. Ian's `decisions/2026-10-27-1-0-ships-every-known-fix.md` puts every known gap in 1.0.

## Proof

- Proof: a replayed case reads the genes of a Reactome pathway with ChEBI participants using the option and expects no ChEBI row, the left-out count by namespace and a total that matches the kept rows. It fails on the base commit.

## Starts from

- Starts from: `map_members` in `crates/biomcp-adapters/src/reactome/relations.rs`, which already splits each reference's `st_id` into namespace and identifier, and `PathwayGenesQuery` in `crates/biomcp-types/src/query/pathway/relation.rs`. WikiPathways already reads GeneProduct nodes only; check that and say so in the help.

## Keeps

- Keeps: the default genes reply, its paging and pool, and the details layer.

## Defers

- Defers: nothing.

## Changes

- Changes: the pathway genes query and Reactome mapping, a case and its recording, help and one release-notes line.

## Progress

- 2026-11-17, measured live: Reactome participants name references as `namespace:id` in lower case. R-HSA-5693607 holds 89 distinct references, 80 `uniprot` and 9 `chebi`. Larger pathways also carry `ensembl` (ReferenceDNASequence and ReferenceRNASequence), `mirbase`, `ena.embl`, `nucleotide`, `iuphar.ligand` and `pubchem.compound`. No `ncbigene` reference was seen; it is kept where present.
- Built: `pathway genes ID --gene-products` (`gene_products: true` in MCP). It keeps `uniprot`, `ensembl` and `ncbigene` rows and leaves out every other namespace from the retained relation before paging, so `total`, `page.total` and `next_offset` count kept rows. The new optional `left_out` gives `rows` and per-namespace counts in first-occurrence order. Next-page and details steps keep the option. The pool is shared with the plain read, so the default reply is unchanged and carries no `left_out`.
- WikiPathways: the GPML adapter keeps only `GeneProduct` DataNodes (`crates/biomcp-adapters/src/wikipathways/gpml.rs`, `member`), so the option leaves nothing out there and reports `left_out.rows` 0. The help says so.
- Case `tests/cases/pathway/gene-products.yaml` replays R-HSA-5693607 with and without the option and pages to the end.
