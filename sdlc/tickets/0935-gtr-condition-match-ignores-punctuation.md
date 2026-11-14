# 0935 — gtr-condition-match-ignores-punctuation

Status: OPEN.

Milestone: 7

## Outcome

GTR test-name matching ignores letter case and hyphens in the name, and treats spaces and hyphens in the query as optional, so "non-small cell" also finds "Nonsmall cell" and "NonSmall Cell"; a name space is skipped only where the query had a space or hyphen, so "CF" does not match "Amniotic Fluid". The reply says how names were matched.

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
- Idea, not built (PM, 2026-11-14): search GTR tests by condition name text with the same case, hyphen and space folding. In NCBI's file, 56 current tests (19 with EGFR) hold "Non-small cell" only in `condition_identifiers`, which `--name` does not read and `--condition-id` (MedGen ID) cannot reach by text.
- Caps measured in job j40659 with the case runner (MCP replies were no larger than CLI): sync 197, "non-small cell" with EGFR 4,192, "lung" with EGFR at limit 50 17,147 bytes; caps set to measured plus 15% (227, 4,821, 19,720). The same job ran `make schema-generate docs-generate`; the tree stayed clean, so the checked-in schemas and `docs/reference/commands.md` match the generated output.
- Review fix (2026-11-14): dropping every space from names let short text match across words ("CF" 551 → 803, matching "Amniotic Fluid" and "Hepatic failure"). The rule now ignores letter case and hyphens in the name, and treats spaces and hyphens in the query as optional; a name space is skipped only where the query had a space or hyphen. The reviewer measured that this keeps "non-small cell" at 7 current tests (the 4 EGFR ones whole-word), "BRCA 1" 64 and "T cell" 57, and returns "CF" to 551 and "ran" to 1,085. Unit tests in `gtr/name_tests.rs` cover the rule, including "CF" against "Amniotic Fluid Culture". The notice, help and schema descriptions say "hyphens and spaces in your text are optional", and the notice is left off when the GTR read failed. The earlier bullets describe the first rule.
