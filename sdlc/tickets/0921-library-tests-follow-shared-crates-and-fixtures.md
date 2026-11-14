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
