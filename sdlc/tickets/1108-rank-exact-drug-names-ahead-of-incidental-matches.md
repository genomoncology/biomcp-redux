# 1108 — rank-exact-drug-names-ahead-of-incidental-matches

Status: OPEN.

Milestone: 7

## Outcome

An exact drug name or documented synonym ranks ahead of broader source search rows, with the source match basis visible.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp search drug adalimumab` returned `"name":"Isopropyl alcohol"` as the first item. The result looks like a ranked drug-name match for adalimumab. The exact match should be easy to find.

## Proof

- Proof: Replay a search where source results mix an exact named drug with an unrelated chemical. Assert the named drug appears first and the match basis is visible. Page the rest without repeats and check the count’s stated scope.

## Starts from

- Starts from: `search` in `crates/biomcp-query/src/drug.rs` and `crates/biomcp-adapters/src/mychem/mod.rs`.

## Keeps

- Keeps: Broader search matches remain available and source attribution is preserved.

## Defers

- Defers: Changes to search against a source the caller names explicitly.

## Changes

- Changes: Default drug search ranking and cases.
