# 1188 — release-gate-stub-help-reply

Status: OPEN.

Milestone: 7

## Outcome

`make docs-check` passes on main. The mock CLI in `release/test_gate.py` answers the MCP `help` probe as a real local server now does: `structuredContent` with `result.commands` and one text part with the same JSON. All `release.test_gate` cases reach the checks they were written for.

## Why

1177 made `release/smoke.py` `mcp_probe` call `help` and require a text part. The 1185 developer found that `make docs-check` (job j47040) fails four `release.test_gate` assertions, because that test's mock returns an empty MCP help reply. 1183 and 1186 fixed the same stale stub in other release tests.

## Proof

- Proof: `python3 -m unittest release.test_gate` fails four assertions on main and passes after the change; `make docs-check` passes.

## Starts from

- Starts from: `release/test_gate.py` mock CLI, `release/smoke.py` `mcp_probe`.

## Keeps

- Keeps: every assertion in `mcp_probe` and in the gate tests.

## Defers

- Defers: Nothing.

## Changes

- Changes: `release/test_gate.py` only, unless another release test has the same stale stub (then fix it here and say so).
