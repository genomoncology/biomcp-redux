# 1130 — mcp-matches-cli-after-recent-fixes

Status: COMPLETE.
Landed: 887b9c930150b0b44d58eaa882ff9d97070a7ef1

Milestone: 7

## Outcome

Every command-line fix landed for the code-mode comparison reaches `biomcp serve` the same way. That covers the fixes in the comparison’s `climb.md` and every ticket in this repository landed from 2026-11-19 to 2026-11-22. For each fix, the same request sent through the command and through an MCP tool call gets the same options, sections, records, refusals and refusal wording. The guidance the skill gives about the parts an MCP caller uses also reaches the MCP side, through tool descriptions, the server's instructions or `help`. A repeatable parity check proves it, and whatever it finds different is fixed in this ticket.

## Why

Ian, 2026-11-22 (relayed by Jarvis, pm mail m40010): "with our recent fixes to the CLI we need to make sure MCP is up to speed." Next the code-mode project runs round 1 with BioMCP reached over MCP, with and without code mode. The MCP arms must not measure an MCP server that lags the command.

## Proof

- A parity check runs offline against saved replies, plus an optional live run. It sends the same request through the command and through the MCP server's `tools/call`, at least one for each fix in scope, and compares the replies. It lives in the repo's tests and runs in the landing gate.
- A table lists each fix with its MCP status: same, fixed here, or different by design, with the reason.
- Before and after this ticket: the size in bytes of `tools/list` and of the server instructions.

## Starts from

- Starts from: main `3b58595cd`. Landed tickets: `git log --since=2026-11-19 origin/main`.

## Keeps

- Keeps: command-line replies unchanged; the MCP revisions and hosted-mode refusals that `biomcp serve --help` lists.

## Defers

- Defers: new MCP features from Ronacher's checklist (outputSchema, tool search), which a separate review is weighing.

## Changes

- Changes: the MCP server code, its tool descriptions or instructions, and tests.
