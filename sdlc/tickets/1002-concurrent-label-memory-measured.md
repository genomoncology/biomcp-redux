# 1002 — concurrent-label-memory-measured

Status: OPEN.

Milestone: 7

## Outcome

The hosted server's peak memory with all its call slots reading large drug labels at once is measured and stated. If it exceeds a stated share of a hosted task's memory, it is bounded.

## Why

Finding E2 of `sdlc/records/2026-11-14-adversarial-review.md` was deferred "before hosted deployment, measurement first". Its static estimate is about 0.7 to 1 GiB transient at 8 calls, from 32 MiB label pages held in several copies. The estimate was never measured. Hosting is now close to launch.

## Proof

- Proof: one `pm job` runs the existing load harness, or a runner test, over hosted HTTP. It runs 8 concurrent `get drug` label calls on the largest recorded labels and records peak resident memory. The numbers and the command go in Progress and in `docs/configuration.md`, so hosting can size its tasks.
- Proof: only if the peak is too high, a bound (for example, how many label reads may run at once) is added with a test. Otherwise, Progress says why none is needed.

## Starts from

- Starts from: `openfda_label/mod.rs:37`, `crates/biomcp/src/config.rs:19`, the load harness under `scripts/` or `crates/biomcp/tests/`.

## Keeps

- Keeps: reply shape and caps.

## Defers

- Defers: nothing.

## Changes

- Changes: a measurement record, docs, and a bound only if one is needed.
