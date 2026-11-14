# 0942 — inner-page-totals-come-before-rows

Status: OPEN.

Milestone: 7

## Outcome

The reply types that still write their own page or total after their rows do so before them: variant hotspots and OncoKB pages, study pages, GWAS association and study pages, and the pages built by the shared `page!` macro in `response_page.rs` (study, pathway and others). Field order only, as 0937 and 0939 did.

## Why

0939's review (2026-11-14): 0939 moved the envelope's `page` and the drug and adverse-event totals ahead of their rows, but these families' own page blocks still follow their rows, so an agent reading only the start of a reply sees rows without their count. The dx-lab-director persona's first-page misreads (bench `personas/dx-lab-director/runs/2026-11-14/analysis.md`) are the reason for the series.

## Proof

- Proof: 0939's `starts_within` runner check on one recorded case per family, and a contract test per type in `crates/biomcp-types/tests/response.rs`, each failing on main.

## Starts from

- Starts from: 0939's change, its `totals_first` helper and `crates/biomcp/tests/runner/assertions/offsets.rs`; `response_page.rs`; the variant, study and GWAS response types.

## Keeps

- Keeps: no schema file changes; MCP text order (alphabetical).

## Defers

- Defers: nothing.

## Changes

- Changes: field order in those types, tests.
