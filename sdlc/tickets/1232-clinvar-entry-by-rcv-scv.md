# 1232 — clinvar-entry-by-rcv-scv

Status: OPEN.

Milestone: 7

## Outcome

`get variant` accepts a ClinVar RCV or SCV accession and returns the variation record that holds it, and honours or refuses a requested VCV version instead of ignoring it.

## Why

Papers and reports cite RCV and SCV accessions; today both are refused, and large copy-number and range variants can be reached only by a VCV number nothing in BioMCP can find.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get variant RCV000014992` and `biomcp get variant SCV000061601` are refused as usage errors.
- `biomcp get variant VCV000013961.100 clinvar` returns version 143 with 88 rows; ClinVar's version 100 has 39 submissions.
- About 1.9% of a random sample of 1,123 ClinVar records (large and range variants) have no small-variant HGVS spelling; HGVS and region input both fail as `rejected` for them. `search variant` reads MyVariant only, and `get gene <symbol> clinvar` gives counts only, so no route lists a gene's ClinVar records.
- Reach by `clinvar:N` or VCV is otherwise about 94.7% of 4,714,028 records, and totals match ClinVar.

## Proof

- Proof: outside-in cases on recorded ClinVar replies: the RCV and SCV accessions return variation 13961's card with that accession's row marked; a versioned VCV returns that version or says only the current one is served.

## Starts from

- Starts from: `crates/biomcp-adapters/src/clinvar/`, the variant input grammar in `crates/biomcp-query/src/variant/`.

## Keeps

- Keeps: VCV and `clinvar:N` input are unchanged.

## Defers

- Defers: a list route for a gene's ClinVar records by classification or review status, which is a new command; file after the PM decides its shape.

## Changes

- Changes: `crates/biomcp-adapters/src/clinvar/`, `crates/biomcp-query/src/variant/`, grammar and help, cases.
