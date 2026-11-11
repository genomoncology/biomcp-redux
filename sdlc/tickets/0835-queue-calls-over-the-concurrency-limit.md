# 0835 — queue-calls-over-the-concurrency-limit

Status: OPEN.

Milestone: 7

## Outcome

When more tool calls arrive at once than the server runs at once, the extra calls wait their turn within the call's deadline and then run; a refusal happens only when the wait would pass the deadline, and it says how long to wait.

## Why

In `biomed-agent-bench/results/baseline-1.0.md` (build c2511ee90, which is current main) `Tool capacity exceeded: this server runs at most 32 tool calls at once` came back 37 times, almost all when one script sent many calls together, which is how code-writing agents work.

## Proof

- Proof: an MCP case sends 64 calls at once to a server limited to 32 against recorded sources and all 64 complete; a case with a deadline too short for the queue gets the timed refusal.

## Starts from

- Starts from: the capacity check in `crates/biomcp/src/mcp/` and the hosted whole-call deadline in `serve`.

## Keeps

- Keeps: per-source rate limits and quotas; the hosted per-user limit in the hosting service is separate and unchanged.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp/src/mcp/`, `tests/cases/mcp/`
