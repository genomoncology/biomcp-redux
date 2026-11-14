# 0935 — gtr-condition-match-ignores-punctuation

Status: OPEN.

Milestone: 7

## Outcome

GTR condition and name matching ignores case, hyphens and spaces, so "non-small cell" also finds "Nonsmall cell" and "NonSmall Cell". The reply says matching is normalized.

## Why

The persona's NSCLC slice of EGFR tests came back 0 (log.md:34-41); in NCBI's `test_version.gz`, 4 current EGFR tests hold the phrase in another spelling and none of the EGFR tests uses the literal hyphenated phrase. Evidence: bench `personas/dx-lab-director/runs/2026-11-14/` (j40302, BioMCP f4f726674) and its code-mode pair `runs/2026-11-14-raw/` (j40303); the analyst's source checks are listed at the end of `analysis.md`. Idea 0819 (literal name search over the pinned catalog) is related; keep its rules if this touches name search.

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

## Progress

- Measured on NCBI's `test_version.gz` (Last-Modified 2026-11-05; the analyst's copy and live job j40568 agree). Among current tests, `lab_test_name` holds "non-small cell" literally once (GTR000571186.1, no EGFR) and in another spelling six more times. Four of those carry EGFR: GTR000578049.1 and GTR000578438.1 ("Nonsmall cell lung cancer, ..."), GTR000595767.1 ("NonSmall Cell Lung Cancer Genetic Panel") and GTR000604346.1 ("EGFR Gene Nonsmall cell lung cancer, ..."). GTR000569877.2 writes its first dash as mis-encoded bytes ("Nonâˆ’Small-cell") and stays unmatched under the narrow rule. 56 current tests (19 with EGFR) hold "Non-small cell" among their condition names; BioMCP's condition filter takes a MedGen ID, not text, so this ticket changes name matching only.
- Changed: GTR `--name` matching ignores letter case, hyphens (hyphen-minus, U+2010, U+2011, soft hyphen) and white space, nothing else; en dashes and other punctuation stay literal, and text of only hyphens and spaces matches nothing. Whole-word ranking still checks the characters around the match in the original name. A GTR search with `--name` carries a request-guidance notice that says so. Help and schema descriptions say the same (description-only schema change).
- Case `tests/cases/diagnostic/gtr-name-spelling.yaml` replays the live files cut to EGFR's 185 relation rows and their 185 version rows (`tests/replies/diagnostic/gtr-egfr/`): "non-small cell" with EGFR finds the four tests above, whole-word, and `lung` with EGFR still finds the persona's 19.
