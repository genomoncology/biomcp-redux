# 0870 — get-de-novo-records-by-den-id

Status: COMPLETE.
Landed: 7e666371a2b545b95a2c1bfde1d8d7c9c81a7885

Milestone: 7

## Outcome

`get diagnostic DEN170058 --source fda-510k` returns the De Novo record that openFDA's 510(k) data holds, so any ID a 510(k) search returns can be read back.

## Why

Bench 0040's rerun (failure #14 in `sdlc/records/2026-11-12-persona-rerun-failures.md`, reg-affairs log :108): the command exits 2 as a usage error on `id`, though openFDA holds DEN170058 (MSK-IMPACT, decision code DENG).

## Proof

- Proof: a routine case with a recorded reply gets DEN170058 and returns its record; a K-number still works.

## Starts from

- Starts from: the fda-510k ID check and adapter in `crates/biomcp-adapters/src/device_search.rs` and the diagnostic ID parsing.

## Keeps

- Keeps: K-number reads unchanged.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-adapters/src/`, `crates/biomcp-query/src/diagnostic/`, `tests/cases/diagnostic/`
