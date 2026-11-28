# 1226 — trial-sponsor-name-match

Status: COMPLETE.
Landed: 0416477d388801b119a8c1752fe7bd26730b10d8

Milestone: 10

## Outcome

`search trial --sponsor <name>` either finds the sponsor's studies under the registry's own spelling, or, when the name as typed matches far fewer studies than a near spelling, says so and names the registry's sponsor names; help's example returns the sponsor it names.

## Why

Help's own example, `--sponsor "MD Anderson"`, returns one study today; the registry lists 3,051 under "M.D. Anderson Cancer Center".

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp search trial --sponsor "MD Anderson"` returns `status: complete`, `total: 1`, no notice. The registry's lead-sponsor query for "M.D. Anderson Cancer Center" returns 3,051.
- Name broadening runs only on zero hits (ticket 0487).
- Every other total compared with the registry matched exactly (613,799 studies; by type, status, phase and results).

## Proof

- Proof: an outside-in case on recorded registry replies: the help example returns a notice with the registry's sponsor names and their counts, or the 3,051-study total, stated as broadened.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ctgov/search_query.rs`, the sponsor broadening from ticket 0487, and the trial help text.

## Keeps

- Keeps: an exact registry sponsor name returns the same total; broadening is always disclosed.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-adapters/src/ctgov/`, `crates/biomcp-query/src/trial/`, help text, trial cases.
