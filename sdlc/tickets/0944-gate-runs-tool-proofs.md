# 0944 — gate-runs-tool-proofs

Status: OPEN.

Milestone: 7

## Outcome

The landing gate runs the case runner's tool-only proofs (`make proof`), so a proof that breaks fails the gate rather than only compiling.

## Why

0939's developer (2026-11-14): the gate and `make full-suite` only lint the tool-only tests (`lint-tools`); a broken proof such as `schema-discovery/starts-within-reads-raw-bytes` would pass the gate. 0939 ran it by hand (j40657).

## Proof

- Proof: a deliberately broken proof on a scratch branch fails `make landing-gate` in a job; the gate's time rises by no more than the measured cost, recorded in Progress.

## Starts from

- Starts from: `scripts/landing_gate.py` (`LANDING` stages), the `proof` target in the Makefile. Time a full `make proof` in a job first; if it is slow, run only proofs whose files changed, as 0914/0921 did for library tests.

## Keeps

- Keeps: gate output format; other stages.

## Defers

- Defers: nothing.

## Changes

- Changes: the landing gate script, Makefile, maybe the runner's proof selection.
