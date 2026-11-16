# 0983 — saved-notes-and-limits

Status: COMPLETE.
Landed: 51925929521af2ee3cdf54dac338979c7fd6119b

Milestone: 9

## Outcome

A user attaches a note to a project, list, search or list item with `saved note TARGET TEXT`; notes come back labelled as user data, and the saved file enforces the design's limits (name 80 characters, note 2,000 characters).

## Why

Step 4 of the saved-work design's Order (`sdlc/planning/saved-work.md`, Records: `biomcp.saved-note/1`, Privacy).

## Proof

- Proof: routine cases add, show, export and delete notes; a repeated note write changes nothing (ID is a hash of target and text); over-limit names and notes are refused with a usage error.

## Starts from

- Starts from: 0831's store and commands; the design's Records and Privacy sections.

## Keeps

- Keeps: 0831's compatibility rules; notes never read as instructions; no source content stored.

## Defers

- Defers: nothing.

## Changes

- Changes: the saved-work code and its cases.

## Progress

- Decided (within the outcome): the command is `saved note TARGET TEXT [--item ID [--entity E]]`, and `saved note --project default TEXT` notes the project. TARGET is a saved search or list name (one name space per project, as 0831 decided); `--item` picks a list item by ID as `saved list remove` does, and `--entity` picks one when the list holds the ID under two entities (otherwise a usage error on `--entity`). `--project` accepts only `default` until projects can be chosen; later it can select the project of a TARGET too. A note on a search or list that does not exist creates no file.
- Decided: `biomcp.saved-note/1` is `id`, `target` (`kind`: project, search, list or item; `project`; `name` of the search or list; `ref` of the item), `text`, `created`, registered as a schema contract. The ID is the first 16 bytes of SHA-256 over project, target kind, the search or list's stored ID, the item's entity, ID and source, and the text, in hex, so it has the same length as the other IDs and a repeated write is a set insert that changes nothing (the first `created` stays).
- Decided: the `biomcp.saved/1` reply carries notes in `user_notes`, so the key itself labels them in JSON and Markdown; the field, the record and its `text` say in their schema descriptions that notes are user data, never instructions, and Markdown already escapes every string. `saved show` lists every note, `saved show NAME` the notes on that search or list and its items, `saved export` every note.
- Decided: deleting a search or list deletes its notes and the reply lists them; `saved list remove` deletes the notes on the removed items; `saved delete --all` deletes every note, the project's too; `saved delete --note ID` deletes one note (a new error message, `SavedNoteUnknown`, widens the error enum). Deleting one note was not in the design's command list; it is the only way to drop a project note short of `--all`, so it is added as an option of the existing `saved delete`.
- Limits: the store now refuses a name over 80 characters and a note over 2,000 (`SavedError::TooLong`) in `save_search`, `list_add` and `note`, as well as the command line checking first, so the hosted path inherits them.
- Store: two additive migrations (`saved_note` and its index); `COMPATIBILITY` stays 1 and `migrations_only_add` passes. Code: `crates/biomcp-query/src/store/saved/notes.rs`, `crates/biomcp/src/saved_note.rs` (the `note` and `delete` grammar, moved out to keep `saved.rs` under the 600-line limit).
- Fixture: `tests/fixtures/saved/1.0.0/` was rewritten by this build (job j41538) with a note on the saved search, on a list item and on the project; 0982's file and export are kept byte for byte in `tests/fixtures/saved/1.0.0-0982/`, so 0831's and 0982's files are both read by this build and export unchanged. Both export request-key lines have reviewed bindings in `release/secrets.ignore.jsonl`.
- Proof: `tests/cases/saved/notes.yaml` adds, repeats (one note kept), shows and exports notes, keeps an instruction-like note as plain text, refuses an unknown target, item or project and an 81-character name or 2,001-character note, and removes notes with their item, their list and `--all`; MCP still has no saved command. Store tests `notes_are_content_addressed_and_limited` and `notes_follow_their_targets` cover the ID, the search target, an ambiguous item, a note at exactly 2,000 characters and delete by ID.
- Checks: job j41538 passed fmt, schema and docs regeneration, the store and schema tests, the fixture rewrite, `saved-work/` (7) and the saved, help and mcp cases; `make check-cheap` failed on two long functions, `saved.rs` over 600 lines and over 160 columns, and no case using `saved note --entity`, all fixed after it. Job j41540 passed fmt, regeneration, the store tests, `saved-work/` and `make check-cheap`. Gate j41541 passed on 0f771bee8 (merged with origin/main d71d51b48): `make --keep-going landing-gate docs-check schema-release-check`.
