# 1220 — litvar-variant-without-rsid

Status: OPEN.

Milestone: 7

## Outcome

`search article --source litvar --gene <gene> --variant <change>` returns LitVar's papers when LitVar resolves the change to one of its variant records, and when BioMCP could not pick a LitVar variant it returns the candidates it found and is not `complete` with no rows.

## Why

A LitVar search for one of the commonest EGFR alterations reads as "no literature" today.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp search article --source litvar --gene EGFR --variant p.E746_A750del` returns `status: complete`, `items: []`. LitVar holds 76 papers under rs121913421 for this change.
- `--variant c.2235_2249del` lists that rsID among its candidates and still returns no rows.
- Control: BRAF V600E returns 32,344 with truncation stated.

## Proof

- Proof: an outside-in case on recorded LitVar replies: the EGFR deletion returns its papers, or the candidate variant list with a `next` step for each, never zero rows with `complete`.

## Starts from

- Starts from: `crates/biomcp-adapters/src/litvar/mod.rs`, `crates/biomcp-adapters/src/litvar/raw.rs`.

## Keeps

- Keeps: rsID lookups and their totals; no paper is attributed to a variant LitVar did not link.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-adapters/src/litvar/`, article cases and replies.
