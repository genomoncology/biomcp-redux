# 1127 — restore-test-guards-weakened-by-recent-landings

Status: OPEN.

Milestone: 7

## Outcome

Every check that a landing on 2026-11-20 or 2026-11-21 loosened without its ticket asking for it guards what it guarded before, or carries a recorded PM ruling beside it. Behaviour does not change except where a restored guard exposes a real fault, which is then fixed or reported.

## Why

The PM's check for changed tests missed files under `crates/*/tests/` for two days (its pathspec matched only the folder name). A read-only review of the 18 landings with changed Rust tests, 2026-11-21, found eight that loosened a check. The PM rules on each below.

## Proof

Each item is proved by its restored check passing on the fix and, for items 1 to 3, failing when the guarded behaviour is broken on purpose (say how you showed that).

1. Restore: `crates/biomcp/tests/runner/other_source_value_proof.rs` (landed with 1076, `e812d87b3`, near line 281). The malformed-stage case must again prove the indication row survives (`records/0/drug/id` or the equivalent row field after 1076's move), not only that the shared identity is present. 1076 required keeping every fact.
2. Restore: `crates/biomcp/tests/runner/first_read.rs` (landed with 1070, `e4e07afa7`, near line 165). An over-limit first read must again assert the source state is `syncing` and offers `sync fda-cdx`, beside the new checks of cause, size and retry delay.
3. Restore: `crates/biomcp-types/tests/diagnostic_contracts.rs` (landed with 1084, `e19239f28`, near line 482). Both row kinds again exercise PMA, K and DEN device IDs and NDA and BLA applications.
4. Ceiling: `crates/biomcp/tests/runner/research.rs` (landed with 1095, `0ffd234eb`, near line 221). Set the trial search reply ceiling to the measured size on current main plus the repo's usual room, not 9,535 bytes. PM ruling: growth for the acronym and official title fields is accepted; unexplained room is not.
5. Ceiling: `crates/biomcp/tests/runner/typed_schemas.rs` (landed with 1086, `de6045b31`, near line 214). Set the `search_drug` definition ceiling to its measured size on current main plus the repo's usual room, with a comment naming this ruling. PM ruling: mechanism-of-action growth is accepted.
6. Exception: `crates/biomcp/tests/ratchets/dependencies.rs` (landed with 1090, `f2b828cbf`, near line 54) allows `caseless` in the adapters crate. PM ruling: allowed if its licence passes `cargo deny` and it is used only for case-insensitive phrase matching; record that reason beside the entry. If the standard library can do the matching the ticket needs, remove the dependency instead.
7. Exception: `crates/biomcp/tests/ratchets/adapter_values.rs` and `adapter_value_audit.json` (landed with 1067, `32064e6b2`) allow a date parse in `fda_dsc/html.rs` that drops its error with `.ok()`. Fix the code so a bad date is reported, and remove the exception.
8. Exceptions: `crates/biomcp/tests/ratchets/source.rs` (landed with 1070, near line 14) exempts all of `bundle/first.rs` from the reasoned-suppression rule, and `crates/biomcp/tests/ratchets/placement.rs` (landed with 1066, `48c145ab6`, near line 484) allows inline tests anywhere in `label_sections_tests.rs`. Narrow each to the specific lines or items that need it, or move the tests to where the rule puts them, and remove the file-wide exception.

## Starts from

- Starts from: each landing commit named above; diff its first parent against it for the file named.

## Keeps

- Keeps: every behaviour on main today. This ticket changes tests and the one `.ok()` site, nothing else, unless a restored guard fails.

## Defers

- Defers: nothing.

## Changes

- Changes: the test and ratchet files named, `adapter_value_audit.json`, and `crates/biomcp-adapters/src/fda_dsc/html.rs`.
