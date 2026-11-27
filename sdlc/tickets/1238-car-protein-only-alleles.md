# 1238 — car-protein-only-alleles

Status: OPEN.

Milestone: 7

## Outcome

`variant normalize --hgvs <protein HGVS> --source car` returns the ClinGen Allele Registry's protein allele (a PA identifier) and the genomic alleles the registry links to it, labelled as a protein-level match.

## Why

A protein-only description, the form most reports use, fails today as a source error.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp variant normalize --hgvs 'NP_004324.2:p.Val600Glu' --source car` returns `status: failed`, problem `@id value.unknown`. The registry returns PA094029 for it.
- The registry's own refusals (alleles over 10,000 bp, uncertain HGVS) are passed on honestly.

## Proof

- Proof: an outside-in case on the recorded registry reply: the command returns PA094029 with its linked canonical alleles as candidates, none presented as the single resolved allele.

## Starts from

- Starts from: `crates/biomcp-adapters/src/allele_registry/raw.rs`.

## Keeps

- Keeps: genomic and transcript normalization are unchanged; a protein change is never collapsed to one nucleotide allele.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-adapters/src/allele_registry/`, variant cases.
