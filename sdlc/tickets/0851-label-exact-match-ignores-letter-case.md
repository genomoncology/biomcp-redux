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
