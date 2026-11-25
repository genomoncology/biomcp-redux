# 1187 — linux-release-tests-in-landing-gate

Status: COMPLETE.
Landed: 9e676fe2b0d1d4101d2eaeef6e97a7a0887a31ca

Milestone: 7

## Outcome

If `python3 -m unittest release.test_linux` (and the other `release/test_*.py` suites that run without network or Docker) costs a few minutes or less on the gate machine, the landing gate runs it. If it costs more, nothing changes in the gate and a pm idea records running it at release qualification, with the measured time.

## Why

1177 broke `release.test_linux` and the landing gate passed, because the gate does not run the release tests (fixed by 1183). The coordinator set the threshold: a few minutes or less.

## Proof

- Proof: Measured run time of the release test suites through `pm job` on the gate machine; if added, a gate run shows the stage.

## Starts from

- Starts from: `Makefile` landing-gate target, `release/test_*.py`.

## Keeps

- Keeps: The gate's other stages and their order.

## Defers

- Defers: Release tests that need Docker or network stay at release qualification.

## Changes

- Changes: Makefile, or a pm idea only.
