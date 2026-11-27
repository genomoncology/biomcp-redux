# 1208 — complete-status-when-a-section-could-not-look

Status: OPEN.

Milestone: 7

## Outcome

A reply's top-level `status` is `complete` only when every section the caller asked for was actually looked up. A section that was skipped for a missing key, that lacked the ID needed to ask its source, or whose source was disabled reports why, and the reply is `partial`.

## Why

The census found the CIViC failure shape, "could not look" reported as "nothing there", in four more places; an agent that reads `status` alone treats each as a clean negative.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case 1: `biomcp get variant CA123643 oncokb` returns `status: complete` with the section `skipped`, reason `no-key`. A source the caller disabled gives `partial`; a source BioMCP could not reach for want of a key gives `complete`.
- Failing case 2: `biomcp get drug Keytruda markets --disable-sources drugsfda` returns the United States row as `no-record`, `matches: 0`, the same words used when Drugs@FDA was asked and held nothing.
- Failing case 3: `biomcp get variant rs1801133 population` returns top status `complete` while its population section is `unavailable`.
- Failing case 4: Orphanet prevalence for a disease with no Orphanet link reports `absent`; the only sign that nothing was asked is a source reason `no-join-id`.
- The variant-to-GWAS route already does this well: an ambiguous rsID reports `unavailable` with `no-join-id`.

## Proof

- Proof: outside-in cases for the four witnesses: each reply is `partial`, names the section that was not looked up and gives its reason; the markets row says the source was disabled, not `no-record`.
- Proof: a contract test over every entity's status computation: a section in state `skipped` or `unavailable`, for any reason other than "not applicable to this record", never yields `complete`.

## Starts from

- Starts from: `crates/biomcp/src/incomplete.rs`, `crates/biomcp-query/src/status.rs`, `crates/biomcp-query/src/drug/markets/read.rs`, `crates/biomcp-query/src/disease/orphadata.rs`.

## Keeps

- Keeps: `not-applicable` sections (a non-coding gene's protein section) do not downgrade the reply; a truly empty source answer stays `complete`.

## Defers

- Defers: the pathway route for genes with no protein ID, which ticket 1216 fixes at its cause.

## Changes

- Changes: `crates/biomcp/src/incomplete.rs`, `crates/biomcp-query/src/status.rs`, `crates/biomcp-query/src/drug/markets/`, `crates/biomcp-query/src/disease/orphadata.rs`, contract docs and cases.
