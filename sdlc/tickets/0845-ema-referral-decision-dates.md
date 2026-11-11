# 0845 — ema-referral-decision-dates

Status: COMPLETE.
Landed: ef3b7c892bc5029f9c18f4fba17cb987f132819e

Milestone: 8

## Outcome

For an EMA product whose latest procedure is a referral, BioMCP reports the European Commission decision date from EMA's referrals file, joined on the procedure reference (referrals `EMA/REF/0000287477` matches the product's `REF/0000287477`), labelled as the Commission decision and with its source. Where no referral matches, the reply keeps 0839's latest-procedure date and notice.

## Why

0839 stopped BioMCP labelling EMA's latest-procedure date as the Commission decision, but agents still cannot get the true date: tecovirimat's decision was 29/05/2026, a day after the date BioMCP reports. In `biomed-agent-bench/results/baseline-1.0.md` Sonnet answered this question wrongly in 5 of 6 BioMCP runs; with only a notice, an agent must leave BioMCP for the EMA site to answer it. The 0839 review (2026-11-11) found the referrals file small (about 730 KB) and the join straightforward.

## Proof

- Proof: a routine case under `tests/cases/drug/` with the recorded referrals row and product row for tecovirimat asserts a Commission decision date of 2026-05-29 with its source, and a product without a matching referral keeps the notice.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ema/` as landed by 0839, and how the EMA medicines file is fetched and cached today.

## Keeps

- Keeps: 0839's naming of the latest-procedure date; EMA's own key names in problem reports.

## Defers

- Defers: decision dates for procedures other than referrals, until a source for them is found.

## Changes

- Changes: `crates/biomcp-adapters/src/ema/`, `crates/biomcp-query/src/drug/ema.rs`, `tests/cases/drug/`
