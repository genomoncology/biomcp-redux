# 0933 — fda-cdx-dated-rows-and-one-row-shape

Status: OPEN.

Milestone: 7

## Outcome

FDA companion-diagnostic rows carry a parsed approval date (or say none could be parsed), the table can be filtered by a date window and read newest first over the whole snapshot, and individual and group-labeling rows share one set of field names while keeping FDA's original cells.

## Why

"Newest companion diagnostics this cycle" took all 244 rows and a local parse, with 2 rows unparseable (log.md:100-102, 133-135), and three of the agent's scripts broke on the two row shapes (calls 36, 38, 45; "Indication - Sample Type" against "Indication(s) - Sample Type"). Evidence: bench `personas/dx-lab-director/runs/2026-11-14/` (j40302, BioMCP f4f726674) and its code-mode pair `runs/2026-11-14-raw/` (j40303); the analyst's source checks are listed at the end of `analysis.md`. This promotes idea 0820, whose rules this ticket keeps: an honest row-date rule, whole-snapshot sort, unparseable rows disclosed, never sorting one page.

## Proof

- Proof: a case over the recorded FDA CDx page asserts the newest rows by date (DEN260027, 10/20/2026, first), an unparseable-date row disclosed, and one field name for the indication cell across both tables.

## Starts from

- Starts from: idea `sdlc/ideas/0820-define-fda-cdx-row-date-semantics-before-adding-.md`; the FDA CDx adapter (`crates/biomcp-adapters/src/fda_cdx/`).

## Keeps

- Keeps: the 1.0.0 schema rules (optional fields, added filters); FDA's original cells; physical order when no sort is asked.

## Defers

- Defers: nothing.

## Changes

- Changes: the FDA CDx adapter, diagnostic search filters, help, cases.
