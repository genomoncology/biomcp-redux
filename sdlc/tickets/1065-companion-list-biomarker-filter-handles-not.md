# 1065 — companion-list-biomarker-filter-handles-not

Status: COMPLETE.
Landed: 0620c3894765097b8ffc7a72937ddb5b13841cf3

Milestone: 7

## Outcome

`biomcp search diagnostic --source fda-cdx --biomarker MSI-High` does not return, unmarked, a row whose biomarker cell says the opposite, such as "Not MSI-High". Help says what the filter does with such cells.

## Why

Ticket 1056 made the filter match a whole biomarker name inside a cell. Its reviewer found that FDA's list has a cell reading "Not MSI-High", which the filter now returns for `MSI-High`. The row shows its own text, but a caller who counts rows or skims a list of "MSI-High tests" could pick a test meant for patients without MSI-High.

## Proof

- Proof: a case on the saved FDA page where `--biomarker MSI-High` either leaves out the "Not MSI-High" rows or marks each one, as the PM decision below says; it fails on the base commit.

## Starts from

- Starts from: `crates/biomcp-adapters/src/fda_cdx/biomarker.rs` from ticket 1056, and the saved FDA page's cells "Not MSI-High" and "Microsatellite stable/MSS (Not MSI-High)".
- PM decision, 2026-11-19: a cell where the name is directly preceded by "Not" is still returned, because leaving a row out silently is the worse error, and the row carries a field saying the cell negates the name. The developer may propose leaving such rows out instead, with the count of cells affected on the saved page.

## Keeps

- Keeps: rows keep FDA's order and cells keep FDA's text. Ticket 1056's seven PIK3CA rows.

## Defers

- Defers: other ways a cell can qualify a name, such as "wild-type"; the developer lists any found on the saved page.

## Changes

- Changes: the fda-cdx biomarker match, its help text and cases.
