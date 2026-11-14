# 0914 — library-tests-run-before-landing

Status: OPEN.

Milestone: 7

## Outcome

A ticket's own library tests run before it lands, so a new test can't pass review without running.

## Why

The landing gate (`scripts/landing_gate.py:9`) runs rights, audit, secrets, pre-job, the first-read check, workflow lint and cases. Library tests (`library-tests`) run only in `make full-suite`. On 2026-11-13, 0909's new `crates/biomcp-fetch/tests/record/command_budget.rs` was compiled by clippy, but its gate never ran it. The reviewer had to run it by hand (j39853). This was also flagged earlier in the day: adapter unit tests don't run in the gate.

## Proof

- Proof: a test that changes under a ticket runs before landing, and the gate log names it. Either:
  - the gate runs the library tests of the crates the branch changed; or
  - `pm ticket land` runs `make full-suite`;

  whichever costs less on the M5. Measure both and record the times.

## Starts from

- Starts from: `scripts/landing_gate.py`, `Makefile` (`landing-gate`, `full-suite`, `library-tests`).

## Keeps

- Keeps: the landing gate's speed, within what the measurement shows is worth it.

## Defers

- Defers: nothing.

## Changes

- Changes: `scripts/landing_gate.py` and/or `Makefile`.
