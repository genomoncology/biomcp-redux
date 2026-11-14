# 0912 — hosted-calls-honour-slow-source-budgets

Status: OPEN.

Milestone: 7

## Outcome

On the hosted server, a call to a slow source, such as a VAERS symptom table or a Monarch similarity ranking, gets the source's declared command budget from 0909. It no longer always ends at `command_timeout_seconds` (20 s). Every other hosted call keeps its bound.

## Why

0909 let a source declare a longer command budget, and the CLI and local MCP honour it. The hosted HTTP server still ends every call at `command_timeout_seconds` (`crates/biomcp/src/mcp/http/call.rs:67-91`). CDC WONDER took 36.8 s for the COVID-19 symptom table in 0909's job j39797, so a hosted VAERS call still times out. Found by the 0909 developer and confirmed in its review (2026-11-13).

## Proof

- Proof:
  - a hosted-mode case replaying 0909's recorded 36.8 s VAERS reply completes;
  - a hosted call to a source with no declared budget still ends at `command_timeout_seconds`;
  - a client cancel still wins.

## Starts from

- Starts from:
  - `crates/biomcp/src/mcp/http/call.rs`;
  - 0909's `command_budget` and `budget_for`.

## Keeps

- Keeps: the hosted bound for every other call; the operator's `command_timeout_seconds`.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp/src/mcp/http/`, `tests/cases/`, `docs/configuration.md`.
