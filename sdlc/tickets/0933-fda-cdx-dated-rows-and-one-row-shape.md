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

## Progress

- 2026-11-14, measured live: FDA's CDx page (fetched today, 180,157 bytes) has 235 individual rows and 9 group rows; its two table substrings are byte-identical to the 2026-11-13 recording in `tests/replies/diagnostic/cdx-group-join/` (projected SHA256 e87b01a8…). Dates appear only in the device cell (individual column 6, group column 3), always inside parentheses, never in another cell. 238 device cells hold one parenthesized date, 5 hold two (individual rows 91-93: `P190032 (08/26/2020)` and `P190032/S008 (12/19/2022)`, the group-labeling supplement; row 141: `P150013 (10/02/2015); updated P150013/S012 (04/16/2019)`; row 161: `P190001 (05/24/2019)` and `P190004 (05/24/2019)`). Forms: `MM/DD/YYYY` everywhere except group row 0, `7/30/2026` (one-digit month). Two cells are malformed: individual row 48 `P120019/S019 (08/22/0218)` (year 218) and row 193 `P190032/S014 (06/28/ 2024)` (a space inside the date). No cell is empty of a date otherwise.
- Row-date rule: a row's `date` is the newest date printed in its own device cell. Each parenthesized group in that cell must be one `M/D/YYYY` date (month and day one or two digits, four-digit year from 1976, when FDA's PMA pathway began, a real calendar day; surrounding spaces ignored). If the cell has no parenthesized group, or any group is not such a date, the row has no `date`, and BioMCP does not repair FDA's text. The date is the newest action FDA lists for the row (approval, clearance or grant, original or supplement), not a claim about which one first approved the indication. On today's page 242 rows are dated and 2 (individual rows 48 and 193) are not.
- Sorting and the date window run over every row of the stored snapshot that matches the other filters, before offset and limit; a single returned page is never sorted. Newest first; ties keep physical order (individual table before group table, then row index). Undated rows sort after all dated rows, in physical order, and a date window leaves them out. When a sort or window is asked, `problems` name every undated matching row in the snapshot (`snapshot:table:index`, field `date`); otherwise they name undated rows on the returned page. With no sort the physical order stays.
