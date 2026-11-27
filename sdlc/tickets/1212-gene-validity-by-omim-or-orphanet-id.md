# 1212 — gene-validity-by-omim-or-orphanet-id

Status: OPEN.

Milestone: 10

## Outcome

`get disease <id> gene-validity` returns the same ClinGen and GenCC curations whether the disease is named by its MONDO ID or by an OMIM or Orphanet ID that the disease card resolves to that MONDO ID, and GenCC rows submitted under an OMIM or Orphanet ID are matched.

## Why

A disease-first lookup by OMIM number reads as "no curated genes" today, though 71.8% of GenCC submissions were made under OMIM or Orphanet IDs.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get disease OMIM:182212 gene-validity` returns `status: complete`, `items: []`, with coverage `matched: 0` for both sources. `biomcp get disease MONDO:0008426 gene-validity` returns 11 items, and the disease card resolves OMIM:182212 to MONDO:0008426.
- Gene-first lookups are faithful: 100 random genes showed no mismatch with the ClinGen and GenCC downloads.
- Ticket 1098 defers disease name matching only.

## Proof

- Proof: an outside-in case on the recorded exports: OMIM:182212 and MONDO:0008426 return the same 11 items; an Orphanet ID with GenCC rows returns them.
- Proof: the ClinGen and GenCC census in the reach census job counts, by submitted disease namespace, the share of export rows the disease-first route returns.

## Starts from

- Starts from: `crates/biomcp-query/src/disease/validity.rs` and the GenCC and ClinGen export readers in `crates/biomcp-adapters/src/gencc/` and `crates/biomcp-adapters/src/clingen/`.

## Keeps

- Keeps: MONDO lookups are unchanged; no row is matched by disease name.

## Defers

- Defers: matching parent diseases to their children's curations (`MONDO:0007254`, breast cancer, returns no ClinGen rows); say so in the reply's coverage and file separately if a case needs the hierarchy.

## Changes

- Changes: `crates/biomcp-query/src/disease/validity.rs`, `crates/biomcp-adapters/src/gencc/`, disease cases and replies.
