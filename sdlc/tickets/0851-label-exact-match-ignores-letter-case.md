# 0851 — label-exact-match-ignores-letter-case

Status: OPEN.

Milestone: 7

## Outcome

Label search's exact-name step matches regardless of letter case, so "apixaban" finds the same label sets as "APIXABAN", including all eight ELIQUIS sets, with no name lookup.

## Why

The pharmacy and therapeutics committee persona found "apixaban" returns 1 label set (a stale 2021 generic) while "ELIQUIS" returns 8 including the current label. 0830's design review (2026-11-11) found the cause is letter case, not a missing synonym: the exact step matches case as typed, "APIXABAN" exact finds 10 sets including all 8 ELIQUIS, and the looser word search also finds 10.

## Proof

- Proof: a routine case with recorded openFDA label replies shows "apixaban" and "APIXABAN" returning the same sets; it fails on the base commit.

## Starts from

- Starts from: the label search steps in the openFDA label adapter and query (exact name step before word search), and the review's job logs (j38107, j38118).

## Keeps

- Keeps: step order (exact before word search); everything else in the reply.

## Defers

- Defers: widening names (0830, 0850).

## Changes

- Changes: the openFDA label adapter and query, `tests/cases/drug/`

## Amendment, 2026-11-12

PM ruling: the name step now pages one list, exact-name hits first and then word-search hits that are not exact, with no threshold. The exact selector asks for the name upper case, title case, sentence case, lower case and as typed, so every casing of a name sends the same request. A page takes two requests: the exact hits, then either the word-only rows that fill it or a one-row probe for the word-only total. A page that starts past the exact hits also probes the exact total, because openFDA answers a skip past the end with "not found", and a page past the end of everything probes both totals, so four requests at most. The page total is the word-search total, so it changes from the exact-only count: apixaban goes from 1 to 10 sets, imatinib stays at 24 and dabigatran at 23 (live comparison j38225). A brand stored in mixed case, such as one with a capital in mid-word, is not an exact hit for other casings and still appears among the word hits. Proof: `tests/cases/drug/label-letter-case.yaml` and the re-recorded `tests/cases/drug/openfda-label-page.yaml`.
