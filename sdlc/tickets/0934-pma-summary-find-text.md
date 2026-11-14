# 0934 — pma-summary-find-text

Status: COMPLETE.
Landed: 268e2dfc40b18d7e08593980842020fb54411ece

Milestone: 7

## Outcome

`get diagnostic <PMA> summary --find TEXT` returns the passages of FDA's safety and effectiveness summary that match, with their fragment numbers, as article text already allows.

## Why

The persona read cobas EGFR v2's summary (P150047) in 22 fragment calls and still missed Table 15's limits of detection (75 / 100 / 100 copies/mL), quoting a footnote from another table instead (log.md:235-239). Evidence: bench `personas/dx-lab-director/runs/2026-11-14/` (j40302, BioMCP f4f726674) and its code-mode pair `runs/2026-11-14-raw/` (j40303); the analyst's source checks are listed at the end of `analysis.md`.

## Proof

- Proof: a case over the recorded P150047 summary asks `--find "Limit of Detection"` and gets Table 15's passage in one reply.

## Starts from

- Starts from: `crates/biomcp-adapters/src/fda_pma/map.rs:42` (splits at 6,144 bytes); article text's `--find` handling, to reuse rather than copy.

## Keeps

- Keeps: the 1.0.0 schema rules; fragment paging without `--find`; size caps.

## Defers

- Defers: other long device documents.

## Changes

- Changes: the PMA summary section, the diagnostic get options, help, cases.

## Progress

- 2026-11-14, measured: BioMCP's lopdf text of P150047B.pdf (53 pages, 22 fragments) puts the "Limit of Detection (LoD)" heading in fragment 8 and Table 15 in fragment 9; Table 15's last row ends 1,741 escaped bytes after the heading. Extracted lines run up to 745 bytes, and table cells come one per line. "Limit of Detection" occurs three times (fragment 4's "limit of detection ( LoD )", the heading, Table 16's title); "LoD" occurs 65 times.
- Changed: `get diagnostic PMA summary` takes `--find TEXT` (optional request field `find`, summary only). The reply keeps `document` without `text` and adds `hits`, each a verbatim passage and the zero-based fragment it starts in; `--section-offset` then counts passages, and the next step pages with the same find. A passage starts up to 512 bytes before a match, at a line break or space, and runs at most 3,072 escaped bytes, ending at a line break after the match; matches inside an earlier passage add none; a phrase wrapped across two extracted lines is found. A find page holds up to 25 passages within 12,000 bytes, article text's find page size.
- Reuse: article text's find paging (offset, 25 hits, byte budget, page info, following offset) and its case-insensitive literal match moved to `biomcp-query/src/text_find.rs`; article text, article supplements and the PMA summary all use it. `pdf::read` now reads the whole text through `pdf::whole`; fragment reading without `--find` is unchanged.
- Schema: one optional request field, one optional reply field and type, one description.
- Proof: `tests/cases/diagnostic/pma-summary-find.yaml`: `--find "Limit of Detection"` returns two passages in one 6,670-byte reply; the second starts in fragment 8 and holds all of Table 15. FDA refused live downloads of the PDF from the M5 (j40564, j40603, j40615: HTTP 406) and the Beelink (302) all afternoon, so the committed reply is the analyst's same-day FDA download, byte for byte; `tests/replies/diagnostic/pma-summary-find/README.md` records its SHA-256 and that gap.
