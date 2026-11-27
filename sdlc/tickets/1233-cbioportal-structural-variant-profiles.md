# 1233 — cbioportal-structural-variant-profiles

Status: OPEN.

Milestone: 10

## Outcome

`get study <study>` exposes a study's structural-variant profile as a section (fusions and rearrangements by gene), and the `mutations` section states which sample list it used.

## Why

226 of cBioPortal's 2,573 molecular profiles (8.8%) are structural-variant profiles, the only cBioPortal record of fusions, and none is reachable.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get study luad_tcga_pan_can_atlas_2018 mutations --gene EML4 --molecular-profile luad_tcga_pan_can_atlas_2018_structural_variants` is refused ("reads only a mutation profile of this study"), and no other section reads that profile. cBioPortal holds four EML4 rows in it.
- `get study <study> mutations` picks the all-cases sample list without stating it in help or the reply, while `variant recurrence` requires the caller to name one.
- 55% of profiles are neither mutation nor discrete copy-number (expression, protein, methylation); those are out of scope here. Mutation and copy-number counts matched cBioPortal (EGFR 86 rows; BRAF V600E 295 samples).

## Proof

- Proof: outside-in cases on recorded cBioPortal replies: the lung study returns its EML4 structural-variant rows by gene; the mutations reply echoes its sample list.

## Starts from

- Starts from: `crates/biomcp-adapters/src/cbioportal/`, `crates/biomcp-query/src/study/`.

## Keeps

- Keeps: mutation and copy-number sections are unchanged.

## Defers

- Defers: expression, protein and methylation profiles, and recurrence for pure insertions and deletions (help states that limit).

## Changes

- Changes: `crates/biomcp-adapters/src/cbioportal/`, `crates/biomcp-query/src/study/`, schemas, study cases.
