# 1056 — companion-list-biomarker-filter-misses-rows

Status: OPEN.

Milestone: 7

## Outcome

`biomcp search diagnostic --source fda-cdx --biomarker PIK3CA` returns every row of FDA's list whose biomarker cell names PIK3CA, including a cell with stray spaces and a cell that lists several biomarkers.

## Why

A caller who trusts the filter gets an incomplete list of tests with `status: complete` semantics for the rows it did return, and nothing says which rows were left out.

## Proof

- Proof: a case on a saved copy of FDA's list where `--biomarker PIK3CA` returns all seven rows below; it fails on the base commit, which returns five.

## Starts from

- Starts from: Seen on main be96d8d40 (1.0.0), Linux, 2026-11-19, while agents used the command line build on real tasks. `--biomarker PIK3CA --limit 50` returned 5 rows: four Piqray rows and Itovebi's FoundationOne Liquid CDx row. `--drug-application NDA219249` and `NDA218197` show two more rows it missed: Itovebi with FoundationOne CDx, whose biomarker cell reads `PIK3CA  ` with trailing spaces, and Truqap with FoundationOne CDx, whose cell reads `PIK3CA, AKT1, and PTEN`.
- The first miss is a plain fault: the cell is not trimmed before the exact match. The second is the documented rule ("Exact table-0 cell") giving a result no caller wants.
- PM decision, 2026-11-19: `--biomarker` matches a whole biomarker name inside the cell, ignoring case and surrounding spaces, so `PIK3CA` matches `PIK3CA, AKT1, and PTEN` and does not match `PIK3CA2`. Help says so.
- The same reply carried `problems: [{"field":"table_1.biomarker","code":"value.unknown"}]`. Say in the reply what that means for the caller: the group table was not searched.

## Keeps

- Keeps: rows keep FDA's order and cells keep FDA's text, spaces included.

## Defers

- Defers: searching the group table by biomarker.

## Changes

- Changes: the fda-cdx filter, its help text and cases.
