# 1230 — clingen-actionability-route

Status: OPEN.

Milestone: 10

## Outcome

`get gene <symbol> clingen` reports ClinGen's clinical actionability curations for the gene (adult and pediatric context, outcome and intervention pairs with their scores and report link), or states that the section does not cover actionability and links ClinGen's report.

## Why

ClinGen publishes 463 adult and pediatric actionability curations and BioMCP has no route to any.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get gene FGF23 clingen gencc` says nothing about actionability; ClinGen's actionability summary holds curations for FGF23.
- Validity and dosage are faithful: 100 random genes showed no mismatch with ClinGen's downloads, and dosage regions matched by ID, band and coordinates.

## Proof

- Proof: an outside-in case on ClinGen's recorded actionability summary: FGF23 returns its curations with context and report link; a gene with none says none.

## Starts from

- Starts from: `crates/biomcp-adapters/src/clingen/` and ClinGen's actionability summary download.

## Keeps

- Keeps: validity and dosage replies are unchanged.

## Defers

- Defers: dosage scores and PMIDs that the dosage download omits for 180 of 1,720 genes, and region search by name (`search gene "16p11.2 recurrent region"` returns 0).

## Changes

- Changes: `crates/biomcp-adapters/src/clingen/`, `crates/biomcp-query/src/gene/`, schemas, gene cases.
