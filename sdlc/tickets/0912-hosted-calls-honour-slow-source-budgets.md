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

## Progress

- A hosted HTTP call's scope is now `Cancellation::hosted(command_timeout_seconds)`. Its end can grow, but only through `Fetch::source_command_deadline(spec, caller)`, which the VAERS and Monarch similarity commands now call in place of `command_deadline(SPEC.budget_for(..))`. When the source declares a `command_budget`, the call's end moves to its arrival plus that budget, and the HTTP waiter sees the same end because the worker and waiter share it. A source with no declared budget never moves it, so every other hosted call still ends at `command_timeout_seconds`, and queue wait still counts against it.
- A fixed limit (`Cancellation::with_budget`, used by the stdio fallback and 0909's 5 s caller-limit test) never grows, and a cancelled scope reports no time left, so a cancel ends a lengthened call.
- Hosted HTTP has no per-call client cancel message; its cancel path is the scope's `cancel()`, which the whole-call timeout and server shutdown use. The fetch test cancels a hosted scope to prove that path.
- New case `adverse-event/vaers-slow-reply-hosted` sends 0909's recorded 36.8 s CDC WONDER reply through a hosted HTTP server. Cases gain a `hosted: true` step option for this.
- `crates/biomcp-fetch/tests/record/command_budget.rs` adds: a hosted 25 s reply with a declared 60 s budget succeeds and the hosted end grows past 20 s; the same reply with no declaration fails at 20 s and leaves the end alone; a cancelled hosted call fails although its source declares 60 s.
- The existing `mcp-hosted/replay-partial-at-deadline` trial still shows an undeclared slow source (mygene, 60 s delay) ending at a 15 s `command_timeout_seconds`.
