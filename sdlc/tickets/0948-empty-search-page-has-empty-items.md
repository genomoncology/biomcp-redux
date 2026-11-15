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

## Progress

- Stopped for a decision: the ticket conflicts with Ian's decision `sdlc/decisions/2026-10-23-json-is-the-default-output.md` ("the envelope omits nulls, empty lists, unrequested sections and repeated commands") and 0362, which made it a runner check: `reject_empty_lists` in `crates/biomcp/tests/runner/schema.rs:114` fails any recorded reply that holds an empty list, with one narrow exception for typed gnomAD filters (0610).
- Branch state (2775f6c28): every reply row list named `items` (43 fields, including the `page!` macro and `Capped<T>`), plus `AlphaGenomeScorerPage.rows` and `DrugCivicPage.records`, keeps `#[serde(default)]` but no longer skips when empty. A schema-only skip test (`crate::no_schema_default`, `paged.rs`) keeps the generated schemas byte for byte: the fields stay optional and get no `default` annotation, so the 1.0.0 baseline is untouched. The `--name cell-free` step of `diagnostic/intended-use.yaml` asserts `/result/items: []`; seven cases that asserted `omits: [items]` now assert `[]`.
- Gate j41088: fmt, clippy, ratchets and both schema checks (`generated_schemas_preserve_release_contracts`, `checked_in_schemas_match_types`) pass. Cases fail: 110 (plus `readme/trial.yaml` in docs-check), 91 read in the failure list all "product reply contains an empty list", at `/result/items` and at nested `Capped` lists such as `/result/warnings/items` and `/result/items/2/interventions/items`.
