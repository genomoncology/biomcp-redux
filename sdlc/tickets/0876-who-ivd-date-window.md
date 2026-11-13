# 0876 — who-ivd-date-window

Status: OPEN.

Milestone: 7

## Outcome

The WHO IVD list takes a prequalification date window, as the WHO medicines list already does.

## Why

From bench 0041 reg-affairs smoke run on BioMCP b87cb3b3e (pm job j38923), `personas/reg-affairs/runs/2026-11-13/log.md` in biomed-agent-bench (until filed: `~/workspace/data/biomed-agent-bench/0041-persona-reg-affairs/` on the M5), log.md:150-165: `search diagnostic --source who-ivd --marker HIV --prequalified-from 2023` is refused, so finding tests prequalified since 2023 means paging all 62 rows and filtering by hand, while `search drug --source who-pq --prequalified-from 2023` works.

## Proof

- Proof: a routine case for `--prequalified-from`/`--prequalified-to` on who-ivd returning only rows in the window with the source total for the window.

## Starts from

- Starts from: the who-pq date filter and the who-ivd adapter (`prequalification_year`).

## Keeps

- Keeps: who-ivd rows and totals; a window on year-only data is stated as year-granular.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-adapters/src/`, `crates/biomcp-query/src/diagnostic/`, `tests/cases/diagnostic/`
