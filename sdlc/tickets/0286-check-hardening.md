# 0286 — Harden line width, release gate labels and a timing test

Status: COMPLETE.
Landed: c17320f3ee3eaaeebdb42004aeb6c896641c19e7

Milestone: 5

## Outcome

The ratchet check fails a Rust line over its width limit that rustfmt left unwrapped, after the existing long SQL lines are split. The release gate prints only checks that ran and fails without Python `assert`, so `python -O` cannot skip it. The HTTP close-time test asserts a bound that holds under coverage and parallel load.

- Why: the milestone 4 code review found 106 product and 336 test lines over 160 characters that pass rustfmt and the line-count ratchet, static pass labels in `release/gate.py` and `container_check.py`, `assert`-based smoke checks, and a wall-clock test that can fail under load.
- Starts from: `store/pool.rs` lines 166 to 316, `bundle/store.rs` lines 130 to 188, `cache.rs` lines 238 to 278, the ratchet tests, `release/gate.py` line 74, `release/container_check.py` line 29, `release/smoke.py` and `tests/http_limits.rs` lines 97 to 108.
- Keeps: every existing check and its targets.
- Changes: the ratchet, the long lines, the release scripts and the one test.
- Proof: the ratchet fails on a planted long line, the gate fails with `python -O` on a planted failure, and the HTTP test passes under the coverage job.
- Defers: nothing.
