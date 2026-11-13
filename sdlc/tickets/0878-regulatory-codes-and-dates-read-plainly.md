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

## Progress

- Measured live 2026-11-13: openFDA's 510(k) data holds 492 DENG records, all with `decision_description` "Unknown"; SESK, SESU, SEKD, SESD and SESP are also "Unknown" there, but this ticket labels only DENG, whose meaning the DEN number states. WHO PQ's export has 657 rows; dolutegravir has 52, 20 of the first 50 with a blank Date of Prequalification (US FDA alternative listings), not 8 as the run log said.
- 510(k) records now carry `decision_reading: "de-novo-granted"` when `decision_code` is DENG, beside openFDA's own `decision_description`; other codes get no reading.
- WHO PQ rows now carry `prequalified_on` (YYYY-MM-DD, parsed from WHO's text, which stays in `date_of_prequalification`) and, for a blank cell, `prequalified_on_status: "not-published"`. A non-blank cell that is not a whole date gets neither.
- Proof: `tests/cases/diagnostic/510k-de-novo.yaml` (existing openFDA recording) and `tests/cases/drug/who-pq-dates-read.yaml` (WHO PQ export recorded live in a job).
