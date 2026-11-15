# 0972 — mcp-replies-keep-field-order

Status: OPEN.

Milestone: 7

## Outcome

MCP replies list each object's fields in the same order as CLI JSON (the order the reply types declare), so what help and the skill say about field order is true over MCP too, without changing any value.

## Why

0962 (2026-11-15): MCP turns each reply into a JSON object that sorts keys alphabetically, so 0966's shared CDx fields (which lead each CLI row) come after FDA's `cells` over MCP, and `help search diagnostic` now says they start each row, which is false over MCP. Agents reading a truncated MCP reply miss what the CLI shows first, as the round 4 agent did with `head -40`.

## Proof

- Proof: an MCP case on the recorded CDx page shows `row_date` before `cells` in the text content; the full suite passes; cache keys, continuation tokens and replay matching are shown unaffected (or migrated) by a named check.

## Starts from

- Starts from: 0962's Progress (serde_json `preserve_order` in the workspace Cargo.toml, or ordered text in MCP text content only); the MCP reply path in crates/biomcp/src/mcp/.

## Keeps

- Keeps: every value; the 1.0.0 schema; recordings and cache keys still match, or the change says what it migrates.

## Defers

- Defers: nothing.

## Changes

- Changes: serde_json features or the MCP reply serialization, regenerated schemas/docs, cases.
