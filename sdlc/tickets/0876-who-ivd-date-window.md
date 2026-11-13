# 0876 — who-ivd-date-window

Status: COMPLETE.
Landed: ce845d4aa633a9116534da08a651189b19ad243d

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

## Progress

- Measured live 2026-11-13: WHO's IVD export has 439 rows, 127 products; every `Year prequalification` cell is a four-digit year, and rows of one product share one year. 62 products carry the HIV marker; 9 of them were prequalified in 2023 or later.
- `search diagnostic --source who-ivd` now takes `--prequalified-from`/`--prequalified-to` as whole years (inclusive), alone or with `--marker`/`--manufacturer`; the page total counts products inside the window. A month or day bound is refused with "WHO IVD publishes only the prequalification year; give --prequalified-from/--prequalified-to as YYYY", other sources refuse the options, and a reversed window is refused.
- Proof: `tests/cases/diagnostic/who-ivd-window.yaml` on the WHO IVD export recorded live in a job: HIV since 2023 returns those 9 products with total 9; 2023 alone totals 4.
