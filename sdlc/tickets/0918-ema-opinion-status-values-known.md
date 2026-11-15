# 0918 — ema-opinion-status-values-known

Status: COMPLETE.
Landed: edc224ba83154ab28e4bb0e37b22ea95d640d621

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
- PM asked to fold in the other blank EMA cells, same root cause. Live counts in the 2026-11-15 medicines file (2,765 records; no key missing from any record, no non-text value anywhere): blank `revision_number` 833, `marketing_authorisation_developer_applicant_holder` 352, `therapeutic_indication` 440, `latest_procedure_affecting_product_information` 668, `active_substance` 93, `international_non_proprietary_name_common_name` 107, `atc_code_human` 532, `pharmacotherapeutic_group_human` 563, `therapeutic_area_mesh` 467, and dates such as `marketing_authorisation_date` 731 and `european_commission_decision_date` 529. The referrals file (593 records) has 44 blank `european_commission_decision_date` cells and no blank `reference_number`.
- One shared rule now: the EMA field reader (`crates/biomcp-adapters/src/ema/raw.rs`, `field`) reads a blank cell as absent for every medicines and referrals field. The five per-field blank-date filters it replaces are removed. A non-text value still reports a problem, and a blank required field (name, category, medicine status) still reports one as missing. The referral's `reference_number` key keeps its previous read, so a blank one would still be accepted as before rather than refusing the file.
- The case now also expects Syfovre (blank revision number and latest procedure reference) to have no problems, no `attribution.version` and no latest-procedure reference.
