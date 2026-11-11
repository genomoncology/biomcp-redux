# 0839 — ema-decision-date-is-the-decision

Status: COMPLETE.
Landed: 1be2a722955acfad5b8a75ec3453d9aec2463e37

Milestone: 7

## Outcome

For an EMA product, the European Commission decision date BioMCP reports is the decision's date, or the field is absent with a notice; the product page's latest-procedure date is never shown under that name.

## Why

In `biomed-agent-bench/results/baseline-1.0.md` (build c2511ee90, which is current main) Sonnet answered the tecovirimat decision date wrongly in 5 of 6 BioMCP runs and 1 of 2 curl runs, a day early, from the field BioMCP labels as the decision date. First reported 2026-11-07 (`biomed-agent-bench/results/full-evaluation.md`, finding 5).

## Proof

- Proof: a routine case under `tests/cases/drug/` with the recorded EMA reply for tecovirimat asserts the field's meaning: either the referral decision date or absence with the notice.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ema/`.

## Keeps

- Keeps: other EMA fields and their source names.

## Defers

- Defers: reading EMA referral records as a source of their own waits for a request.

## Changes

- Changes: `crates/biomcp-adapters/src/ema/`, `tests/cases/drug/`
