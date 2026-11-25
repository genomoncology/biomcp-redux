# 1189 — release-gate-tests-in-landing-gate

Status: COMPLETE.
Landed: 8849bcde4cd135b67246ccb2688f513cbe541c74

Milestone: 7

## Outcome

If `python3 -m unittest release.test_gate` (the installed-CLI release gate tests, left out of the landing gate's `release-tests` stage by 1187 because they failed then) costs a few minutes or less on the gate machine, the `release-tests` stage runs it. If it costs more, the gate is unchanged and the PM records a pm idea to run it at release qualification, with the measured time.

## Why

1188 fixed the stale MCP help reply in these tests, so they pass on main (edb7cb8ab). 1187 set the terms: release tests join the landing gate when they cost a few minutes or less (coordinator, 2026-11-25).

## Proof

- Proof: measured run time of `release.test_gate` through `pm job` on the gate machine; if added, a landing-gate run shows it inside the `release-tests` stage.

## Starts from

- Starts from: `Makefile` `release-tests` stage from 1187, `release/test_gate.py`.

## Keeps

- Keeps: the other gate stages and the 1187 selection.

## Defers

- Defers: tests that need Docker or network stay at release qualification.

## Changes

- Changes: `Makefile` only, or a pm idea.
