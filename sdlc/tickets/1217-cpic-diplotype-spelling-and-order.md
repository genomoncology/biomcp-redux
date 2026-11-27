# 1217 — cpic-diplotype-spelling-and-order

Status: OPEN.

Milestone: 10

## Outcome

`get pgx --gene <gene> diplotype --diplotype <text>` finds a CPIC diplotype whichever order the two alleles are given in and whatever letter case the gene, allele or level is typed in, and when no row matches it says the spelling was not found and offers CPIC's spellings for that gene instead of returning `complete` with nothing.

## Why

CPIC's 112,820 diplotype rows are reachable only by an exact string today, and help's own example misses.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get pgx --gene CYP2C19 --source cpic diplotype --diplotype '*2/*1'` returns `status: complete` with a header and no result; `'*1/*2'` returns Intermediate Metabolizer.
- The help example for DPYD, `'*1/*2A'`, also returns nothing: CPIC spells it `c.1905+1G>A (*2A)/Reference`.
- `--gene g6pd` and `--cpic-level c` return `complete`, total 0, because matching is case-sensitive. `biomcp get pgx --gene BRAF --source cpic` returns `complete` with no notice that CPIC has no BRAF record.
- Ticket 0166 chose "a literal zero stays a successful empty reply".

## Proof

- Proof: outside-in cases on recorded CPIC replies: `'*2/*1'` returns the `'*1/*2'` row; `g6pd` equals `G6PD`; an unknown diplotype returns a not-found notice with example spellings; the help example returns a row.

## Starts from

- Starts from: `crates/biomcp-adapters/src/cpic/read.rs` and the PGx help text.

## Keeps

- Keeps: an exact CPIC spelling returns the same row; no phenotype is inferred for a diplotype CPIC does not list.

## Defers

- Defers: CPIC tables with no route: allele (1,378), allele frequency (391,760, deferred by 0168), test alert (2,213), population, publication and drug; and removed pairs (61 of 635), which the pair view hides without help saying so.

## Changes

- Changes: `crates/biomcp-adapters/src/cpic/`, `crates/biomcp-query/src/pgx/`, help text, PGx cases.
