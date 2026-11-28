# 1227 — trial-phase-filter-exact

Status: COMPLETE.
Landed: 555ea67ac8d0c8198712d1192290a7ed73e9698c

Milestone: 10

## Outcome

`search trial --phase <N>` does what help says: either it returns only studies whose phase is exactly that value, with the combined phases (1/2, 2/3) as their own choices, or help says a phase value also returns the combined-phase studies.

## Why

Help says "Exact source phase", but `--phase 1` includes 17,251 phase 1/2 studies.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp search trial --phase 1 --limit 1` returns NCT03515551, whose registry phases are `["PHASE1","PHASE2"]`. The total, 66,453, equals the registry's `AREA[Phase]PHASE1` count, which includes combined-phase studies.
- Related missing filters, each with a registry count: intervention type (`--intervention DEVICE` is a text search returning 92,273 against 77,420 device-type studies), studies with no phase (145,484), with no intervention (62,358), with no location (61,320).

## Proof

- Proof: outside-in cases on recorded registry replies: `--phase 1` under the chosen rule returns the count help describes, and a combined-phase choice returns the 17,251.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ctgov/search_query.rs`, the phase choices in the trial grammar.

## Keeps

- Keeps: the registry's totals stay the reply's totals; no default status, type or phase filter is added.

## Defers

- Defers: an intervention-type filter and filters for missing phase, intervention or location; file when a case needs them.

## Changes

- Changes: `crates/biomcp-adapters/src/ctgov/`, grammar and help, trial cases.
