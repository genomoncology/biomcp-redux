# 0862 — civic-join-and-section-limits

Status: OPEN.

Milestone: 7

## Outcome

The CIViC join matches every allele that encodes the protein change and reports a failed join as failed, not empty; the CIViC section honours `--section-limit` and filters by disease and drug.

## Why

From the bench 0039 persona review (2026-11-12, runs on BioMCP 58d83b6da), `personas/<slug>/runs/2026-11-12/review.md` and `log.md`: tumor-board log.md:313 (one DNA spelling of C797S returned empty), log.md:86 and 108 (2-3 rows whatever the limit, no filters); dx-lab-director log.md:62-64.

## Proof

- Proof: routine cases for C797S across its alleles and a CIViC section with limit 10 and a disease filter.

## Starts from

- Starts from: the CIViC adapter and variant join.

## Keeps

- Keeps: CIViC identities; source attribution.

## Defers

- Defers: the composite variant evidence pack (tumor-board log.md:441).

## Changes

- Changes: `crates/biomcp-adapters/src/civic/`, `crates/biomcp-query/src/variant/`, `tests/cases/variant/`
