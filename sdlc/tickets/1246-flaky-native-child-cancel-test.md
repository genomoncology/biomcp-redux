# 1246 — flaky-native-child-cancel-test

Status: COMPLETE.
Landed: 4019f100c33f94b5d9c165e5cb95eba33e88adac

Milestone: 10

## Outcome

`release.test_mutations.MutationCommands.test_cancellation_stops_separate_native_child_groups` passes or fails only on the behaviour it tests, on every machine that runs core gates, so an unrelated ticket's gate never fails on it.

## Why

PM ruling 2026-11-28: the test failed in landing gates for two unrelated tickets on the M5, so it is flaky, not a regression in those tickets. Find and fix the cause. Do not skip, retry or loosen the test, and do not widen its timeout to hide the cause.

## Evidence

- Job j48814 (ticket 1218, commit 0c98747cb, machine m5, 2026-11-28): `release/test_mutations.py` line 215 in `assert_owned_children_stop`, `self.assertEqual(runner.wait(timeout=15), 143 if cancel else 0)`: `AssertionError: 1 != 143`. The cancelled runner exited 1 instead of 143 (terminated by SIGTERM). The ticket's change touches only PGx search.
- Job j23615 (ticket 0457, commit d3c0f6905, machine m5, 2026-10-31): the same test failed in `assert_owned_children_stop`.
- Rerun of the 1218 gate on the same commit: job j48818.

## Proof

- Proof: the cause is named and shown (for example a race between the signal and the runner's own exit path), and the test passes in a loop of repeated runs on the M5 through `pm job`, with the loop count and job reported.

## Starts from

- Starts from: `release/test_mutations.py` lines 200–232 and the runner code it exercises.

## Keeps

- Keeps: the test's assertion that a cancelled runner stops its separate native child groups and exits 143.
