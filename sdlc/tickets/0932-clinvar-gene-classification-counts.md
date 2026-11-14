# 0932 — clinvar-gene-classification-counts

Status: OPEN.

Milestone: 7

## Outcome

One call gives a gene's ClinVar variant counts by germline classification and by review status, each variant counted once, with the ClinVar release it came from.

## Why

A truth-set table for a hereditary-panel validation needs "BRCA1: N pathogenic or likely pathogenic, M at expert panel". The BioMCP agent fell back to overlapping E-utilities word counts (log.md:205-212, 223-230); code mode parsed ClinVar's 458 MB file and double-counted every variant across genome builds (31,753 records, 8,403 P/LP; raw log.md:237-244). ClinVar's `variant_summary.txt.gz` gives 4,327 BRCA1 P/LP on GRCh38, 2,286 at expert panel. Evidence: bench `personas/dx-lab-director/runs/2026-11-14/` (j40302, BioMCP f4f726674) and its code-mode pair `runs/2026-11-14-raw/` (j40303); the analyst's source checks are listed at the end of `analysis.md`.

## Proof

- Proof: a case over a recorded ClinVar reply (or a small slice of the summary file) for one gene asserts counts by classification and review status, each variant once, with the release date.

## Starts from

- Starts from: `crates/biomcp-adapters/src/clinvar/`; ClinVar's E-utilities property filters (`"clinsig pathogenic"[prop]` gave 4,752 for BRCA1, including multi-gene records) and `variant_summary.txt.gz`. Decide live which is honest and cheap; say in the reply what is counted (single-gene records or all).

## Keeps

- Keeps: the 1.0.0 schema rules (an added section or optional fields); existing variant cards.

## Defers

- Defers: somatic classifications and per-condition counts.

## Changes

- Changes: the ClinVar adapter, a gene section or command, help, cases.
