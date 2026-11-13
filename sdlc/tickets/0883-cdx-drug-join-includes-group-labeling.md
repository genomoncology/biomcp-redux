# 0883 — cdx-drug-join-includes-group-labeling

Status: OPEN.

Milestone: 7

## Outcome

A companion-diagnostic search by drug application returns every row of FDA's list that names that application, including the group-labeling table, or says which rows it could not join.

## Why

From the bench 0041/0042 reg-affairs runs (2026-11-13): for osimertinib (TAGRISSO, NDA 208065) BioMCP's `search diagnostic --source fda-cdx --drug-application NDA208065` returns 10 rows, all `individual` (checked on build b87cb3b3e). The raw-tools agent parsed FDA's own list page and found 15 rows naming TAGRISSO, including ONCO/Reveal P200011, MI Cancer Seek P240010 and cobas S031 group labeling (`personas/reg-affairs/runs/2026-11-13-raw/log.md:36`). If the extra rows are group-labeling rows that BioMCP holds but does not join to the application, the reply is silently incomplete for exactly the join reg-affairs called its best demo.

## Proof

- Proof: first compare, live, FDA's list rows naming NDA 208065 and BLA 125514 with BioMCP's rows, and record which are missing and why; then a routine case from a recorded page showing the group rows joined, or a stated problem for any row whose application cannot be read.

## Starts from

- Starts from: `crates/biomcp-adapters/src/fda_cdx/` (`html.rs` group table, `search.rs` `matches`).

## Keeps

- Keeps: exact matching on application number; snapshot identity; existing individual rows.

## Defers

- Defers: biomarker matching for group rows (already reported as a problem).

## Changes

- Changes: `crates/biomcp-adapters/src/fda_cdx/`, `tests/cases/diagnostic/`

## Progress

- Measured live on 2026-11-13 (job j39031, main 40b3504b5, fresh HOME after `biomcp sync fda-cdx`): FDA's page has 244 rows. 15 name NDA 208065: 10 individual rows (BioMCP returned all 10) and 5 group-labeling rows (cobas EGFR v2 P120019/S031, FoundationOne CDx P170019/S033, FoundationOne Liquid CDx P190032/S008, ONCO/Reveal P200011, MI Cancer Seek P240010), which BioMCP missed. 16 name BLA 125514, all individual, and BioMCP returned all 16; the raw agent's count was right and there is no pembrolizumab group row.
- Cause: the group table lists each drug in its own `<li>`, and the parser joined list items with no separator, so "NDA 021743" and the next trade name read as one word ("021743Tagrisso"). No application in the cell could be matched to its link, the cell was marked a drug conflict, and the row joined nothing. The same gap ("BLA 125084VECTIBIX" across a `<p>`) silently dropped individual row 8 (cetuximab), and "NDA 211651/S010" (rows 14 and 15, talazoparib) was not read as NDA 211651.
- Changed: paragraphs and list items now end a line in cell text; an application written with a supplement suffix reads as its application; a drug-application search now reports an unreadable row (drug or device link) as an `identity.conflict` problem when that row's drug cell text or a link address contains the asked application's digits, so a row it cannot join is named, not dropped, and unrelated searches stay clean. On the current page that is one row: individual row 89 links NDA 213246 but its text reads NDA214246 (an FDA typo); an NDA213246 search names it and an NDA208065 search reports no problems.
- Case: `tests/cases/diagnostic/cdx-group-join.yaml` replays the 2026-11-13 page tables.
