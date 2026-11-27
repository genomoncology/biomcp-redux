# 1234 — faers-drug-role-stated

Status: OPEN.

Milestone: 7

## Outcome

`search adverse-event <drug>` states in help and in the reply that FAERS counts include reports where the drug is listed in any role, and offers a filter for reports where the drug is the suspect.

## Why

A FAERS count is read as "reports about this drug"; today it includes reports where the drug was only concomitant, and nothing says so.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp search adverse-event pembrolizumab` returns 109,152 reports with no statement of drug role; report 27325180 in that set lists 68 drugs.
- There is no filter for role, country, reporter or outcome. The suspect-only count could not be measured, because the census reads FDA sources through the product only.

## Proof

- Proof: outside-in cases on recorded FAERS replies: the reply's `query` echoes the role counted; a suspect-only filter returns a smaller total; help states the default.

## Starts from

- Starts from: `crates/biomcp-types/src/adverse_event/faers.rs`, `crates/biomcp-adapters/src/faers/`.

## Keeps

- Keeps: the default total is unchanged unless the PM rules the default should be suspect-only.

## Defers

- Defers: MAUDE product problem codes and patient outcome on `get`.

## Changes

- Changes: `crates/biomcp-adapters/src/faers/`, `crates/biomcp-types/src/adverse_event/`, help text, cases.
