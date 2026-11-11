# 0829 — gtr-search-by-test-name

Status: OPEN.

Milestone: 8

## Outcome

`search diagnostic --source gtr --name TEXT` finds genetic tests by literal test name across the synced catalog.

## Why

Lab directors look tests up by name; the command refuses `--name` for this source though the names are in the local bundle. Replaces idea 0819.

## Proof

- Proof: a routine case under `tests/cases/diagnostic/` finds a known test by part of its name from the fixture bundle; no match answers empty.

## Starts from

- Starts from: `crates/biomcp-types/src/query/diagnostic/search_validation.rs`, which refuses the option, and `crates/biomcp-adapters/src/gtr/`.

## Keeps

- Keeps: every existing field, schema ID rule (a changed reply schema takes a new major), source attribution and page behavior; no new source, key or dependency. Literal, case-insensitive matching; no fuzzy ranking.

## Defers

- Defers: matching tests to FDA companion diagnostics is declined for 1.1 (no shared identifier).

## Changes

- Changes: `crates/biomcp-adapters/src/gtr/`, `crates/biomcp-types/src/query/diagnostic/`, `tests/cases/diagnostic/`
