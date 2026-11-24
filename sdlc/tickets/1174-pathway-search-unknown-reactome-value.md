# 1174 — pathway-search-unknown-reactome-value

Status: COMPLETE.
Landed: 9e24d8c47ca3de5559bf14c5a19bb9d871b54d02

Milestone: 7

## Outcome

`biomcp search pathway --gene EGFR` completes when Reactome returns its normal pathway list for the gene, or, if a value is truly unknown, the problem names the value so a user can act on it.

## Why

On 1.0.0-87f56ceabae5, `search pathway --gene EGFR` returned status `partial` with problem `value.unknown` on Reactome record P00533 field `pathways`, while still listing 36 pathways. A partial with no named value tells the user nothing. Found by the docs writers on 2026-11-24.

## Proof

- Proof: a replay of the EGFR call shows which Reactome value was unknown; either the value is supported and the search is complete, or the problem carries the value and the docs-visible message says what was skipped.

## Changes

- Changes: Reactome pathway parsing for gene search and a replay case.
