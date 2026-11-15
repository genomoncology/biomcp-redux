# 0964 — gtr-search-by-condition-name

Status: OPEN.

Milestone: 7

## Outcome

A GTR test search can match a condition by name text, with the same case, hyphen and space folding as `--name`, so "non-small cell lung cancer" finds the tests that name it only among their conditions.

## Why

0935 (PM idea, 2026-11-14): in NCBI's `test_version.gz`, 56 current tests (19 with EGFR) hold "Non-small cell" only in `condition_identifiers`, which `--name` does not read and `--condition-id` (a MedGen ID) cannot reach by text; the dx-lab-director's NSCLC slice of EGFR tests came back 0.

## Proof

- Proof: a case on the synced GTR catalog (recorded real rows) where a condition-name search finds a test whose name lacks the phrase; the persona's NSCLC + EGFR query returns the tests 0935 counted.

## Starts from

- Starts from: 0935's ticket and its folding; the GTR catalog sync and search code (`git log origin/main --oneline -- '*gtr*'`).

## Keeps

- Keeps: `--name` and `--condition-id` behavior; reply size caps.

## Defers

- Defers: nothing.

## Changes

- Changes: GTR search filter, catalog columns if needed, cases, help.
