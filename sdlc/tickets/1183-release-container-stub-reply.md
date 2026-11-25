# 1183 — release-container-stub-reply

Status: COMPLETE.
Landed: f9d4333492769f6abaaf35107e91623aca14d19b

Milestone: 7

## Outcome

`python3 -m unittest release.test_linux` passes on main. The fake container's `serve` reply in `release/test_linux.py` answers the probe's `help` call as a real local server now does: a successful reply with `structuredContent` holding `result.commands`, and one text part with the same JSON.

## Why

1177 (local MCP replies carry text) made `release/smoke.py` `mcp_probe` call `help` and require a text part matching the structured content. The fake container in `release/test_linux.py` still answers the tool call with `{}`, so four `LinuxContainers` tests fail with "installed plugin command did not return help commands" (seen in job j47028). The landing gate does not run these tests, so main carries the failure.

## Proof

- Proof: `python3 -m unittest release.test_linux` fails on main with four failures and passes after the change.

## Starts from

- Starts from: `release/test_linux.py` (the `args[-1] == 'serve'` branch of the fake docker), `release/smoke.py` `mcp_probe`.

## Keeps

- Keeps: every assertion in `mcp_probe`; the tests' defect cases still fail for their own reasons.

## Defers

- Defers: whether the landing gate should run `release.test_linux`: note it in the report.

## Changes

- Changes: `release/test_linux.py` only.
