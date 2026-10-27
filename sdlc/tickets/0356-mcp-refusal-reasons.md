# 0356 — Say why MCP refuses a local-only command

Status: OPEN.

Milestone: 5

## Outcome

When an MCP client calls a command its transport refuses (`sync`, `store` and `key` over HTTP, `key` over stdio; stdio keeps `sync` and `store clean`, PM ruling m10617), the error says the command is unavailable over MCP and must run from the command line. It no longer returns the generic `usage.arguments` message.

- Why: the docs lead's second-pass product findings (2026-10-27). Agents get "expected a supported command with checked arguments" and retry with other arguments.
- Starts from: the MCP command admission and the error codes from 0338.
- Rules: a fixed message with the command name only; no echoed input.
- Proof: stdio and HTTP cases for each refused command return the new code and message.
- Changes: MCP admission, error codes and cases.
- Defers: nothing. Joins the 0336 refresh.
