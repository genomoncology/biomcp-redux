# 1229 — interpro-domains-fail-on-valid-proteins

Status: OPEN.

Milestone: 7

## Outcome

`protein domains <accession>` returns InterPro's entries for a protein with many entries, returns `empty` for a protein InterPro has none for, and states how many unintegrated member-database signatures it left out.

## Why

A valid protein with 26 InterPro entries fails today as a source error.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case 1: `biomcp protein domains P27708` returns `status: failed`, source reason `bad-reply`, problem `reply value.unknown`. InterPro holds 26 entries for P27708. The cause is not proven; the raw reader's 25-row limit is the suspect.
- Failing case 2: `biomcp protein domains Q05BU3` returns `failed`; InterPro answers 204 (no entries).
- Read from code: unintegrated signatures are rejected. InterPro holds 41,894 of them; KRAS has seven against four integrated entries.

## Proof

- Proof: outside-in cases on recorded InterPro replies: P27708 returns 26 entries across pages; Q05BU3 returns `empty`; KRAS states its count of unintegrated signatures.

## Starts from

- Starts from: `crates/biomcp-adapters/src/interpro/raw.rs`.

## Keeps

- Keeps: proteins that work today return the same rows.

## Defers

- Defers: returning the unintegrated signatures themselves.

## Changes

- Changes: `crates/biomcp-adapters/src/interpro/`, protein cases.
