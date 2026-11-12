# 0854 — gtr-test-name-matches-whole-words

Status: OPEN.

Milestone: 7

## Outcome

`search diagnostic --source gtr --name TEXT` lists tests whose name contains TEXT as whole words first, then tests where TEXT appears only inside a longer word, and the reply says which rows are which.

## Why

Bench 0037's persona rerun (dx-lab-director, 2026-11-12): `--name HAMA` returned Hamartoma tests ahead of the HAMA assay, because 0829 matches any part of a name, as its ticket asked.

## Proof

- Proof: a routine case under `tests/cases/diagnostic/` with a fixture holding a "HAMA" test and a "Hamartoma" test shows the HAMA test first and the Hamartoma test after it, marked as a partial-word match; it fails on the base commit.

## Starts from

- Starts from: 0829's GTR name search and its fixture bundle; bench `personas/dx-lab-director/runs/2026-11-12/`.
- PM decision: 2026-11-12. Rank, don't drop: 0829 promised finding a test by part of its name, so partial-word hits stay, after whole-word hits. Same shape as 0851's exact-then-word rule.

## Keeps

- Keeps: finding a test by part of its name; case-insensitive matching; empty answer when nothing matches.

## Defers

- Defers: fuzzy or misspelled names.

## Changes

- Changes: GTR name search in `crates/biomcp-adapters/src/gtr/` or its query, `tests/cases/diagnostic/`
