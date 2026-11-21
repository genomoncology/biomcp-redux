# 1095 — search-trials-by-acronym-and-official-title

Status: COMPLETE.
Landed: b40da7785f2954b15154563ade3df23ce7451856

Milestone: 7

## Outcome

Search ClinicalTrials.gov by its acronym and official title fields and return both fields in trial search rows with `--fields` support.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp search trial 'AREA[Acronym]SURMOUNT' --phase 3 --status COMPLETED --has-results --limit 50 --fields=title` returned `"total":3`. The positional search passes text to the registry’s general search. The reported roster needed both acronym and official title evidence. `biomcp search trial --intervention tirzepatide --sponsor 'Eli Lilly and Company' --phase 3 --status COMPLETED --has-results --limit 100 --fields=acronym,official_title,title` reported `"unplaced":["acronym"]` and suggested only `--fields=title`, which is the brief title.

## Proof

- Proof: On a saved registry set, search by an acronym and by distinctive official-title text, assert that each matches the stated field, and check both fields in a narrowed reply. A suggested field selection never silently substitutes the brief title for the official title.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ctgov/search_query.rs` and `crates/biomcp-adapters/src/ctgov/detail.rs`.

## Keeps

- Keeps: Free-text and other native registry filters retain their behavior.

## Defers

- Defers: Grouping trial records into programs automatically.

## Changes

- Changes: The ClinicalTrials.gov search request and row, help and cases.
