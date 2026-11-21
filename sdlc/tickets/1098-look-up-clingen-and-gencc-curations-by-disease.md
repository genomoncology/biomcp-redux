# 1098 — look-up-clingen-and-gencc-curations-by-disease

Status: COMPLETE.
Landed: 6aa7cfcb61af5a91e929395765fbc0d8a67df65d

Milestone: 7

## Outcome

Look up gene–disease curations by source disease name or ID across ClinGen and GenCC, with row filters for classification and submitter plus freshness and coverage for each source.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get disease MONDO:0019391 gene-validity --source clingen` returned `unknown section: error.option is not a section of this record; valid_values lists them`. Two export scans and a 79,988-byte gene verification batch were needed to establish coverage.

## Proof

- Proof: Replay both source exports with the same disease and overlapping genes and assert source, classification, inheritance and submitter are preserved. A stale copy names its date and the refresh action.

## Starts from

- Starts from: `crates/biomcp-query/src/gene/sections/clingen.rs`, `crates/biomcp-query/src/gene/sections/gencc.rs` and the existing stored bundles.

## Keeps

- Keeps: Gene-first sections and each source’s own assertions stay available.

## Defers

- Defers: Equating disease names from different sources without a checked ID.

## Changes

- Changes: Disease query, ClinGen and GenCC bundle lookups, help and cases.
