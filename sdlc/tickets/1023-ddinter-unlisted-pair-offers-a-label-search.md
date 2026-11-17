# 1023 — ddinter-unlisted-pair-offers-a-label-search

Status: OPEN.

Milestone: 7

## Outcome

When a named drug pair has no DDInter row, the reply's next steps offer `search drug NAME --source openfda-label` for each drug, as every other DDInter interactions reply does after 1013, instead of a direct label read that fails when DDInter's name is not an exact openFDA label name.

## Why

1013's review (2026-11-16) found the direct label read fails for DDInter names such as Paracetamol or Salbutamol (`crates/biomcp-query/src/drug/resolve.rs:346`). 1013 changed the other path and kept this one by its Keeps line.

## Proof

- Proof: the `ddinter-pair` case's unlisted-pair step asserts the search steps; it fails on the base commit.

## Starts from

- Starts from: 1013's next-step code in `crates/biomcp-query/src/drug/ddinter.rs`.

## Keeps

- Keeps: the counts, rows and every other next step.

## Defers

- Defers: nothing.

## Changes

- Changes: that next step, its case, help text if it names the step, one release-notes line.

## Progress

- 2026-11-16: A named pair DDInter does not list now gets the same `next` as every other interactions reply: `search drug NAME --source openfda-label` (kind `disambiguate`) for the drug and the partner, in place of the two `get drug NAME label --label-section drug_interactions` reads. The separate unlisted-pair code path in `crates/biomcp-query/src/drug/ddinter.rs` is gone; counts, rows and the other steps are unchanged. Help already said every reply offers the search, so it is unchanged; the release-notes line now says an unlisted pair is included. Schemas and docs regenerate with no diff.
- Proof: `tests/cases/drug/ddinter-pair.yaml` step 4 asserts the two search steps exactly. Job j42118 at 53bca0c4f passed `make check-cheap`, all four `drug/ddinter` cases and `mcp-parity`, and the case fails with the base commit's `ddinter.rs` (35e17aabd): it got the two `section` label reads.
- Gate `make --keep-going landing-gate docs-check schema-release-check` passed in pm job j42124 at 2ebed7efa (branch on origin/main 35e17aabd).
