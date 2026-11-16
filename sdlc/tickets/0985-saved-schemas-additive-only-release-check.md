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
- Proof (from 0982's review, 2026-11-15): `saved changed` compares only the row-format digest, not the reply schema ID, so an envelope-only major (such as 0849's 137-schema bump) does not rebaseline every saved search (`crates/biomcp/src/command/recheck.rs:177-194`); a trial shows a schema-ID-only change reports `changed`, not `rebaselined`.

- Proof (from 0983's review, 2026-11-15): `saved note --project default NAME TEXT` is refused instead of silently noting NAME (`crates/biomcp/src/saved_note.rs:119-123`); an item can be picked by `--source` when a list holds the same entity and ID under two sources (`crates/biomcp-query/src/store/saved/notes.rs:100-117`); an unknown stored note kind or item source is skipped with a problem rather than failing `saved show` and `saved export` (`notes.rs:215`); a `--format md` case for `saved show` with a note holding Markdown characters proves the user-data label and escaping.

## Starts from

- Starts from: `release/` schema release check; `tests/cases/mcp/parity.yaml`, `tests/cases/saved/lists.yaml`; `asks_help` in `crates/biomcp/src/saved.rs`.

## Keeps

- Keeps: 1.0.0 schema rules for every other schema.

## Defers

- Defers: nothing.

## Changes

- Changes: the saved-work code and its cases.

## Progress
