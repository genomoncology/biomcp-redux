# 0895 — vaers-symptoms-ranked-by-count

Status: COMPLETE.
Landed: 9e9ea5102967bdef10245b7714e2fc84120a6288

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

## Progress

- Measured live on 2026-11-13: CDC WONDER's VAERS request form (D8) has no sort or rank field, and its API and VAERS help pages document none. WONDER returns the whole grouped table in one reply, symptoms in alphabetical order (recorded MMR and COVID-19 replies). So BioMCP sorts the full table it already receives.
- Changed: `search adverse-event --source vaers` takes `--sort reports` (VAERS only; other sources refuse it as a usage error). It orders the whole retained table by WONDER's Events Reported count, largest first, before paging; ties and rows without a count keep WONDER's order. Counts, percents, footer and report total stay WONDER's, and a source notice states the order. The default stays WONDER's order. When a page doesn't hold the whole table, the reply offers a `sort` next step that reuses the held snapshot with no new WONDER request. Schemas only gain an optional `sort` field and description text. `docs/cautions.md` has a VAERS symptom order section.
- Proof: `tests/cases/adverse-event/vaers-symptoms-by-reports.yaml`, replaying the COVID-19 symptoms reply recorded in job j39525 (12,971 rows, 995,917 events, vintage 10/30/2026). Sorted, the top five are HEADACHE 151,715 (15.23%), PYREXIA 131,336, FATIGUE 130,958, PAIN 108,525 and CHILLS 106,760; WONDER's own measure metadata gives 151,715 as the largest row.
- Size limits: `help search adverse-event` measures 8,961 bytes after merging main at b6bc70a21 (job j39719); its cap in `tests/cases/adverse-event/help.yaml` rises from 8,761 to 10,306 (measured plus 15 percent, 2026-10-26 caps decision). `help get adverse-event` is unchanged at 3,517 bytes and keeps its cap. The search_adverse_event typed tool fits the 2,400 budget that 0905 set. Earlier wording cuts made only to fit the old 2,000 budget were restored.
- Open: the COVID-19 symptoms request ran past BioMCP's deadline in three of four live attempts on 2026-11-13 (jobs j39507, j39525; WONDER answered the fourth in 18 seconds). That is a separate reliability problem from ordering.
