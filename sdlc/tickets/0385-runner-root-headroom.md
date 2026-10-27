# 0385 — Move variant trial case registration out of the runner root

Status: OPEN.

Milestone: 5

## Outcome

The `runner::variant_trials` aggregation moves into the existing `variant::trials` module, so the test runner's root file drops about 26 lines below its 600-line limit. Sibling modules and case order stay as they are.

- Why: a fresh RCA (coordinator m10791 ask 8, logged in 0060) found that 0351 and 0366 each failed a job on the root file's 600-line limit, and both local fixes leave the root at exactly 600, so the next added case fails again.
- Starts from: the runner root, `runner::variant_trials` and `variant::trials`.
- Rules: two files; existing ratchets unchanged; no new framework or gate.
- Proof: `case --list` output is identical before and after; the landing gate passes; the root is at least 20 lines under its limit.
- Changes: the runner root and `variant::trials`.
- Defers: nothing.
