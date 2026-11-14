# 0934 — pma-summary-find-text

Status: OPEN.

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
