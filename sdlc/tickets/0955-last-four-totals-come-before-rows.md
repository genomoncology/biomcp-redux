# 0955 — last-four-totals-come-before-rows

Status: OPEN.

Milestone: 7

## Outcome

The four reply types 0942 left write their total (or page, or offset) before their rows: `GwasStudyPage`, `StudyMutations`, the cell-line section `offset`, and `ProteinSectionPage`. Field order only; the checked-in schema files for these types change text only, by the new order of their `required` lists.

## Why

0942 (2026-11-14) moved every other reply's total ahead of its rows (the paper's W2, the first page read as the whole) but left these four, because in each both rows and total are required and the generated schema lists `required` in declaration order. On main, `search gwas --studies` with 50 results puts its total at byte 1,934 of 2,429. 0942's review read the checker: `compatibility::check_reviewed` compares `required` as a set (`crates/biomcp-types/.../compatibility.rs:19`, `:139-140`), so the reorder passes the 1.0.0 baseline with no review entry. PM decision 2026-11-14: a schema text change that only reorders `required` is allowed for this ticket.

## Proof

- Proof: 0942's contract test helper (`crates/biomcp-types/tests/response/inner_totals.rs`) covers the four types and fails on main; a recorded `search gwas --studies` case asserts the total starts within 1,200 bytes; `make --keep-going landing-gate docs-check schema-release-check` passes with only `required` order changed in the schema files (show the schema diff in Progress).

## Starts from

- Starts from: 0942's ticket and tests; `make schema-generate`; `scripts/schema_release.py`; `tests/cases/gwas/studies.yaml`.

## Keeps

- Keeps: every schema rule other than `required` order; MCP text order (alphabetical).

## Defers

- Defers: nothing.

## Changes

- Changes: the four types' field order, their generated schema files, contract tests, the gwas studies case.
