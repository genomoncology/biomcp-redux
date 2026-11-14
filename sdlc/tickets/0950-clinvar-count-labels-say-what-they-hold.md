# 0950 — clinvar-count-labels-say-what-they-hold

Status: OPEN.

Milestone: 7

## Outcome

The ClinVar gene section's field descriptions say what each bucket holds: "records whose aggregate label includes X", so combined labels ("Pathogenic/Pathogenic, low penetrance; risk factor", "Conflicting…; risk factor", bare VUS-high) land where the description says. A test pins that a 429 echoing the key in a header is still refused.

## Why

0932's review (2026-11-14): `crates/biomcp-types/src/response/gene/clinvar.rs:31,37,47` say "Pathogenic." and "risk factor" while combined labels land elsewhere; `request.rs:353`'s comment says NCBI though the 429 exception is global; the leftover parallel `Job`/`Done` dispatch in `clinvar/gene.rs:110-148,192-210` can be plain calls.

## Proof

- Proof: descriptions updated and regenerated; a transport test for a 429 with the key in a header; a unit or case check for one combined label.

## Starts from

- Starts from: the review lines above.

## Keeps

- Keeps: counts and buckets.

## Defers

- Defers: nothing.

## Changes

- Changes: descriptions, a comment, a small refactor, one test.
