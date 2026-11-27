# 1216 — pathways-for-genes-without-protein-id

Status: OPEN.

Milestone: 10

## Outcome

`search pathway --gene <symbol>` and `get gene <symbol> pathways` return Reactome's pathways for a gene Reactome maps without a UniProt accession (a microRNA or other non-coding gene), `search pathway --gene` accepts a UniProt accession, and when no Reactome lookup was made the reply says so and is not `complete` with zero rows.

## Why

Reactome maps 70 human microRNAs to 27 pathways; today each reads as "no pathways", with Reactome marked `ok` though it was never asked.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case 1: `biomcp search pathway --gene MIR21` returns `status: complete`, `page.total: 0`, sources `hgnc ok`, `reactome ok`. `biomcp get gene MIR21 pathways` says `not-applicable`. Reactome maps miR-21 to R-HSA-8943723 and R-HSA-9725371.
- Failing case 2: `biomcp search pathway --gene P15056` returns `complete` with 0 rows and a problem `gene value.unknown`; Reactome's own mapping returns 16 pathways for P15056.
- Cause, read from code: the lookup runs over an empty protein-ID list, and a source with zero calls is marked `ok`. Ticket 0028 shipped `not-applicable` on purpose and its test asserts it.

## Proof

- Proof: outside-in cases on recorded Reactome replies: MIR21 returns its two pathways on both routes; `--gene P15056` returns BRAF's pathways; a gene Reactome maps by no identifier returns `empty` only after Reactome was asked.

## Starts from

- Starts from: `crates/biomcp-query/src/pathway/gene.rs`, `crates/biomcp-query/src/pathway/search.rs`, `crates/biomcp-query/src/status.rs`, and Reactome's miRBase and Ensembl mapping files.

## Keeps

- Keeps: protein-coding gene lookups return the same pathways and totals.

## Defers

- Defers: search by ChEBI compound (`search pathway "ChEBI:15377"` returns 0 where Reactome lists 544 pathways) and `get pathway` on a Reaction ID; file when a case needs them.

## Changes

- Changes: `crates/biomcp-query/src/pathway/`, `crates/biomcp-adapters/src/reactome/`, pathway and gene cases.
