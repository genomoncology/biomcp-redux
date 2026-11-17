# 1038 — replay-clock-reaches-every-command

Status: OPEN.

Milestone: 7

## Outcome

Every command that reads a source runs on 1035's replay clock, legs read one after another inside a worker add up, and `Leg::dependency` keeps the looked-up source's own time limit live.

## Why

1035's review (2026-11-17) accepted it with these gaps:
- Commands started from the plain deadline get no command clock: gene get (`crates/biomcp-query/src/gene/get.rs:20`), drug aliases, resolve and markets, gene search, health. Nothing checks that new code uses the command deadline.
- Two legs used one after another inside one worker each start at the command's time (`crates/biomcp-fetch/src/deadline.rs:79-83`, `124-132`), e.g. a variant section `fan_out` job.
- `Leg::dependency` (`crates/biomcp-fetch/src/lib.rs:512-515`) no longer applies the source's own limit; a user-set PubMed or PMC OA limit under 3 s is not enforced there, live.
- Drop folds worker time only on the command's own thread (`deadline.rs:124-131`, `352-362`); fold unconditionally.

## Proof

- Proof: a replayed gene get case shows HGNC then section time adding, failing on the base commit; a check fails on any new command that bypasses the command deadline; a test shows `Leg::dependency` honours a configured source limit below its cap.

## Starts from

- Starts from: 1035's clock in `crates/biomcp-fetch/src/deadline.rs`.

## Keeps

- Keeps: live deadlines and pacing.

## Defers

- Defers: nested commands sharing an outer command's clock.

## Changes

- Changes: the listed commands' deadline start, worker leg chaining, `Leg::dependency`, one check and cases.
