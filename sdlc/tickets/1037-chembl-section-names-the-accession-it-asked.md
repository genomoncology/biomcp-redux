# 1037 — chembl-section-names-the-accession-it-asked

Status: OPEN.

Milestone: 7

## Outcome

`get protein O75417-2 chembl` says in the reply that ChEMBL was asked about the canonical accession O75417, both when a target is found and when the section is empty.

## Why

1030's review (2026-11-17): the isoform is answered with the canonical protein's target, but only `docs/cautions.md:202` says so. The empty-section record says "asked UniProt O75417-2" (`crates/biomcp-query/src/protein/source_sections.rs:72`, `:88`) though ChEMBL was asked about O75417.

## Proof

- Proof: the 1030 isoform replay step asserts the canonical accession in the section; it fails on the base commit.

## Starts from

- Starts from: `crates/biomcp-adapters/src/chembl/target.rs:78-81` and the section record.

## Keeps

- Keeps: counts and mapping.

## Defers

- Defers: nothing.

## Changes

- Changes: one optional field or the empty record's wording, the schema, the case.
