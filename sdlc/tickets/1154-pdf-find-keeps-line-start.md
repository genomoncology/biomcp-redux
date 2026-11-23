# 1154 — pdf-find-keeps-line-start

Status: OPEN.

Milestone: 7

## Outcome

A PDF phrase search returns each passage from the start of its line when it fits the passage budget, so a hit never drops leading words such as "No".

## Why

Found 2026-11-23 in the BioMCP 1.0 adversarial review (Astra, snapshot cf9a822889ce), reproduced by the reviewer. With page 36 of the 335-page trial fixture changed to `No events were observed.`, the part reader returned the sentence intact but `trial document ... --find events` returned `events were observed.` with status `complete`. At the start of a page the passage builder skips past the first space even when the whole prefix fits (`crates/biomcp-query/src/pdf_find.rs:71`). The checked-in case already expects the loss: `tests/cases/trial/large-posted-sap.yaml:46` expects `race was tabulated.` from `Baseline race was tabulated.`

## Proof

- A case where a hit's line starts with `No` returns the whole line, and the part read and the search hit agree on the text.
- The existing expectation in `large-posted-sap.yaml` changes to the whole line; say so in the report.

## Changes

- `crates/biomcp-query/src/pdf_find.rs` and its callers (the diagnostic PDF reader shares the helper); the large-posted-sap case.
