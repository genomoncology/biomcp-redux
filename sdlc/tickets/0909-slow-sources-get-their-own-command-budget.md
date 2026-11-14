# 0909 — slow-sources-get-their-own-command-budget

Status: OPEN.

Milestone: 7

## Outcome

A question that goes to a known slow source finishes when the source answers in its normal time. Today that fails: a VAERS symptom table from CDC WONDER takes about 18-25 s, and a Monarch full-mode ranking takes about 20 s. A source can declare a longer command budget, and the command uses the larger of the user's budget and the source's. Any limit the MCP client sets still caps it.

## Why

Each source's deadline sits inside the whole command's budget, which defaults to 20 s (`crates/biomcp/src/config.rs:176`). So a source's own deadline above 20 s never takes effect.

- **VAERS** already declares 30 s (`crates/biomcp-adapters/src/vaers/mod.rs:22`). On 2026-11-13, three of four live COVID-19 symptom requests failed at the 20 s budget; the one that worked took 18 s (0895 jobs j39507 and j39525; review of 0895). A pv-scientist agent would fail this question most of the time.
- **Monarch** gained a 30 s similarity deadline in 0886, with the same 20 s cap. Full-mode phenodigm ranking took 19-23 s, and its first recording attempts timed out.

## Proof

- Proof:
  - a test where a source declaring a 60 s budget answers a replayed request with a delay over 20 s and succeeds;
  - the same request to a source without a declared budget fails at 20 s;
  - a client-set limit below the source's budget still wins.

## Starts from

- Starts from:
  - the command deadline start in `crates/biomcp-fetch/src/lib.rs:386-405` (for example `cx.budget.max(spec.deadline)` for the sources a command calls);
  - `vaers::SPEC` and the Monarch similarity spec from 0886.

## Keeps

- Keeps: the 20 s default for every other command; `command_timeout_seconds` as the user's override.

## Defers

- Defers: nothing.

## Changes

- Changes:
  - `crates/biomcp-fetch/`;
  - `crates/biomcp-adapters/src/vaers/` (raise the deadline to about 60 s);
  - the Monarch similarity spec;
  - `docs/`.
- Also, two small cleanups from the 0895 review:
  - offer the VAERS sort step only when a snapshot exists (`crates/biomcp-query/src/adverse_event/vaers.rs:141`, guarded like the page step at :114), so "no new request" stays true;
  - say "percent of the N reports" in the sort notice and the percent field's description.
