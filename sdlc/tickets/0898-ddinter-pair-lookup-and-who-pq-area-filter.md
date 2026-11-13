# 0898 — ddinter-pair-lookup-and-who-pq-area-filter

Status: OPEN.

Milestone: 7

## Outcome

Two small lookups in one call each: DDInter interactions for a named pair of drugs (with a plain note that DDInter's published file carries severity levels only, no management text), and WHO prequalified medicines filtered by therapeutic area.

## Why

In the pv-scientist persona run of 2026-11-13 (biomed-agent-bench `personas/pv-scientist/runs/2026-11-13/log.md` and `report.md`, analysis beside them; code-mode run in `runs/2026-11-13-raw/`): proving three DDInter pairs absent took paging 12 + 258 + 550 rows, and the agent then curled DDInter to learn the file has no management column (log.md:175 on, report Q5); finding WHO PQ's 18 oncology rows took 12 pages over 566 rows (report Q10).

## Proof

- Proof: recorded cases where a pembrolizumab–lenvatinib pair lookup returns no row with the levels-only note, a known interacting pair returns its level, and WHO PQ filtered to oncology returns the recorded oncology rows.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ddinter/`, `who_pq/`.

## Keeps

- Keeps: per-drug interaction lists.

## Defers

- Defers: scraping DDInter's web checker for management text.

## Changes

- Changes: `crates/biomcp-adapters/src/ddinter/`, `who_pq/`, `crates/biomcp-types/`, `tests/cases/drug/`, `docs/`
