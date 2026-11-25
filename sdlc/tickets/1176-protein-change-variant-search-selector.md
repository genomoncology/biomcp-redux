# 1176 — protein-change-variant-search-selector

Status: COMPLETE.
Landed: 8d44830dfc0ad2ba90d04973961d2fc03fb9e8a6

Milestone: 7

## Outcome

`biomcp search variant --gene BRAF --protein V600K` and `biomcp get variant "BRAF V600K"` find the variant at default settings, as they do with `--disable-sources noncommercial,licensed`. Protein-change lookup uses the snpEff protein selector by default, so substitutions that need two base changes (BRAF V600K, V600R, KRAS G12F) are found, and EGFR L858R returns every row the snpEff selector returns.

## Why

On 1.0.0-c801f0fc2b84 (2026-11-24 adversarial review), the default search returned `complete` with zero rows for BRAF V600K, V600R and KRAS G12F, and `get variant "BRAF V600K"` returned `absent`: a wrong answer reported as a success. The default picks the dbNSFP selector, which holds single-base changes only.

## Proof

- Proof: Replay cases for search and get of BRAF V600K, BRAF V600R, KRAS G12F and EGFR L858R at default settings fail on main and pass after the change.

## Starts from

- Starts from: `crates/biomcp-query/src/variant/search.rs:105-111` (selector choice); the restricted-level path, which already uses the snpEff selector.

## Keeps

- Keeps: Single-base protein changes such as BRAF V600E still return the same variants; the restricted-level path is unchanged.

## Defers

- Defers: Any gap of the snpEff selector itself: record it as found, not fixed here.

## Changes

- Changes: variant search selector choice, its replay fixtures and cases.
