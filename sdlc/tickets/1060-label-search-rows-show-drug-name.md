# 1060 — label-search-rows-show-drug-name

Status: COMPLETE.
Landed: 71728276d19a58ec9474039a0541714c83b88474

Milestone: 7

## Outcome

Every row of a label search shows the drug's name. No row shows the label's set ID in the `name` field.

## Why

A caller reading the list cannot tell what two of ten rows are without a second call for each.

## Proof

- Proof: `biomcp search drug --indication PIK3CA --source openfda-label --limit 50` shows a drug name on every row; it fails on the base commit, where two rows have their set ID as the name.

## Starts from

- Starts from: Seen on main be96d8d40 (1.0.0), Linux, 2026-11-19, while agents used the command line build on real tasks. In that search, rows `86718885-da44-4d3d-b3ca-06cbcedcffd8` (version 13, 2026-07-06) and `62adb152-d102-4c33-a06f-58df8ea70e92` (version 5, 2024-10-14) have `name` equal to their ID and no application numbers. An agent had to fetch each to learn they are an Ibrance and an Itovebi label.
- Likely these labels lack openFDA's harmonized `openfda` block. The name is still in the label's own text, such as the title or the first words of the indications section.

## Keeps

- Keeps: a name is taken only from the label's own record, never guessed. A row with no name anywhere says so plainly.

## Defers

- Defers: grouping several label sets of one drug into one row.

## Changes

- Changes: the openFDA label search row, its cases.
