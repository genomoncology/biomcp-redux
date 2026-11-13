# 0887 — gtr-rows-carry-test-type-methods-and-lab-contact

Status: OPEN.

Milestone: 7

## Outcome

A GTR test search answers "what tests exist for this gene and where can the family order one" in as few calls as NCBI itself needs: each row carries the test's gene count, method categories, lab location and order link, and the search can return all of a gene's tests (a few hundred) so the agent counts them itself.

## Why

In the rare-disease persona run of 2026-11-13 (biomed-agent-bench `personas/rare-disease/runs/2026-11-13/log.md`, analysis beside it; code-mode run in `runs/2026-11-13-raw/`), the agent paged 5×50 rows and then fetched 8 tests × 2 cards to sample single-gene vs panel vs exome, and still had no lab address, director or order link (log.md:255 on). The code-mode agent got all 215 KMT2D tests with gene count, methods, the lab's full address, director and order URL from one E-utilities search and three esummary batches (raw log.md:160-168).

## Proof

- Proof: a recorded case for KMT2D where search rows carry gene count, method categories, lab city/state/country and order or test URL, all from the recorded GTR reply; the search returns every test in one call or states the total and its pages; the counts by gene-count bucket agree with the recorded reply.

## Starts from

- Starts from: `crates/biomcp-adapters/src/gtr/`.

## Keeps

- Keeps: the test card and its sections.

## Defers

- Defers: a filter on test type.

## Changes

- Changes: `crates/biomcp-adapters/src/gtr/`, `crates/biomcp-types/`, `tests/cases/diagnostic/`, `docs/`
