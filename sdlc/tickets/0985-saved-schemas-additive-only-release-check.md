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

- 2026-11-16: Additive-only mode: the release comparison treats any `biomcp.saved-*` schema as additive-only. It keeps its major, takes no reviewed exceptions, keeps the same required fields, and every shared path must be compatible in both directions, so only optional fields (and descriptions) may be added. `saved_schemas_accept_only_added_optional_fields` shows an added optional field passes while a required addition, an enum widening and a major bump fail, and that the same changes still pass outside saved work. The 1.0.0 baseline holds no saved schemas yet, so the mode applies once a release records them.
- 2026-11-16: The MCP root-help step asserts the saved line is absent (`excludes`); the export step asserts `/error/option`. `asks_help` now skips a value-taking option's value, and `--note` accepts a value starting with `-`, so `saved list add help-words --entity gene --note --help BRAF` keeps the note `--help` (fails on the base, which answered help).
- 2026-11-16: `saved changed` compares only the row digest after `#` in the stored row schema, so a new schema ID with the same rows reports `changed`, not `rebaselined` (store test in `tests.rs`).
- 2026-11-16: `saved note --project default NAME TEXT` is refused on `--project`. `saved note --source` picks an item held under one entity and two sources; an ambiguous item with `--entity` given points at `--source`.
- 2026-11-16: Decision: the envelope's `problems` need a provider `Source`, which a stored note or item has none of, so an unknown stored note kind or item source is reported in a new optional `skipped` list in the `biomcp.saved/1` result (record, field, value) and the row stays in the file. `saved show` and `saved export` no longer fail on it. A list add or remove reply leaves such an item out without naming it.
- 2026-11-16: A `saved show --format md` step proves notes appear under "User notes" with Markdown, HTML and the link kept literal.
