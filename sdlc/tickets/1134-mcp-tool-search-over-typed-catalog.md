# 1134 — mcp-tool-search-over-typed-catalog

Status: COMPLETE.
Landed: 97c55b0e78776e4363cb346a66103275d3752324

Milestone: 7

## Outcome

An MCP caller can find typed tools by task without loading the whole catalog: a search returns tool names and short descriptions first, and full definitions on request.

## Why

Found by a Codex review of `biomcp serve` against Armin Ronacher's checklist for MCP servers called from code mode ("What is Codemode", 2026-10-06, in Ian's notes inbox), on main `3b58595cd`, 2026-11-22, run at Ian's request (pm mail m40005): Typed mode's `tools/list` is 97,620 bytes for 18 tools, or 120,595 with output schemas. Both articles argue that loading every tool definition up front is what code mode and tool search avoid. Anthropic reports 150,000 tokens down to 2,000 on one workflow.

## Proof

- A test sends a task phrase such as "trials near a city" and gets the trial tool's name and short description, then its full definition on request.
- Bytes an agent loads to make its first call, before and after.

## Starts from

- Starts from: main after ticket 1130 (MCP matches CLI after recent fixes) lands, since both change the MCP server.

## Keeps

- Keeps: command-line replies, except where the outcome names them.

## Defers

- Defers: nothing.

## Changes

- Changes: the MCP server's tools and its initialize instructions, and tests.
