# 0984 — saved-typed-mcp-tool-and-save-step

Status: COMPLETE.
Landed: 76ef8d4e808530eeccffe270a243eb7df47bb4a8

Milestone: 9

## Outcome

Typed MCP gets one `saved` tool with the design's actions (`show`, `save_search`, `changed`, `list_add`, `list_remove`, `note`, `export`, `delete`); a first search page with results offers a runnable `save` next step; the core accepts a saved-work folder and derives the file path from an account header in hosted mode, refusing without it.

## Why

Step 5 of the saved-work design's Order (`sdlc/planning/saved-work.md`, Commands, Hosted points 3).

## Proof

- Proof: mcp cases run each action; the `save` step runs as written; hosted mode without the header refuses and a client-sent path is never accepted.
- Proof: tools-list and reply tokens before and after, measured against the typed MCP budget decisions.

- Proof (from 0985's review, 2026-11-15, because a rolling hosted deploy runs mixed builds): a snapshot with any skipped row or source is read as incomplete, so `saved changed` answers `incomplete` instead of false `new`/`gone` (`crates/biomcp-query/src/store/saved/snapshot.rs`, `snapshot()`); `saved note` on an item whose stored source this build cannot read answers with its skip rather than a local error (`notes.rs:204-206`); the saved schemas' additive-only check accepts a reviewed raise of a shared text length limit, as it accepts reviewed pattern widenings (`crates/biomcp-types/tests/schemas/compatibility/additive.rs`).

## Starts from

- Starts from: 0831, 0982 and 0983; the typed MCP tool registry; the hosted mode entry point.

## Keeps

- Keeps: Typed MCP budgets (`sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md`); the core never learns who the user is beyond the header and has no object-storage client.

## Defers

- Defers: nothing.

## Changes

- Changes: the saved-work code and its cases.

## Progress

- Decided (within the outcome): the typed `saved` tool spells each action as the command line's saved words and runs them through the same code (`crates/biomcp/src/saved_mcp.rs`, `crates/biomcp/src/mcp/typed/saved.rs`); values follow `--` or join their option with `=`, so none is read as an option, and a refused argument is named by the tool's field (`name`, `ids`, `text`...). Only listed fields are read: any other, such as `path`, is refused before anything runs. Like the entity tools, the schema names its fields without forbidding others. `list_add`'s `entity` takes `cell_line` or `cell-line`.
- Decided: `save_search` takes `{name, call}`, where `call` is a search call in the advertised `{name, arguments}` form a reply's next steps use. The call is checked as the entity tool would check it, must be a first-page search, runs through the MCP path (hosted policy, keys and excluded sources apply), and is saved only when it succeeds. MCP answers with the saved search, not the search page it just read; the command line still answers with the page.
- Decided: the `save` step is typed MCP only. Single mode and the command line have no `saved` tool call to offer (single mode still has no saved command, as 0831 decided), and leaving CLI replies alone keeps every CLI reply size unchanged. A typed first search page with status complete and at least one row gets `{"kind":"save","name":"saved","arguments":{"action":"save_search","save_search":{"name":<the search's own words>,"call":<the search call>}}}`. The name is the search's words after `search` (80 characters at most), so running the step again changes nothing. It is offered only where it can run: not when the catalog lacks `saved`, and not in hosted mode without an account.
- Decided (Hosted points 2-3): `serve --hosted --saved-folder DIR` (requires `--hosted`) turns saved work on in hosted mode; without the folder hosted mode offers no `saved` tool, so today's hosted catalog is unchanged until the hosting side is ready. Each call's file is `DIR/<account>/saved.sqlite`, where the account is the `x-biomcp-account` header: 1 to 128 ASCII letters, digits, `-` or `_`, an opaque key the wrapper chooses (it may hash its user ID), so it can never climb out of the folder. A malformed or repeated header is HTTP 400, like the other hosted headers; a `saved` call without it answers `usage.mcp_unavailable` with the new message `SavedAccount` (widens the error message enum). Plain HTTP and stdio ignore the header and use the data folder. The core reads nothing else about the user and has no object-storage client. Documented in `docs/configuration.md`.
- Decided: a hosted `saved changed` re-check runs through the call's own transport (its keys, left-out sources and deadline); `crate::command::recheck` takes the MCP context. A caller's own `--fresh` stays refused in hosted mode.
- Code: the store calls take an `At` (encoding, file, MCP context) instead of an encoding; `saved changed` moved to `crates/biomcp/src/saved_changed.rs` to keep `saved.rs` under 600 lines.
- 0985 review proofs: a snapshot with any skipped row or source is now read as incomplete (`snapshot()`), so `saved changed` answers `incomplete`, not false new or gone (`unknown_snapshot_row_sources_are_skipped`, extended); `SavedStore::note` now returns the note or its skip, and a note on an item whose stored source this build cannot read is kept and answers with the skip (`a_note_on_an_unreadable_item_answers_its_skip`); the saved schemas' reverse check takes a reviewed raise of a text length limit from a new `length_raises` list in `release/schema-reviews.json` (schema, path, from, to, review), bound to its path and limits (`saved_schemas_take_reviewed_text_length_raises`).
- Proof: `tests/cases/saved/mcp.yaml` (typed stdio, recorded trial replies) runs the reply's save step as written and by hand, sees the search from the command line, runs every action, and refuses `path`, an unknown action, a non-search call, an unknown name and a list add without IDs. The runner trial `saved-work/hosted-account-folder` starts a hosted HTTP server with a saved-work folder: no header is refused and writes nothing; alice's list lands in `<folder>/alice/saved.sqlite`; bob sees none of it; `../alice`, `alice/x`, `..` and a repeated header get HTTP 400; `path` is refused; a search without an account gets no save step, and with one the step runs as written. Test catalogs list `saved` (`typed_catalog.rs`); hosted without a folder still serves the 17 entity tools.
- Measured (job j41567, `target/debug/biomcp serve --tool-mode typed`, recorded `trial/condition` replies; tokens estimated as characters/4, since tiktoken is not installed on the M5): the `saved` tool is 2,392 characters against the 2,400 typed tool budget (name, description and input schema), 2,516 characters (about 629 tokens) as defined. The typed tools/list grows from 87,502 characters (about 21,876 tokens, 17 tools) to 90,019 (about 22,505 tokens, 18 tools). A first `trial` search page grows from 5,532 to 5,744 characters (about 1,383 to 1,436 tokens) for its save step; other replies are unchanged. No other wording was trimmed; the tool's own descriptions were written to fit.
- Gate j41569 stopped at the first-read check, which counted the typed tools; `scripts/check_docs.py` now counts `saved` among the shared tools. Gates j41570 and j41571 failed only `tests/cases/diagnostic/runnable-related.yaml`, whose typed first page now carries the save step: the typed step counts 7 and asserts the step (j41571 had the change on the CLI step by mistake). Gate j41572 passed the cases and failed one library test that counted 17 local typed tools; it now counts 18.
- Gate j41574 passed on fdc43dd9b (origin/main 1ec8a30af): `make --keep-going landing-gate docs-check schema-release-check`.
