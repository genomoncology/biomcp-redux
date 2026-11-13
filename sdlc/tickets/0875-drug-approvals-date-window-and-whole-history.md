# 0875 — drug-approvals-date-window-and-whole-history

Status: OPEN.

Milestone: 7

## Outcome

A drug's approvals section takes a decision-date window, and a whole application history (Keytruda's 128 submissions) fits in one page.

## Why

From bench 0041 reg-affairs smoke run on BioMCP b87cb3b3e (pm job j38923), `personas/reg-affairs/runs/2026-11-13/log.md` in biomed-agent-bench (until filed: `~/workspace/data/biomed-agent-bench/0041-persona-reg-affairs/` on the M5), log.md:40-45 and 212-220: counting Keytruda's supplements per year took three paged calls at the 50-row maximum, and the approvals section has no date window, though `--decision-from/--decision-to` exist for PMA and 510(k).

## Proof

- Proof: routine cases for `get drug BLA125514 approvals --decision-from 2024-01-01` returning only those submissions with the window stated, and a page of up to 200 rows within the reply budget.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/drugsfda/approvals.rs`, the section limit bounds, the PMA/510(k) date-window options.

## Keeps

- Keeps: page totals and next steps; reply size limits measured, not guessed.

## Defers

- Defers: per-year counts (the agent counts rows).

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `crates/biomcp-types/src/`, `tests/cases/drug/`

## Progress

- Stacked on 0874 (`ticket/0874-sponsor-newest`): both change the Drugs@FDA submission order code in `crates/biomcp-adapters/src/drugsfda/map.rs`.
- Measured live on 2026-11-13 (jobs j39033, j39043): BLA125514 holds 128 submissions, 109 of them efficacy supplements, 26 dated on or after 2024-01-01. Each efficacy supplement on a page costs one FDA letter download, and FDA letters cannot be recorded (the record run of a default 10-row windowed page stopped with exit 5 while reading letters; the existing supplement-purpose case uses synthetic letters for the same reason).
- Changed: `get drug <id> approvals --source drugsfda` takes `--decision-from` and `--decision-to` (inclusive, YYYY-MM-DD, Drugs@FDA approvals only, not with `--document-id`). The page total counts only submissions in the window, undated ones are left out, and a notice states the window and "N of M". `--section-limit` goes up to 200 for Drugs@FDA approvals; every other source and section keeps 50. A page asked for with more than 50 rows reads no supplement letters and says so.
- Measured: the whole 128-row history is 90,461 bytes and took 4 s live; the 2024-onward window is 19,979 bytes.
- Proof: `tests/cases/drug/approvals-window.yaml` over one recorded application reply.
- Review follow-ups: the window notice now reads "on or after", "on or before" or "from A to B, inclusive", and the case adds a same-day window (2024-01-12 to 2024-01-12) that returns only submission 147, which proves both ends are inclusive.
