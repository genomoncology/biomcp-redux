# 1240 — standing-offline-reach-check

Status: OPEN.

Milestone: 10

Depends on: 1208

## Outcome

`make landing-gate` runs an offline reach check: small pinned slices of each open source's own release replay through the real mappers, and the check fails when any record class returns `complete` with zero rows or when a class's reached share falls below its recorded floor.

## Why

The source reach census found record classes with no route in most sources, and every gap sat in a class with no fixture. Tests used famous records only.

## Evidence

Full tables and the proposed check: [source reach census](../records/2026-11-27-source-reach-census.md), "Proposed standing check".

- Sources in the first cut: ClinVar, the ClinGen Evidence Repository, ClinGen validity and dosage, GenCC, WikiPathways, Reactome, CPIC, cBioPortal, the ClinicalTrials.gov value lists, the two shipped lists, and CIViC once its reach tickets land.

## Proof

- Proof: the check passes on main after the tickets it covers land; a mutation that drops one record class (for example ClinVar records without a classification) makes it fail with the class named. Runtime 60 seconds or less, no network, a few megabytes of fixtures.

## Starts from

- Starts from: the census helpers' slices, the outside-in runner, the CIViC census test from "Reach CIViC profiles without a ClinGen allele ID".

## Keeps

- Keeps: no network in the gate; no licensed content in the repo.

## Defers

- Defers: FDA sources, which are sampled only through BioMCP in the weekly live census (1241).

## Changes

- Changes: `tests/`, fixtures, `Makefile` landing-gate stage.
