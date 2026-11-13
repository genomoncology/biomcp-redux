# 0878 — regulatory-codes-and-dates-read-plainly

Status: OPEN.

Milestone: 7

## Outcome

Regulatory codes and dates read plainly: a De Novo record says it is a De Novo grant, and WHO prequalification dates are ISO dates, with a blank stated as not published.

## Why

From bench 0041 reg-affairs smoke run on BioMCP b87cb3b3e (pm job j38923), `personas/reg-affairs/runs/2026-11-13/log.md` in biomed-agent-bench (until filed: `~/workspace/data/biomed-agent-bench/0041-persona-reg-affairs/` on the M5), log.md:109, 118 and 233-236: DEN170058 shows `decision_description: "Unknown"` (openFDA's own value for code DENG, checked 2026-11-13), which a tracker column shows as Unknown; WHO PQ `date_of_prequalification` arrives as "14  Oct,  2014" and is empty on 8 of 52 dolutegravir rows.

## Proof

- Proof: routine cases showing DENG labelled as a De Novo grant with the source's own value kept, and WHO PQ rows with an ISO date and a stated blank.

## Starts from

- Starts from: the 510(k) adapter's decision fields and the who-pq adapter.

## Keeps

- Keeps: the source's original value beside any reading of it; never invent a date.

## Defers

- Defers: a De Novo search source of its own.

## Changes

- Changes: `crates/biomcp-adapters/src/`, `crates/biomcp-types/src/`, `tests/cases/`
