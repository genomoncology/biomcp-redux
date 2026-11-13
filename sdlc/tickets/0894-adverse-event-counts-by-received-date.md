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

## Progress

- Measured openFDA live (jobs j39359, j39384, j39406, 2026-11-13). `count=receivedate` returns every daily bucket of the searched range, oldest first, with no cap: aspirin 2004-2025 gave 7,359 buckets in one reply, summing to 846,202, the same as the dated search total. Sending `limit` above 100 without an API key is refused with 403 API_KEY_MISSING, and `skip` is refused with count. So BioMCP sends no `limit` and asks once for the whole range. The brief's "capped at 1000 by default" did not hold.
- The pv-scientist agent's `+` finding: a literal `+` inside the range, percent-encoded to `%2B`, breaks openFDA's range parse (our probe got a 500 parse error; the agent saw 0 terms). BioMCP builds the count request with the same serializer as the search (spaces as `%20`), and the recorded case shows nonzero counts equal to the search totals.
- Added `--aggregate received-year` and `--aggregate received-month` for FAERS and MAUDE. They need `--received-from` and `--received-to`, which now also accept a bare year (`2021`). The count uses the search's own expression, so FAERS widened names and the reaction, seriousness and MAUDE filters apply. The reply's `received_counts` lists every period of the range, oldest first, with zeros; a period the range only partly covers, or that ends after the source's `last_updated`, is marked `partial`. MedDRA is needed only with `--reaction`; where MedDRA is off, `--aggregate` stays offered without `reactions`.
- Recorded case `tests/cases/adverse-event/received-counts.yaml`: pembrolizumab + myocarditis 2021-2026 gives 142, 198, 319, 433, 512 (the run's five dated totals, each matched by a recorded dated search) and 365 for 2026, marked partial (openFDA last updated 2026-10-28). Also a month case with a partial first month, a MAUDE case (insulin pump 2024-2025: 10,022 and 12,103) and two usage refusals.
