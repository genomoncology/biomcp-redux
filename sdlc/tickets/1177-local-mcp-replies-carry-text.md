# 1177 — local-mcp-replies-carry-text

Status: COMPLETE.
Landed: 6c9fe302d9c6d7611ffb2b2dcc87b1d84f97c3da

Milestone: 7

## Outcome

A local BioMCP MCP server (stdio and `serve --http`) sends each successful tool reply both as structured content and as a text content part by default. The current single-form behaviour stays available as the explicit opt-in `once`.

## Why

0837 (MCP replies are sent once) chose structured-only replies, tested with Claude Code and pi only. `biomcp mcp-config` and the setup pages also teach Cursor, Claude Desktop, VS Code and Cline, which were never tested, and protocol revision 2025-03-26 has no structured field, so those clients may show the model `"content": []`. Found by the 2026-11-24 adversarial review. This revises the 0837 default for untested clients; 0837's measured cost of sending both was nil in both tested clients.

## Proof

- Proof: A stdio and an HTTP case at protocol revisions 2025-03-26 and the current revision show a non-empty text part on a successful reply by default, and none with `once` set; `release/smoke.py` asserts the text part.

## Starts from

- Starts from: `crates/biomcp/src/mcp/protocol.rs:161-171`, `crates/biomcp/src/config.rs:97-107`, `release/smoke.py:192-212`, `sdlc/tickets/0837-mcp-replies-are-sent-once.md`.

## Keeps

- Keeps: Structured content is unchanged; hosted settings that set the reply form explicitly keep their value; `once` behaves exactly as today's default.

## Defers

- Defers: Testing every named client live: the default no longer depends on them.

## Changes

- Changes: MCP reply form default, its config docs, smoke assertion and cases.
