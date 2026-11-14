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

## Progress

- Measured: the persona ran `get pgx recommendations --drug-id RxNorm:32968 --gene CYP2C19 --source cpic` and piped it through `head -c 1800` (call record 90 of the bio run). That cut kept only the first two rows (Ultrarapid and Rapid Metabolizer). On f4f726674 the default page holds 5 of 24 rows, and `page.total` sits after the items, at byte 5,456 of 5,815, so it was never seen. No reply named the other phenotypes.
- CPIC live, 2026-11-14: RxNorm:32968 has 24 CYP2C19 rows, three for each of eight phenotypes. The Poor Metabolizer rows (ids 8609171, 8609179, 8609187) say "Avoid clopidogrel if possible", and so do the Likely Poor Metabolizer rows. PostgREST's `phenotypes->>CYP2C19=eq.Poor Metabolizer` matches exactly and is case-sensitive, so "poor metabolizer" matches nothing. Across all of CPIC, one gene and drug has at most 206 rows. HLA-A and HLA-B rows carry no phenotype.
- Changed: `get pgx recommendations` takes `--phenotype TEXT` (optional `phenotype` query field, CPIC only, recommendations only). It filters both the view and the drug table on the exact CPIC phenotype for the gene. The drug table's first page carries `result.phenotypes`, which lists each phenotype CPIC has for that gene and drug in row order with its row count. It sits right after the header, at byte 273 of the persona's call, with Poor Metabolizer at byte 550. When the page already holds every row of an unfiltered read, the list comes from those rows. Otherwise one extra read fetches it: `select=phenotype:phenotypes->>GENE`, limit 1,000, and it must be complete. A failed list read leaves the rows and adds a notice. A filter that matches nothing adds a notice that points at `result.phenotypes`. With no phenotypes at all, the existing pair notice applies.
- Schemas: one optional query field and one optional reply field were added, and `make schema-generate` passed its compatibility check (j40460). Docs were regenerated in the same job. Cases: `tests/cases/pgx/recommendations-filter-by-phenotype.yaml` runs over replies recorded live in j40460 and j40474. `empty-pair-says-why` gained one recorded reply for its new phenotype-list read. The synthetic CPIC table runner now serves the phenotype read.
- Overlap with 0937 (ticket/0937-skill-totals), which moves `page` before `items`: both branches edit the `PgxRecommendations::Table` variant in `crates/biomcp-types/src/pgx/response.rs`, and the generated `crates/biomcp-types/schemas/pgx-recommendations.json` may conflict too. Merge the variant as header, page, phenotypes, items, then regenerate the schemas.
