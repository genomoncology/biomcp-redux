# 1037 — chembl-section-names-the-accession-it-asked

Status: COMPLETE.
Landed: b2acd1fc6e6a89e9dca0e224ac4468029abbe4d5

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

## Progress

- Change: the ChEMBL section's data now carries an optional `accession`, the canonical UniProt accession ChEMBL was asked about, and the empty section's record names that canonical accession instead of the isoform the user typed. One adapter function `chembl::canonical` serves both. `docs/cautions.md` says so. The only schema change is the added optional field.
- j42263 (55525834): live `get protein O75417-2 chembl` replied `accession` O75417 with CHEMBL6025 (46 assays, 693 activities, counts unchanged); live `get protein Q8N726-2 chembl` replied empty with record UniProt Q8N726. Its UniProt reply is now recorded (provenance updated); its ChEMBL lookup is the same request as Q8N726's and matched the recorded reply. The case's O75417-2 step asserts `data/accession` O75417, and a new Q8N726-2 step asserts the empty record names Q8N726. check-cheap, types tests, schema-release-check and the protein and mcp cases passed.
- Gate j42267 on 16f8fed7 (`make --keep-going landing-gate docs-check schema-release-check`): passed. Main since gained only the 1038 ticket file.
