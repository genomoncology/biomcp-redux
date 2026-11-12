# 0858 — empty-and-failed-replies-say-why

Status: OPEN.

Milestone: 7

## Outcome

A reply never comes back empty, partial or failed without saying why: a label section absent from the label says so (not just empty); a section the chosen source lacks is a guided error, as other sources already give; a batched trial get names the reason for each missing section; a failed reply always names its cause in problems; an empty CPIC recommendation list for a provisional pair says the pair is provisional.

## Why

From the bench 0039 persona review (2026-11-12, runs on BioMCP 58d83b6da), `personas/<slug>/runs/2026-11-12/review.md` and `log.md`: life-sciences-ci log.md:40-43 (boxed warning empty, no reason); rare-disease log.md:87 vs 89 (silent 0 rows vs guided error); trial-ops log.md:46 (4 of 20 sections null); msl log.md:112 and pt-committee log.md:82 (`failed` with the only source `ok`, no problems); tumor-board log.md:265-270 (provisional CPIC pair, zero rows).

## Proof

- Proof: one routine case per behaviour above, each failing on the base commit.

## Starts from

- Starts from: the reply builders for drug label sections, disease sections, trial batched gets, article pdf and CPIC recommendations.

## Keeps

- Keeps: status rules; native totals; existing error schema unless a new code is needed (then the schema rule applies).

## Defers

- Defers: new sources; the fixes for the underlying data gaps.

## Changes

- Changes: `crates/biomcp-query/src/`, `tests/cases/`
