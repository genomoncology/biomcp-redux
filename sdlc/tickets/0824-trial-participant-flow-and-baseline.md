# 0824 — trial-participant-flow-and-baseline

Status: OPEN.

Milestone: 8

## Outcome

`get trial NCT... results` shows participant flow (started, completed and not completed, by arm and period, with reasons) and baseline characteristics when the registry posted them.

## Why

The results section reads outcome measures and adverse events only; the navigator and trial-ops personas needed dropout numbers and got them by curl.

## Proof

- Proof: a routine case under `tests/cases/trial/` with a recorded reply for a trial with posted results asserts one flow period and one baseline measure; it fails on the base commit.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ctgov/detail/results.rs`, which reads `outcomeMeasuresModule` and `adverseEventsModule`; the registry's `participantFlowModule` and `baselineCharacteristicsModule` are in the same fetched record.

## Keeps

- Keeps: every existing field, schema ID rule (a changed reply schema takes a new major), source attribution and page behavior; no new source, key or dependency. The results section's byte ceiling in `tests/*-budgets.yaml` holds: page long flows with the section paging the outcomes already use.

## Defers

- Defers: nothing; trials without posted results answer as they do now.

## Changes

- Changes: `crates/biomcp-adapters/src/ctgov/detail/results.rs`, `crates/biomcp-types/src/`, `tests/cases/trial/`
