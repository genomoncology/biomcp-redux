# 1147 — disease-search-keeps-source-rows

Status: OPEN.

Milestone: 7

## Outcome

A disease search returns every matching row its source reports, or pages to them, and never shows a total larger than what it can return.

## Why

Found 2026-11-22 by scouts exploring BioMCP for multi-entity questions (about 760 CLI calls on build b7def4651). `biomcp search disease SLC6A1 --limit 12` showed source total 4, returned 1, and flagged `ols obo_id value.unknown`. Searches for DDX3X, HTRA1, SCN8A and STXBP1 showed the same warning, and STXBP1 and SCN8A returned animal-model terms in a human disease search.

## Changes

- Rows the OLS source returns with an identifier in another field are mapped, not dropped; rows that cannot be identified are counted and reported.
- Human disease search excludes non-human ontology terms unless asked, or labels them.
- `returned` and `total` agree with what the page can deliver.

## Proof

- Replay of the OLS reply for one of these queries: all four rows returned with identifiers; a non-human term is excluded or labelled.
