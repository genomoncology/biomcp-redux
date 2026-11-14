# 0949 — gtr-gene-with-condition-refusal-names-the-combination

Status: OPEN.

Milestone: 7

## Outcome

When `search diagnostic --source gtr --gene X --condition-id Y` is refused, the message says which combination isn't supported (or the combination works), not that the gene is invalid.

## Why

dx-lab-director round 2 call 14: the combination was refused as an invalid gene. Evidence: bench branch `ticket/0048-dx-r2-evals`, `personas/dx-lab-director/runs/2026-11-14-r2/` (pm job j40752, BioMCP 2d08cab05).

## Proof

- Proof: a case with that combination shows the accurate message or the joined result.

## Starts from

- Starts from: GTR search validation in `crates/biomcp-types/src/query/diagnostic/search_validation.rs` and the GTR adapter.

## Keeps

- Keeps: refusal shapes.

## Defers

- Defers: nothing.

## Changes

- Changes: validation or the GTR filter, a case.
