# 0918 — ema-opinion-status-values-known

Status: OPEN.

Milestone: 7

## Outcome

The EMA markets row for an authorised medicine reads its opinion status without a `value.unknown` problem.

## Why

In the pharmacovigilance persona's round 2 (bench `personas/pv-scientist/runs/2026-11-13-r2/`, log.md:104), `get drug pembrolizumab markets` returned the problem `{"source":"ema","record":"EMEA/H/C/003820","field":"opinion_status","code":"value.unknown"}` for Keytruda. The field is read at `crates/biomcp-adapters/src/ema/map.rs:148`. A regulatory reader should not see a problem on a long-authorised medicine.

## Proof

- Proof: a recorded EMA case for Keytruda (EMEA/H/C/003820) with no opinion_status problem and the source's real value shown. Check the live file for every opinion_status value it holds, and a case for each value the mapping did not read before.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ema/map.rs:148,183-189`.

## Keeps

- Keeps: a problem for values that really are unknown.

## Defers

- Defers: nothing.

## Changes

- Changes: the EMA adapter, `tests/cases/drug/`.
