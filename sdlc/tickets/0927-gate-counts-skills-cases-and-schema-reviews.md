# 0927 — gate-counts-skills-cases-and-schema-reviews

Status: OPEN.

Milestone: 7

## Outcome

The landing gate's changed-crate library tests (0914, 0921) treat a change to `skills/` or to the example cases in `tests/cases/` as a change to `biomcp`, and a change to `release/schema-reviews.json` as a change to `biomcp-types`, so the library tests that read those files run.

## Why

0921's review (2026-11-14): these files are read by library tests that the gate otherwise skips. `release/schema-reviews.json` is compiled into `biomcp-types` tests (`crates/biomcp-types/tests/schemas/policy.rs:138-140`), and `schema-release-check` doesn't run that test. `skills/` and the example cases are read by `crates/biomcp/build.rs` (lines 246 and 326) and checked by `biomcp`'s own tests, such as `src/help.rs:221` reading `examples.json`. `scripts/test_gate_stages.py:150` asserts that a `tests/cases/` change selects no crate, which locks the gap in. The skill change in 0922 is the next ticket this would protect.

## Proof

- Proof: `scripts/test_gate_stages.py` asserts a `skills/` change and a `tests/cases/` change select `biomcp`, and a `release/schema-reviews.json` change selects `biomcp-types`; the assertion at line 150 is changed.

## Starts from

- Starts from: `scripts/changed_library_tests.py` as 0921 left it; 0921's review.

## Keeps

- Keeps: 0921's selection rules and fallback.

## Defers

- Defers: running `crates/biomcp/tests/tools.rs` (0921's open question).

## Changes

- Changes: `scripts/changed_library_tests.py`, `scripts/test_gate_stages.py`.
