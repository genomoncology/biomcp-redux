# 0984 — saved-typed-mcp-tool-and-save-step

Status: OPEN.

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
