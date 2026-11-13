# 0895 — vaers-symptoms-ranked-by-count

Status: OPEN.

Milestone: 7

## Outcome

VAERS symptom rows can be returned largest first, so "which symptoms dominate COVID-19 vaccine reports" is one call.

## Why

In the pv-scientist persona run of 2026-11-13 (biomed-agent-bench `personas/pv-scientist/runs/2026-11-13/log.md` and `report.md`, analysis beside them; code-mode run in `runs/2026-11-13-raw/`), CDC WONDER VAERS returned 12,971 symptom rows in alphabetical order and the top symptoms would take about 260 pages (log.md:83). The code-mode agent answered the question from a 2021 data mirror because WONDER refused it.

## Proof

- Proof: a recorded case where the COVID-19 symptoms section sorted by count returns the largest rows first with counts and shares taken from the recorded WONDER reply.

## Starts from

- Starts from: `crates/biomcp-adapters/src/vaers/`; check live whether WONDER can sort, else sort the recorded full reply BioMCP already receives.

## Keeps

- Keeps: the source's own order as the default unless measured otherwise.

## Defers

- Defers: brand-level vaccine names.

## Changes

- Changes: `crates/biomcp-adapters/src/vaers/`, `crates/biomcp-types/`, `tests/cases/adverse_event/`, `docs/`
