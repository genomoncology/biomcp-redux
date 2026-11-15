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

## Progress

- 2026-11-15, measured live in pm job j41168: EMA's medicines file (2,765 records, timestamp 2026-11-15T06:02:09Z) holds three `opinion_status` values: blank (2,705, including Keytruda and 1,863 other authorised medicines), `Positive` (45) and `Negative` (15). Positive and Negative already read; the blank cell failed text construction and became the `value.unknown` problem.
- The EMA adapter now reads a blank `opinion_status` as no opinion status, as it already reads a blank date cell as no date. A non-text value still reports a problem. After the change, `get drug pembrolizumab markets` reports no problem, and Keytruda's regulatory record has no `kind`.
- Proof: `tests/cases/drug/ema-opinion-status.yaml` with replies in `tests/replies/drug/ema-opinion-status/` (EMA trimmed to Keytruda, Itvisma Positive, Syfovre Negative; Drugs@FDA unchanged; WHO PQ synthetic from the markets case).
- Seen, not changed: blank `revision_number` (833 records), `marketing_authorisation_developer_applicant_holder` (352), `therapeutic_indication` (440) and `latest_procedure_affecting_product_information` still report `value.unknown` (Syfovre shows two of them). Keytruda has none of these blank.
