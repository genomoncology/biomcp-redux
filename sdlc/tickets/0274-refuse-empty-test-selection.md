# 0274 — Fail a test run whose filter selects no tests

Status: OPEN.

Milestone: 1

## Outcome

A case runner or contract test run given a name filter that selects zero tests exits nonzero and names the filter, instead of reporting success. A run with no filter, and a filter that selects at least one test, behave as they do today.

- Why: three jobs (0265 j15319, 0262 j15403 and 0266 j15531) reported a passing phase that ran zero tests because the filter was rebuilt by hand, so a reviewer read untested work as proved.
- Starts from: `crates/biomcp/tests/cases.rs` and its `libtest_mimic::run` call, the other libtest-mimic harnesses in the workspace, the 0060 line for zero-selected phases, and the coordinator RCA in m7554.
- Keeps: every existing filter that matches, skip and ignore behavior for selected tests, and test output. No new framework.
- Changes: each libtest-mimic harness's filter handling, and the runner README with one sentence on the rule.
- Proof: a harness case runs the binary with a filter that matches nothing and asserts a nonzero exit and the filter in stderr; a matching filter still passes. No source calls.
- Defers: plain `cargo test` harnesses outside libtest-mimic, which keep cargo's behavior.
