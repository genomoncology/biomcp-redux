# 1039 — grant-search-wording-says-award-rows

Status: OPEN.

Milestone: 7

## Outcome

`search grant` help, schemas and hint use one vocabulary: rows are awards or award components per fiscal year. The hint names the narrowing option in a form both CLI and MCP callers can use.

## Why

1032's re-review (2026-11-17): `--limit` and `--offset` still say "project-years" (`crates/biomcp-types/src/query/grant.rs:64`, `:68`, `:127`, `:131`; module comment `crates/biomcp-types/src/response/grant.rs:1`) while the rest says award rows; the hint names `--investigator-id` though an MCP caller uses `investigator_id` (`response/grant.rs:111-112`).

## Proof

- Proof: the grant help case and one replayed step assert the new wording; they fail on the base commit.

## Starts from

- Starts from: the lines above.

## Keeps

- Keeps: behaviour and counts.

## Defers

- Defers: nothing.

## Changes

- Changes: descriptions, regenerated schemas and docs, the case.
