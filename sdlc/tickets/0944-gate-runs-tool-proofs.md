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

## Progress

- Changed: both gates (`make landing-gate` after `cases`, `make full-suite` before `coverage`) run a new `tool-proofs` stage, `make tool-proofs`, which runs the whole `tools --proofs` catalog (29 proofs, 1 ignored) with plain `cargo test --test tools -- --proofs`. `make proof PROOFS=...` is unchanged for focused runs. `scripts/test_gate_stages.py` checks the new stage order and that a failing `tool-proofs` stops the gate.
- Measured: the base gate on main (j40893, 4e99fe4be) took 588 s. A first version ran the catalog through `scripts/trials.py`, which vendors every crate and rebuilds all dependencies against the vendored copy. Its stage took 594 s, about 309 s to build and 285 s to run the proofs (j40909, with the M5 loaded: cases took 320 s against 242 s on main). Plain `cargo test` reuses the cases stage's build, so the stage now builds only the `tools` test binary.
- Proof: with `schema-discovery/starts-within-reads-raw-bytes` deliberately broken on the scratch branch `scratch/0944-broken-proof`, `make landing-gate` failed at `gate stage=tool-proofs ... exit=2` and named the proof ("starts_within read the wrong byte") (j40909). The scratch branch was deleted from origin afterwards.
