# 1133 — mcp-errors-in-structured-content

Status: OPEN.

Milestone: 7

## Outcome

An MCP tool error carries its machine-readable reply (code, arguments, the runnable help or recovery call) in `structuredContent` as well as in text, so a script can branch on the error code without parsing text.

## Why

Found by a Codex review of `biomcp serve` against Armin Ronacher's checklist for MCP servers called from code mode ("What is Codemode", 2026-10-06, in Ian's notes inbox), on main `3b58595cd`, 2026-11-22, run at Ian's request (pm mail m40005): In the default local reply format, tool errors carry JSON only in `content[0].text`. A gene get with no ID returned `isError:true`, `usage.arguments` and a runnable help call, and code had to parse the text block to read them.

## Proof

- Tests: usage, absent and source-failure errors each come back with `isError:true` and a `structuredContent` that matches the command line's JSON error reply.

## Starts from

- Starts from: main after ticket 1130 (MCP matches CLI after recent fixes) lands, since both change the MCP server.

## Keeps

- Keeps: command-line replies, except where the outcome names them.

## Defers

- Defers: nothing.

## Changes

- Changes: the MCP protocol layer's error path, and tests.
