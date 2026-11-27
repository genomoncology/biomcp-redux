# 1219 — gwas-trait-name-matching

Status: COMPLETE.
Landed: d5e07dccbac857b08b8428cc5244d6b8321e33f5

Milestone: 10

## Outcome

`search gwas --trait <name>` matches the catalog's trait names without regard to letter case, and when a name matches no trait exactly it returns the catalog's nearest trait names as candidates instead of `complete` with zero rows.

## Why

A capital letter fails the whole page today, and a common short name reads as "no associations".

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case 1: `biomcp search gwas --trait "Breast carcinoma"` returns `status: failed`, source reason `bad-reply`, problem `association_page value.unknown`, and suggests `--fresh`. `--trait "breast carcinoma"` returns 2,094 rows. Cause: a case-sensitive name check applied to every returned row.
- Failing case 2: `biomcp search gwas --trait "type 2 diabetes"` returns `complete` with 0 rows; the catalog holds 9,655 rows under "type 2 diabetes mellitus". `--trait alzheimer` also returns 0.
- `--trait-id` with an ID the catalog does not know (`EFO_0000305`, a 404 at the source) also returns `complete`, 0.

## Proof

- Proof: outside-in cases on recorded replies: the capitalised name returns the 2,094-row page; "type 2 diabetes" returns candidate trait names with their IDs; an unknown trait ID returns not-found.

## Starts from

- Starts from: `crates/biomcp-adapters/src/gwas/search/identity.rs`, `crates/biomcp-adapters/src/gwas/search/`.

## Keeps

- Keeps: an exact lower-case name returns the same total; no association is returned for a trait the caller did not choose.

## Defers

- Defers: a hint, on a parent trait, of how many more rows the child traits hold (`--trait-id MONDO_0004992` returns 397 of 27,077 with children; the flag exists and is documented).

## Changes

- Changes: `crates/biomcp-adapters/src/gwas/search/`, GWAS cases and replies.
