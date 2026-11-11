# 0826 — trial-search-rows-carry-sponsor-and-dates

Status: OPEN.

Milestone: 8

## Outcome

Each ClinicalTrials.gov row from `search trial` carries the lead sponsor and the start and primary-completion dates the registry returns.

## Why

Agents open each trial to learn who runs it and when; the fields are in the search response already, so this removes follow-up calls.

## Proof

- Proof: a routine case under `tests/cases/trial/` asserts the three fields on a recorded search page; the trial page byte ceiling still passes.

## Starts from

- Starts from: the search row mapping under `crates/biomcp-adapters/src/ctgov/` and the trial header type in `crates/biomcp-types/src/`.

## Keeps

- Keeps: every existing field, schema ID rule (a changed reply schema takes a new major), source attribution and page behavior; no new source, key or dependency. Row order, totals and continuation tokens are unchanged.

## Defers

- Defers: NCI rows wait for a request.

## Changes

- Changes: `crates/biomcp-adapters/src/ctgov/`, `crates/biomcp-types/src/`, `tests/cases/trial/`
