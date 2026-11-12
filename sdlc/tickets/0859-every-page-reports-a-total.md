# 0859 — every-page-reports-a-total

Status: OPEN.

Milestone: 7

## Outcome

Every search and section page reports the source's total, or says the source gives none, and a page cut short by a limit says so with its next step; trial locations report their total and accept a larger page.

## Why

From the bench 0039 persona review (2026-11-12, runs on BioMCP 58d83b6da), `personas/<slug>/runs/2026-11-12/review.md` and `log.md`: pt-committee log.md:13, rare-disease log.md:21, msl log.md:15 and 42, comp-bio log.md:47-50: capped pages looked complete; patient-navigator log.md:155 and 450: 230 sites took 24 calls at 10 per page.

## Proof

- Proof: routine cases for an article search, a CIViC section and trial locations show the total and the truncation next step; locations accept a page of 100.

## Starts from

- Starts from: the shared page type and the adapters that drop the native total.

## Keeps

- Keeps: never invent or sum totals; paging contracts.

## Defers

- Defers: distance sort for locations (BioMCP 0083).

## Changes

- Changes: `crates/biomcp-query/src/`, `crates/biomcp-adapters/src/`, `tests/cases/`
