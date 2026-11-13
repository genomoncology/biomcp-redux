# 0871 — drug-label-name-lookup-fits-the-time-limit

Status: OPEN.

Milestone: 7

## Outcome

A drug label read by a many-label generic name (atorvastatin, rosuvastatin, quetiapine, metformin) finishes well inside openFDA's 15-second call limit on a cold cache, instead of sometimes failing as `deadline-exceeded`.

## Why

0866's developer measured, live on 2026-11-12, that once the size limit is lifted these lookups take 8 to 12 seconds cold (metformin 8.0 s, atorvastatin 9.3 s, rosuvastatin 12.0 s), and atorvastatin's first run failed with `deadline-exceeded` (j38824; pass j38829). Almost all of it is waiting on openFDA's sequential 50-label pages; a cached repeat takes 0.1-0.25 s.

## Proof

- Proof: first measure cold timings for the ten drugs in 0866's table on main after 0866, then after the change; put both tables here.
- Proof: a routine case shows the lookup still returns the same chosen label, total and reason line as 0866's case.

## Starts from

- Starts from: 0866's measurement table and case `tests/cases/drug/label-many-names.yaml`; the name lookup's page loop in `crates/biomcp-query/src/drug/`; openFDA's pacing in `crates/biomcp-adapters/src/quotas.rs`.
- PM decision: 2026-11-12. Prefer fetching the pages in parallel within openFDA's published rate limit; a longer openFDA call limit only if parallel pages can't get under it, and then made configurable with a default.

## Keeps

- Keeps: 0861's choice rule; openFDA's published rate limits; totals.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `crates/biomcp-adapters/src/`, `tests/cases/drug/`
