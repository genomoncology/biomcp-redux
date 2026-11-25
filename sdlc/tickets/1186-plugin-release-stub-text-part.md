# 1186 — plugin-release-stub-text-part

Status: OPEN.

Milestone: 7

## Outcome

The fake MCP reply in `release/test_promotion.py` answers as a real local server now does: `structuredContent` plus one text part with the same JSON, and the plugin release tests pass.

## Why

1177 made `release/smoke.py` `mcp_probe` require a text part; the 1183 developer found by inspection that the plugin test's stand-in (release/test_promotion.py:127) lacks one.

## Proof

- Proof: `python3 -m unittest release.test_promotion` (the plugin probe test) fails or would fail on main and passes after the change; if it currently skips, say how it was run.

## Starts from

- Starts from: `release/test_promotion.py` around line 127, `release/smoke.py` `mcp_probe`.

## Keeps

- Keeps: Every assertion in `mcp_probe`.

## Defers

- Defers: Nothing.

## Changes

- Changes: release/test_promotion.py only.
