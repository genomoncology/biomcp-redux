# 0054 — Hold library tests to the listed exceptions

Status: OPEN.

Milestone: 1

## Outcome

Every Rust test outside the runner sits in a file that architecture section 10.4 lists with its reason, and the ratchet test enforces that list for crate `tests/` folders as well as inline `#[test]` in `src/`. Today the ratchet checks only inline tests, so two folders grew outside the list: `crates/biomcp-adapters/tests/` (5 files, 889 lines, 21 tests; the HGNC and MyGene files call themselves temporary) and `crates/biomcp-query/tests/` (2 files, 737 lines, 17 tests). The inline list also still names `crates/biomcp-query/src/article/rank.rs`, which no longer exists.

- Starts from: architecture 5.5 (adapters have no tests of their own) and 10.4; `crates/biomcp/tests/ratchets/source.rs` `INLINE_TESTS`; the files above.
- PM decision: 2026-10-24. For each test in those folders, do one of three things. Delete it when a runner case already proves the same behavior. Move it to a runner case when the binary can reach the behavior through a recording. Keep it only when the binary cannot reach the behavior without a test hook, such as concurrent pool publication, store corruption or decoder limits that need many tiny inputs, and then add the file and its reason to 10.4. Prefer the runner. The temporary HGNC and MyGene contracts are expected to go.
- Keeps: the coverage floor, which may not drop; every behavior those tests prove.
- Proof: the ratchet test lists allowed test files under `crates/*/tests/` and `crates/*/src/`, each with a reason, and fails on any other file holding `#[test]`. One negative ratchet case plants a test file in an unlisted folder and is refused. Architecture 10.4 matches the list. The stale `rank.rs` entry is gone. make landing-gate runs through pm job, and coverage stays at or above the floor.
- Defers: nothing.

## Changes

- `crates/biomcp/tests/ratchets/`: enforce one reasoned file list across source and integration tests, compare architecture, and refuse an unlisted test folder.
- `crates/biomcp/tests/runner/`, adapter/query integration tests and command cases: remove duplicate proofs and move reachable response-boundary tables through the command runner; retain documented inaccessible store/concurrency and API-boundary checks.
- `sdlc/planning/architecture.md`: match the enforced test-file reasons and remove the stale rank exception.
