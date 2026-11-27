# 1221 — gnomad-dataset-scope-stated

Status: OPEN.

Milestone: 10

## Outcome

The population section states which gnomAD dataset and release it queried, and for a variant class that dataset does not hold (mitochondrial, structural, copy-number, tandem repeat) it reports the section `unavailable` with reason out of scope, or reads the gnomAD dataset that holds the class, instead of `empty`.

## Why

The dataset is fixed to gnomAD r4 in code, so a mitochondrial variant gnomAD publishes reads as "no variant record".

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get variant 'NC_012920.1:m.3243A>G' population` returns `status: complete` with the population section `empty`. gnomAD's r3 mitochondrial data holds m.3243A>G (allele number 56,383, six heteroplasmic carriers).
- gnomAD's r4 API answered "Variant not found" for the two mitochondrial sites tried, so the class is absent from the dataset asked, not from gnomAD.
- Structural variants and short tandem repeats have no route, and help does not say the dataset is `gnomad_r4`, GRCh38, release 4.1.0 only. Read from code for structural and copy-number variants.

## Proof

- Proof: outside-in cases on recorded replies: the m.3243A>G reply either returns the r3 mitochondrial record with its dataset named, or `unavailable` with an out-of-scope reason; the reply's `query` or section names the dataset and release; help states it.

## Starts from

- Starts from: `crates/biomcp-adapters/src/gnomad/variant.rs`, `crates/biomcp-query/src/variant/population_release.rs`.

## Keeps

- Keeps: r4 SNV and indel replies are unchanged.

## Defers

- Defers: structural-variant and tandem-repeat records as new routes; file when a case needs them.

## Changes

- Changes: `crates/biomcp-adapters/src/gnomad/`, `crates/biomcp-query/src/variant/population*.rs`, help text, cases.
