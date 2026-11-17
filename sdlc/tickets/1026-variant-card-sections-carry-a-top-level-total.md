# 1026 — variant-card-sections-carry-a-top-level-total

Status: OPEN.

Milestone: 7

## Outcome

The variant card sections assertions, CIViC, hotspots, OncoKB and GWAS carry their whole-set count as a top-level `total`, equal to their nested `page.total`, and 1022's schema check treats any section with a counted page as holding rows, so no section that counts a set lacks a top-level `total`.

## Why

1022's review (2026-11-16) found its check keys only on an `items` field (`crates/biomcp-types/tests/schemas/section_totals.rs:61`), so these five sections, whose rows sit under `assertions`, `profiles`, `records` and `associations`, still keep their count only in `page`. An agent reading `.data.total` there repeats the comp-bio mistake (bench `personas/comp-bio/runs/2026-11-16/analysis.md` 10.1).

## Proof

- Proof: the widened check fails on the base commit naming these five types and passes after; the variant cases assert each top-level total equals the nested one; `schema-release-check` accepts the added optional fields.

## Starts from

- Starts from: 1022's `section_totals.rs` and the five variant section types.

## Keeps

- Keeps: the nested `page.total`.

## Defers

- Defers: nothing.

## Changes

- Changes: the five section types and schemas, the check, variant cases, one release-notes line.
