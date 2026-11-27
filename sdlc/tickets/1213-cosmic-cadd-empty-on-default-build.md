# 1213 — cosmic-cadd-empty-on-default-build

Status: COMPLETE.
Landed: 7b3636f43213efe84150717819198bd8dbc7f640

Milestone: 10

## Outcome

The `cosmic` and `conservation`/`predict` sections of `get variant` return MyVariant.info's COSMIC and CADD fields for a variant addressed on GRCh38, by reading the index that carries them, or report the section `unavailable` with the reason; and a gene-and-protein shorthand such as `"BRAF V600E"` uses the assembly help states.

## Why

MyVariant's GRCh38 index carries neither COSMIC nor CADD, so the default build returns `empty` for the best-known somatic variants.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get variant CA123643 cosmic` (BRAF V600E) returns `status: complete`, `cosmic.state: empty`. `biomcp get variant 'chr7:g.140453136A>T' cosmic --build GRCh37` returns COSM476. MyVariant's hg19 record has COSM476 and a CADD score of 32; its hg38 record has neither field.
- `biomcp get variant "BRAF V600E"` returns GRCh37 candidates although help says the identity assembly defaults to GRCh38; with `--build GRCh38` the candidates differ.
- In a sample of 207 ClinVar-carrying hg38 records, 47% had dbNSFP and none of the non-SNVs did.

## Proof

- Proof: outside-in cases on recorded replies: CA123643 `cosmic` returns COSM476 or `unavailable` with reason; the shorthand returns GRCh38 candidates by default, and the reply's `query` echoes the assembly used.

## Starts from

- Starts from: `crates/biomcp-adapters/src/myvariant/cosmic.rs`, the MyVariant request builders in `crates/biomcp-adapters/src/myvariant/`, and the shorthand resolution in `crates/biomcp-query/src/variant/`.

## Keeps

- Keeps: GRCh37 lookups with `--build GRCh37` are unchanged; no field is lifted between assemblies without the allele's own liftover.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-adapters/src/myvariant/`, `crates/biomcp-query/src/variant/`, help text, variant cases.
