# 0252 — Report live result churn as information in the nightly live check

Status: OPEN.

Milestone: 1

Depends on: 0068

## Outcome

The nightly live check reports dependent request changes that follow from changed successful discovery results as churn, and churn alone does not fail the run. Exhausted rate limits are reported as throttled cases, counted separately, and do not fail the run alone. Schema, report and invariant defects, unexpected status failures and errors still fail the run.

## Why

Scheduled run j14800 failed on 12 drift cases, and the bounded review found no product defect: ten were live search churn, one was Semantic Scholar throttling and one was HGNC data change.

## Proof

- Proof: a fixture report pair whose discovery reply changed and whose dependent request set changed exits zero with the case listed as churn. A fixture with a changed request plan and an unchanged discovery reply still fails. A fixture whose dependent set differs only by concurrency order shows no churn. A fixture with 429 after bounded retries lists the case as throttled and exits zero. A fixture with a 500 or a schema failure still fails.

## Starts from

- Starts from: `scripts/nightly_live_check.py`, the landed 0180 drift witnesses, and the j14800 report on the evidence store at `0068-scheduled/20261026T080118Z-owirvcp0`.
- PM ruling 2026-10-26: the coordinator review (m6851) classified eight article cases as conclusive discovery churn, `article/fold/live-rank` and `article/search/gene-terms` as probable churn with witnesses cut at the 1 MiB bound, `article/sections/grammar` as Semantic Scholar 429 after bounded retries, and `gene/retired` as HGNC data change. Accept its recommendation.

## Keeps

- Keeps: keyless isolated settings, the 1 MiB witness bound, bounded budgets, no recording writes and no baseline refresh. A witness cut at the bound is reported as uncertain, and it is never called churn by assumption.

## Defers

- Defers: giving the nightly check a Semantic Scholar key, which waits on the Semantic Scholar license ruling. Refreshing baselines waits for a separate ruling.

## Changes

- Changes: `scripts/nightly_live_check.py`, its report schema and its fixtures.
