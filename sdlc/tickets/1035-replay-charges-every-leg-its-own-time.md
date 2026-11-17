# 1035 — replay-charges-every-leg-its-own-time

Status: OPEN.

Milestone: 7

## Outcome

Every command that runs a source leg charges that leg's replay time to the command's replay clock, so a replayed case takes the same path live would and replay cases catch a change that makes a call too slow for its live limit.

## Why

1034's review (2026-11-17) found about 100 sites ending with `command.finish(deadline.remaining())` where the leg's request time is never added to the command deadline, so replay runs faster than live. Example: recall search (`crates/biomcp-query/src/drug/recall.rs:81`) leaves about 1.5-2 s of alias-walk recall queries uncharged. A replay miss still fails loudly, so no wrong answer ships, but replay can pass a call that would time out live.

## Proof

- Proof: a replayed case whose live run nears its limit (for example the recall alias walk) shows replay's elapsed time matching live's within pacing, and fails on the base commit; a check or test lists no remaining site that finishes a command without its leg's time.

## Starts from

- Starts from: 1034's `Leg::command_remaining` and the `command.finish` sites.

## Keeps

- Keeps: live deadlines and pacing.

## Defers

- Defers: nothing.

## Changes

- Changes: the finish sites, one case, and a doc note on `Leg::sibling`/`Leg::dependency` that `command_remaining` measures against the leg's opened end.
