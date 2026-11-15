# 0954 — wikipathways-sync-failure-names-its-reason

Status: OPEN.

Milestone: 7

## Outcome

`biomcp sync wikipathways` either succeeds on the M5 or fails with a message that names the cause (which file, which step, what the source returned), and the cause found is fixed or recorded.

## Why

0940's developer (2026-11-14): the sync worked on the M5 in job j40885, then failed four times in a row with no reason given, even with a fresh cache folder, while the same catalog files downloaded fine with curl. A failure without a reason can't be acted on, and `tests/cases/pathway/empty-relations.yaml` starts with that sync and runs in public live checks.

## Proof

- Proof: a live job reproduces or rules out the failure; a case or test shows a failed WikiPathways sync names its cause.

## Starts from

- Starts from: pm jobs j40885 and the later failed 0940 jobs (`pm job list 0940`); the WikiPathways sync adapter in `crates/biomcp-adapters/src/wikipathways/`; the shared bulk sync path in `crates/biomcp-fetch` (0951 is changing its time limit; coordinate if both are open).

## Keeps

- Keeps: the sync's files and cache layout.

## Defers

- Defers: nothing.

## Changes

- Changes: the WikiPathways sync and the shared sync error message, as the cause requires.
