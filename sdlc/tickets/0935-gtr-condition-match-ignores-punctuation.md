# 0935 — gtr-condition-match-ignores-punctuation

Status: OPEN.

Milestone: 7

## Outcome

GTR condition and name matching ignores case, hyphens and spaces, so "non-small cell" also finds "Nonsmall cell" and "NonSmall Cell". The reply says matching is normalized.

## Why

The persona's NSCLC slice of EGFR tests came back 0 (log.md:34-41); in NCBI's `test_version.gz`, 5 current EGFR tests hold the phrase in another spelling and none hold the literal hyphenated phrase. Evidence: bench `personas/dx-lab-director/runs/2026-11-14/` (j40302, BioMCP f4f726674) and its code-mode pair `runs/2026-11-14-raw/` (j40303); the analyst's source checks are listed at the end of `analysis.md`. Idea 0819 (literal name search over the pinned catalog) is related; keep its rules if this touches name search.

## Proof

- Proof: a case over the recorded GTR catalog slice: "non-small cell" with EGFR finds the tests spelled "Nonsmall cell" and "NonSmall Cell".

## Starts from

- Starts from: `crates/biomcp-adapters/src/gtr/`; idea `sdlc/ideas/0819-search-gtr-tests-by-literal-name-across-the-pinn.md`.

## Keeps

- Keeps: the 1.0.0 schema rules; counts and paging over the whole catalog.

## Defers

- Defers: synonym expansion (NSCLC to non-small cell lung cancer).

## Changes

- Changes: the GTR matcher, help, cases.
