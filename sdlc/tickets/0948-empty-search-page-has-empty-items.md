# 0948 — empty-search-page-has-empty-items

Status: OPEN.

Milestone: 7

## Outcome

A search page with no matches carries an empty `items` list (or the source's list field), so scripts that read `.items` don't break on zero results.

## Why

dx-lab-director round 2 call 27: an empty search page had no `items` key and the agent's script failed. Evidence: bench branch `ticket/0048-dx-r2-evals`, `personas/dx-lab-director/runs/2026-11-14-r2/` (pm job j40752, BioMCP 2d08cab05). Many list fields use `skip_serializing_if` empty.

## Proof

- Proof: a recorded empty search case asserts `items` is present and empty.

## Starts from

- Starts from: the page types with `skip_serializing_if = "<[_]>::is_empty"` on items (grep `crates/biomcp-types/src/`). Decide whether all search pages or only the diagnostic one; prefer all page types for one rule.

## Keeps

- Keeps: the 1.0.0 schema rules (an always-present optional list is fine; check required lists); reply size caps.

## Defers

- Defers: non-page lists.

## Changes

- Changes: page types, cases.
