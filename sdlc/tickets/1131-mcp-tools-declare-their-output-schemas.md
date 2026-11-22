# 1131 — mcp-tools-declare-their-output-schemas

Status: OPEN.

Milestone: 7

## Outcome

Every typed MCP tool declares an `outputSchema` by default, `help` included. Each schema names the fields its actions return, down to rows and result fields, not only the reply envelope with `result` as a generic object, and each sampled reply validates against it.

## Why

Found by a Codex review of `biomcp serve` against Armin Ronacher's checklist for MCP servers called from code mode ("What is Codemode", 2026-10-06, in Ian's notes inbox), on main `3b58595cd`, 2026-11-22, run at Ian's request (pm mail m40005): In typed mode only 3 of 18 tools declare `outputSchema` by default, and 17 of 18 with `mcp_typed_output_schema: true`. The declared schemas describe the envelope and leave `result` as a generic object, so a script cannot know a reply's fields before it calls.

## Proof

- A test lists the typed tools with default config and checks that every one has an `outputSchema`, and that a saved reply for each action validates against it.
- Size of `tools/list` in typed mode before and after, in bytes. The PM rules on any growth above the 120,595 bytes measured with schemas on today.

## Starts from

- Starts from: main after ticket 1130 (MCP matches CLI after recent fixes) lands, since both change the MCP server.

## Keeps

- Keeps: command-line replies, except where the outcome names them.

## Defers

- Defers: nothing.

## Changes

- Changes: the typed MCP tool catalog and its schema generation, and tests.
