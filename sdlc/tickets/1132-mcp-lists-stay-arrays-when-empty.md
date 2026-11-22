# 1132 — mcp-lists-stay-arrays-when-empty

Status: OPEN.

Milestone: 7

## Outcome

A reply's collection arrays are present at zero, one and many rows, and keep the same success shape across pages. A zero-match trial search has `result.items: []`, and a zero-hit research `search` has `results: []`.

## Why

Found by a Codex review of `biomcp serve` against Armin Ronacher's checklist for MCP servers called from code mode ("What is Codemode", 2026-10-06, in Ian's notes inbox), on main `3b58595cd`, 2026-11-22, run at Ian's request (pm mail m40005): A zero-match trial search kept its schema and page but omitted `result.items`, and the research `search` tool omitted `results` on zero hits. A script that maps over the array then fails on the empty case.

## Proof

- Tests on saved replies: zero, one and many rows for each search family, and the same for the command-line JSON.

## Starts from

- Starts from: main after ticket 1130 (MCP matches CLI after recent fixes) lands, since both change the MCP server.

## Keeps

- Keeps: command-line replies, except where the outcome names them.

## Defers

- Defers: nothing.

## Changes

- Changes: reply builders for search and paged replies, through both the command and MCP, and tests.
