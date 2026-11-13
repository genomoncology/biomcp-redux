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
