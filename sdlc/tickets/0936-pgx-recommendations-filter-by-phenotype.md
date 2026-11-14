# 0936 — pgx-recommendations-filter-by-phenotype

Status: OPEN.

Milestone: 7

## Outcome

CPIC recommendations for one drug can be filtered by phenotype (e.g. Poor Metabolizer), and an unfiltered reply's first page says how many rows and which phenotypes exist.

## Why

The persona reported CPIC's clopidogrel advice from the first page only (Ultrarapid and Rapid Metabolizer, "standard dose"); CPIC holds 24 rows and the Poor Metabolizer rows say "Avoid clopidogrel if possible" (log.md:254-256). A PGx consult without that line is wrong. Evidence: bench `personas/dx-lab-director/runs/2026-11-14/` (j40302, BioMCP f4f726674) and its code-mode pair `runs/2026-11-14-raw/` (j40303); the analyst's source checks are listed at the end of `analysis.md`.

## Proof

- Proof: a case over a recorded CPIC reply for clopidogrel with a Poor Metabolizer filter returns "Avoid clopidogrel if possible"; the unfiltered first page lists the phenotypes present.

## Starts from

- Starts from: `crates/biomcp-adapters/src/cpic/`; the CPIC API's recommendation rows for RxNorm 32968.

## Keeps

- Keeps: the 1.0.0 schema rules (an added filter or optional field); existing pgx cases.

## Defers

- Defers: filtering other pgx sources.

## Changes

- Changes: the CPIC adapter, pgx get options, help, cases.
