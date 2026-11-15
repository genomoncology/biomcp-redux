# 0948 — empty-search-page-has-empty-items

Status: OPEN.

Milestone: 7

## Outcome

An empty search page keeps leaving out `items`, like every empty list (Ian, `sdlc/decisions/2026-11-15-empty-search-pages-keep-omitting-items.md`). The README, the envelope `page` description and the skill say that `page.total` and `page.returned` are 0 and that an agent reads `total` before `items`.

## Why

dx-lab-director round 2 call 27: an empty search page had no `items` key and the agent's script failed. Evidence: bench branch `ticket/0048-dx-r2-evals`, `personas/dx-lab-director/runs/2026-11-14-r2/` (pm job j40752, BioMCP 2d08cab05). Many list fields use `skip_serializing_if` empty.

## Proof

- Proof: the recorded empty search case asserts `items` is absent and the page counts are 0; the README, the envelope `page` description and the skill say so.

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
- PM decision 2026-11-14: option 2. Ian's decision `sdlc/decisions/2026-10-23-json-is-the-default-output.md` stands (replies omit empty lists, because size matters more than format); a ticket does not override it. The code change is reverted (commits 903c8597d and 7a6820cdc revert 2775f6c28 and ced9deaf4), and the Outcome is reworded to state the omission plainly instead.
- Changed: `README.md` (the JSON output paragraph) says replies leave out nulls and empty lists, so a zero-result search has `page.total` and `page.returned` 0 and no `result.items`. The envelope `page` field description (`crates/biomcp-types/src/response/envelope.rs`) says the same; a description change only, regenerated into the checked-in schema files. The persona's `--name cell-free` step in `tests/cases/diagnostic/intended-use.yaml` asserts total 0, returned 0 and `/result` omits `items`.
- Ian decided 2026-11-15 (`sdlc/decisions/2026-11-15-empty-search-pages-keep-omitting-items.md`): keep omitting `items`; the docs and skill say `total` is 0 and to read `total` before `items`. Outcome reworded to match.
- Gate j41101 failed only the ratchets line-width check (160 characters) at `envelope.rs` line 40. Shortened the `page` description ("the result leaves out `items`") and regenerated the 136 checked-in schema files by the same text substitution; description-only change.
- `skills/SKILL.md` gains one line: an empty page has `total` 0 and no `items`, so read `total` before `items`.
