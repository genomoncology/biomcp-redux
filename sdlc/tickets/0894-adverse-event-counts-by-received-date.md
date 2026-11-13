# 0894 — adverse-event-counts-by-received-date

Status: OPEN.

Milestone: 7

## Outcome

A FAERS or MAUDE search can return counts by received year (and by month), using openFDA's own `count=receivedate` facet, with the same widened names and filters as the search, so a trend slide is one call.

## Why

In the pv-scientist persona run of 2026-11-13 (biomed-agent-bench `personas/pv-scientist/runs/2026-11-13/log.md` and `report.md`, analysis beside them; code-mode run in `runs/2026-11-13-raw/`), the five-year myocarditis trend took five dated searches (log.md:336); the agent's own curl showed openFDA's `count=receivedate` answers in one request. The code-mode agent took 12 calls for the same trend.

## Proof

- Proof: a recorded case for pembrolizumab + myocarditis 2021-2025 where the per-year counts equal the five dated search totals (142, 198, 319, 433, 512 on the run's data, or the recorded day's values) and the widened names apply; a partial current year is marked partial.

## Starts from

- Starts from: `crates/biomcp-adapters/src/faers/`, `maude/`; openFDA `count=` with daily buckets summed to years.

## Keeps

- Keeps: the case search and its paging.

## Defers

- Defers: counts by age, sex, country or reporter type; disproportionality statistics.

## Changes

- Changes: `crates/biomcp-adapters/src/faers/`, `maude/`, `crates/biomcp-query/src/adverse_event/`, `crates/biomcp-types/`, `tests/cases/adverse_event/`, `docs/`
