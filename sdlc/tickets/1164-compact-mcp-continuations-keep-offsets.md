# 1164 — compact-mcp-continuations-keep-offsets

Status: OPEN.

Milestone: 7

## Outcome

Every continuation a typed MCP reply offers, in the compact catalog and the full one, advances: following it returns the next page, never the first page again.

## Why

Found 2026-11-23 while verifying the docs workflow "Somatic biomarker and drug evidence" (hosting ticket 0116). The compact MCP call `variant {"action":"get","get":{"id":"CA123643","section":["civic","hotspots"]}}` returned continuations that omitted the offsets; replaying one returned the first page again. The reply's section data named offsets 1 and 3, and the CLI advanced with `--offset 3`. An agent that follows `next`, as the skill teaches, loops on page one.

## Proof

- Proof: a replay of the CA123643 CIViC and hotspots read over compact typed MCP: following each offered continuation returns the next page, until the last; the same holds in the full catalog and for any other multi-section get that pages per section.

## Changes

- Changes: The typed MCP continuation builder and the variant multi-section paging, and a replay case.
