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

## Measured

Cold label reads by name (`get drug NAME label --label-section indications_and_usage`, debug build, fresh home, keyless, M5), 2026-11-12. Before: j38859 on this branch's base (0872's tip, whose change affects only the choice, not the pages read). After: j38867. The ten drugs are the seven 0866 measured (j38824, j38829) plus simvastatin, sertraline and gabapentin; every run returned the same label and total before and after.

| Drug | Labels | Before | After |
| --- | --- | --- | --- |
| atorvastatin | 247 | 7.3 s | 5.8 s |
| metformin | 284 | 6.0 s | 4.3 s |
| lisinopril | 159 | 3.1 s | 2.6 s |
| amlodipine | 134 | 3.5 s | 2.7 s |
| rosuvastatin | 176 | 3.5 s | 3.0 s |
| quetiapine | 150 | 4.3 s | 3.6 s |
| aripiprazole | 123 | 3.3 s | 2.7 s |
| simvastatin | 112 | 2.4 s | 2.5 s |
| sertraline | 166 | 4.4 s | 2.7 s |
| gabapentin | 415 | 6.9 s | 3.8 s |

What limits it: openFDA. Metformin's eight 50-label pages (5.6-9.1 MB each) took 0.5-1.0 s apiece and arrived one after another before (6.1 s in all); after, the first page (1.0 s) gives the total and the other seven overlap four at a time within the 4-per-second pacing (3.8 s in all), each taking 0.5-1.4 s. A cached repeat takes 0.06-0.25 s. openFDA's speed varies by day: 0866 measured rosuvastatin at 12.0 s cold where these runs saw 3.5 s, so the gain matters most on slow days, when pages that used to add up now overlap.
