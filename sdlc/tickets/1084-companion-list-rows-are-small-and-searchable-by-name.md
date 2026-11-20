# 1084 — companion-list-rows-are-small-and-searchable-by-name

Status: COMPLETE.
Landed: ddf2bad3f278df36f22d507efe66ee3cd02d054d

Milestone: 7

## Outcome

A row of FDA's companion diagnostic list states each value once, `--fields` narrows a row to the columns asked for, and the list can be searched by a drug name or a test name.

## Why

Measured 2026-11-20 on a Linux build of main 5db190f8a. A page of 50 rows from `biomcp search diagnostic --source fda-cdx --limit 50` is 76,685 bytes, about 1,500 a row, and the whole list is 244 rows. Each row gives every value twice, once as a flat field (`diagnostic`, `drug`, `indication`, `device`) and again under `cells`, and repeats the same `snapshot` hash on every row. `--fields=diagnostic,drug` still returns 58,507 bytes, because `cells` (745 bytes) and `snapshot` are kept on every row.

In twelve traced agent runs the same day, agents that wanted the rows for a drug or a test tried `search diagnostic fulvestrant --source fda-cdx` and `--name NAME`; both are refused, so two runs paged the whole list instead.

## Proof

- Proof: a case on the saved FDA page where `--fields=diagnostic,drug` returns rows holding those two columns and nothing else that was not asked for, and the page is under a quarter of its size today; it fails on the base commit.
- Proof: a case where a search by a drug name and one by a test name each return the rows whose drug or diagnostic cell holds that name, and say which column matched; it fails on the base commit, where both are refused.
- Proof: the unnarrowed row states each cell's text once. The ticket reports bytes per row before and after.

## Starts from

- Starts from: `crates/biomcp-adapters/src/fda_cdx/`, the reply schema `biomcp.diagnostic-companion`, and tickets 1071 (choosing columns) and 1076 (values stated once).
- PM decision, 2026-11-20: the snapshot hash is stated once for the page. Links inside a cell stay available; the developer proposes where they sit so the text is not repeated. This changes the reply's schema; follow the repo's schema rules.

## Keeps

- Keeps: no row or cell text is dropped from an unnarrowed reply. The `--biomarker` and `--drug-application` filters behave as now.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-adapters/src/fda_cdx/`, the diagnostic search command, its schema and cases.
