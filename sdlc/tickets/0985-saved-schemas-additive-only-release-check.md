# 0985 — saved-schemas-additive-only-release-check

Status: OPEN.

Milestone: 9

## Outcome

The schema release check holds `biomcp.saved-*` schemas to additive-only changes (Compatibility point 3 of `sdlc/planning/saved-work.md`), and 0831's review gaps are closed: the MCP root-help step proves the `saved` line is absent, the `saved export --unrecognized-argument` step asserts `/error/option`, and a `--help` word given as an option value (`saved list add x --entity gene --note --help BRAF`) is read as the value, not a help request.

## Why

0831 review (2026-11-15) left these open; Compatibility point 3 was deferred from 0831.

## Proof

- Proof: a schema-release-check test shows a non-additive change to a `biomcp.saved-*` schema fails and an added optional field passes.
- Proof: the three case steps above fail on the base commit.

## Starts from

- Starts from: `release/` schema release check; `tests/cases/mcp/parity.yaml`, `tests/cases/saved/lists.yaml`; `asks_help` in `crates/biomcp/src/saved.rs`.

## Keeps

- Keeps: 1.0.0 schema rules for every other schema.

## Defers

- Defers: nothing.

## Changes

- Changes: the saved-work code and its cases.

## Progress
