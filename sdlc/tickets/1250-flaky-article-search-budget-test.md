# 1250 — flaky-article-search-budget-test

Status: OPEN.

Milestone: 10

## Outcome

`search_and_hydration_share_one_budget_while_pool_age_uses_wall_time` in `crates/biomcp-query/tests/article_search.rs` passes or fails only on the behaviour it tests, on every machine that runs core gates and under gate load, so an unrelated ticket's gate never fails on it.

## Why

PM ruling 2026-11-28: one-off gate failures that nobody tracks become next week's random red gates. This test has failed in the gates of five unrelated tickets on two machines, and passed six times in a row on the same commit when run alone, so it is flaky under load, not a regression. Find and fix the cause. Do not skip, retry or loosen the test, and do not widen a timeout to hide the cause.

## Evidence

The failure each time is at line 189: `assertion left == right failed`, `left: Some(Cache(Busy))`, `right: None`.

- j49080: ticket 1247, commit a858974c7, m5, 2026-11-28, machine load about 8 of 18 cores.
- j47028: ticket 1176, commit 8d44830df, blue, 2026-11-25.
- j43254: ticket 1059, commit c488555e1, m5, 2026-11-19.
- j32817: ticket 0691, commit a08538971, m5, 2026-11-06.
- j23035: ticket 0456, commit b8e83e043, m5, 2026-10-30.
- j49097: the same test run six times in a row alone on a858974c7, on m5, passed every time.

## Proof

- Proof: the cause is named and shown (for example a shared cache lock or a wall-clock race the test depends on), and the test passes in a loop of repeated runs alongside the rest of the test binary under load on the M5 through `pm job`, with the loop count and job reported.

## Starts from

- Starts from: `crates/biomcp-query/tests/article_search.rs` lines 169–190 and the article search cache and budget code it exercises.

## Keeps

- Keeps: what the test asserts: search and hydration share one budget, pool age uses wall time, and no cache refusal is reported when none is due.
