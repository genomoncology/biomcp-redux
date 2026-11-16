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

## Starts from

- Starts from: 0831, 0982 and 0983; the typed MCP tool registry; the hosted mode entry point.

## Keeps

- Keeps: Typed MCP budgets (`sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md`); the core never learns who the user is beyond the header and has no object-storage client.

## Defers

- Defers: nothing.

## Changes

- Changes: the saved-work code and its cases.

## Progress
