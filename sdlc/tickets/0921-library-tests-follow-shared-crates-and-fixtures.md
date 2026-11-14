# 0921 — library-tests-follow-shared-crates-and-fixtures

Status: OPEN.

Milestone: 7

## Outcome

The landing gate's changed-crate library tests (0914) also run the tests of every crate that depends on a changed crate, and a change under the shared top-level test data (`tests/fixtures/`, `tests/replies/`, `tests/native/`) runs the library tests that read it. A change to `biomcp-types`, `biomcp-fetch` or `biomcp-test-support` alone runs its dependents' library tests, and the gate log names them.

## Why

0914's review (2026-11-14) found that a change to a shared crate runs only that crate's own tests: every crate depends on `biomcp-types`, and `biomcp-test-support` is the helper every crate's tests use. Eight library test files read the shared test data, which selects no crate. So a change can break another crate's library tests and still land, which is what 0914 set out to stop.

## Proof

- Proof: the gate-stage unit test (`scripts/test_gate_stages.py`) covers a `biomcp-types`-only change selecting its dependents, a `tests/replies/` change selecting the crates that read it, and a `biomcp-test-support` change selecting every crate; one gate job on the branch shows the selection in its log.

## Starts from

- Starts from: `scripts/changed_library_tests.py:26-45`, `Makefile:104-105`; the crate graph from `cargo metadata` or a fixed table; `crates/biomcp-test-support/src/fixtures.rs` and direct readers such as `crates/biomcp-query/tests/gene_resolution.rs:38`.
- Cost bound: the all-crates library tests took 261 s (j40034).

## Keeps

- Keeps: 0914's behaviour for a ticket's own crate, the all-crates fallback when no base is found, and passing a failing cargo exit through.

## Defers

- Defers: running `crates/biomcp/tests/tools.rs`, which neither the gate nor `make library-tests` runs; fix the comment at `scripts/changed_library_tests.py:44` that says biomcp's other targets are only cases and ratchets, and note `tools.rs` here for a later decision.

## Changes

- Changes: `scripts/changed_library_tests.py`, `scripts/test_gate_stages.py`, and if needed `Makefile`.

## Progress

`scripts/changed_library_tests.py` now reads the crate graph from `cargo metadata --no-deps` in the job's checkout. A changed crate selects itself, every crate whose library builds on it (followed through normal and build dependencies), and every crate whose tests use one of those (dev dependencies, one step, as cargo builds them). A change under `tests/fixtures/`, `tests/replies/` or `tests/native/` counts as a change to `biomcp-test-support`, whose `fixtures()` serves that data: every crate's tests use that helper, and the direct readers (`biomcp-query` tests, `biomcp` bin tests) use it too, so picking files one by one would be fragile and gain nothing. If `cargo metadata` fails, every crate runs. On today's graph a change to `biomcp-types`, `biomcp-fetch`, `biomcp-test-support` or the shared test data selects all six crates; `biomcp-adapters` selects adapters, query and biomcp; `biomcp-query` selects query and biomcp; `biomcp` selects itself. The comment at `changed_library_tests.py` on biomcp's other targets now says the gate runs `cases` and `ratchets`, and that `tools` (`test = false`) runs in neither the gate nor `make library-tests`, left for a later decision.

`scripts/test_gate_stages.py` covers these against real Git with a stub cargo that returns the workspace's real crate graph: a `biomcp-types`-only, `biomcp-fetch`-only and `biomcp-test-support`-only change each select every crate; each of the three shared-data folders selects every crate; adapters and query select only their dependents; `tests/cases/` and docs select none; a failed `cargo metadata` runs every crate; 0914's cases still hold.

Gate j40185 on the M5 at 751e01776 (`make --keep-going landing-gate docs-check schema-release-check`) passed. It ran with `LIBRARY_TESTS_BASE=6ae2c585a`, the commit before 0914, whose only crate change is a `biomcp-types` test, so the diff stands for a `biomcp-types`-only change read through the real `cargo metadata`. Its log says `crates=biomcp,biomcp-adapters,biomcp-fetch,biomcp-query,biomcp-test-support,biomcp-types` and names both cargo commands. Stage times: pre-job 232 s, cases 166 s, changed-library-tests 172 s (all six crates, every test passed). So a shared-crate or shared-data change adds about 3 minutes to the gate, the same as 0914's all-crates run; a change to biomcp alone still costs only its bin tests. Without the override this branch changes no crate and selects none.
