# 1013 — ddinter-replies-count-by-level

Status: OPEN.

Milestone: 7

## Outcome

A DDInter interactions reply gives counts by level (major, moderate, minor) over the whole set, and its `next` offers the label's `drug_interactions` section.

## Why

pt-committee reported 323 rows but not 117/200/6 (bench `personas/pt-committee/runs/2026-11-16/analysis.md` 1.3, 4.4).

## Proof

- Proof: a replayed case for apixaban asserting the level counts; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

## Progress

- 2026-11-16: DDInter interactions replies carry a new optional `levels` field: pairs per DDInter level across every listed pair (all pages, after any `--partner` filter), Major, Moderate, Minor first, then other levels by name, then pairs with no level. Every interactions reply's `next` now offers each named drug's openFDA label `drug_interactions` section after the page step (the drug, plus the partner when one is named); before, only an unlisted named pair did. Help for the `interactions` section and the release notes say so. Schemas changed only by the added optional field and descriptions.
- Proof: `tests/cases/drug/ddinter-level-counts.yaml` replays the synthetic DDInter partitions (DDInter is noncommercial, so no apixaban rows are committed; 0695) for "Synthetic Anchor One" at one row per page and asserts Major 2, Moderate 1, Minor 1 over four pairs and the label step. It fails on the base commit f34d6b2bd (job j42004). `ddinter-pair` step 7 grew to 3,179 bytes with the added field and step; its cap is now 3,656 (measured plus 15%).
- Live check (job j42004, fresh `sync ddinter` on the M5): `get drug apixaban interactions --source ddinter --section-limit 50` gave total 323 and `levels` Major 117, Moderate 200, Minor 6, matching fact 1.3, and `next` offered `get drug Apixaban label --label-section drug_interactions --source openfda-label`.
- Jobs: j41990 regenerated schemas and docs, passed `make check-cheap`, `mcp-parity` and `help/` cases; its two drug case failures (an unknown `length` operator in the new case, `ddinter-pair` step 7 size) are fixed. j42004 passed fmt check and all four ddinter cases.
- Gate `make --keep-going landing-gate docs-check schema-release-check` passed in pm job j42010 at 408c1d7aa (branch merged with origin/main cf7fb8816).
