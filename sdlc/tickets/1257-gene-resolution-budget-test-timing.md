# 1257 — gene-resolution-budget-test-timing

Status: OPEN.

Milestone: 10

## Outcome

The test in `crates/biomcp-query/tests/gene_resolution.rs` that runs under a 1.5-second budget (line 150) leaves enough real time for local store work that it cannot fail under gate load, while still proving what it asserts.

## Why

Found while fixing 1250: BioMCP limits local store work in real wall-clock time from what is left of the command's budget, and tests that replay recorded delays on a simulated clock under budgets of a second or two leave only milliseconds of real time for that work. The article search tests with that shape failed under load (1250, jobs j49112 and j49124); this one has the same shape and has not yet been seen failing. Fix it before it turns red at random.

## Proof

- Proof: the test passes 100 times in a loop of its whole test binary on the M5 under added CPU and disk load through `pm job`, with the loop count and job reported, and its assertions are unchanged.

## Starts from

- Starts from: `crates/biomcp-query/tests/gene_resolution.rs` line 150 and the 1250 fix in `crates/biomcp-query/tests/article_search.rs`, which scaled simulated times so the budget leaves seconds of real time.
