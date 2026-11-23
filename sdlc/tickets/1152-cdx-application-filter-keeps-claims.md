# 1152 — cdx-application-filter-keeps-claims

Status: OPEN.

Milestone: 7

## Outcome

A companion-diagnostic search filtered by drug application returns every claim the FDA table lists for that application.

## Why

Found 2026-11-22 by a round 2 scout and confirmed in review: the typed NDA filter on `search diagnostic --source fda-cdx` missed an August 2025 tissue claim that is visible in the source row of FDA's companion-diagnostic table, while an unfiltered search showed it. A filtered search that drops a row reads as "no such claim".

## Changes

- The application filter matches every application a row names, in whatever spelling the table uses (with or without NDA/BLA prefix, several applications per cell).
- A row whose drug cell cannot be parsed is reported, not dropped.

## Proof

- Replay of the table with that row: the filtered search returns it; a row with two applications matches either.
