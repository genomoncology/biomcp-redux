# 1053 — document-variant-shorthand-in-release-notes

Status: COMPLETE.
Landed: 85c36a6e2458ddb3f12e0ff95506585ae49f9416

Milestone: 7

## Outcome

The 1.0 release notes explain gene and protein shorthand for CLI and typed MCP variant search and get, with a `BRAF V600E` example and the behavior for multiple verified alleles.

## Why

The landed shorthand change omitted the entry required by the [November 16 release-notes decision](../decisions/2026-11-16-review-follow-ups.md).

## Proof

- Proof: `make docs-check` passes. Review the release note examples against the landed shorthand CLI and typed MCP cases.

## Starts from

- Starts from: `release/notes.md`, `docs/reference/commands.md` and the landed variant shorthand cases in `tests/cases/variant/`.

## Keeps

- Keeps: Existing CLI and MCP behavior.

## Changes

- Changes: `release/notes.md`
