# 0873 — remaining-pages-report-totals-honestly

Status: OPEN.

Milestone: 7

## Outcome

The pages 0859's survey left behind report a total honestly: the source's own count, a stated "the source gives no total", or a stated filtered count. None sums two sources' totals. A cut-short article search says in plain words that rows past those read can't be paged.

## Why

0859's survey (its Progress section) and its review on 2026-11-12 found these gaps, all instances of 0859's Outcome that it left for a follow-up:

- Pathway search across Reactome and WikiPathways adds their totals (`crates/biomcp-query/src/pathway/union.rs:260`), which 0859's Keeps forbid.
- GWAS search with a p-value cut reports the count before BioMCP's own filter.
- Article recommendations and author papers: the source gives no total and the reply doesn't say so.
- Cell-line search reports no total; whether Cellosaurus offers one is unchecked.
- Article search with one source listed twice loses that source's count (`crates/biomcp-query/src/article/sources.rs:209-210`).
- A cut-short Europe PMC search says so only through the `truncated` flag; past the 100 rows read there is no step and no sentence saying why.
- Drug approvals showed 11 of 25 records with no next step in a persona run (unchecked; the drug files were busy with the label tickets).

## Proof

- Proof: first check each item live and record which still holds; then a routine case for each item that holds shows the honest total or the "no total" statement.

## Starts from

- Starts from: 0859's survey table and its changes to `crates/biomcp-types/src/response/page.rs` and `crates/biomcp-query/src/article/page.rs`.

## Keeps

- Keeps: never invent or sum totals; paging contracts; 0859's behaviour.

## Defers

- Defers: paging Europe PMC beyond BioMCP's read pool.

## Changes

- Changes: `crates/biomcp-query/src/`, `crates/biomcp-adapters/src/`, `tests/cases/`
