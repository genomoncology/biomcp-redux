# 1225 — trial-obsolete-nct-alias

Status: COMPLETE.
Landed: 00ebc15ec54854274ee7c465960988799e1e1100

Milestone: 10

## Outcome

`get trial <NCT>` and `search trial --id <NCT>` follow an obsolete NCT number to the study that replaced it, and state the redirect.

## Why

A reader who holds an older NCT number from a paper gets a refusal or a clean zero today.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get trial NCT01141231` returns `status: failed`, source reason `rejected`; `biomcp search trial --id NCT01141231` returns `complete`, total 0. The registry redirects NCT01141231 to NCT02589938, and its `AREA[NCTIdAlias]NCT01141231` query returns that study.
- About 0.6% of 1,500 sampled registry records carry an alias.

## Proof

- Proof: outside-in cases on recorded registry replies: both commands return NCT02589938 with a notice naming the alias.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ctgov/search_query.rs`, the trial get path in `crates/biomcp-adapters/src/ctgov/` and `crates/biomcp-query/src/trial/`.

## Keeps

- Keeps: current NCT numbers behave as today; an unknown number still says not found.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-adapters/src/ctgov/`, `crates/biomcp-query/src/trial/`, trial cases.
