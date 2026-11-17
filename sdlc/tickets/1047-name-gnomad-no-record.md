# 1047 — Name the gnomAD lookup when the allele has no record

Status: OPEN.

Milestone: 7

## Outcome

A confirmed gnomAD variant miss on the standalone population command or a variant card carries the queried dataset selector and exact allele key in its empty section. Markdown states that gnomAD found no variant record for that lookup.

## Why

The rare-disease bench's population calls for ClinVar 68552 (r1 call 38, r5 call 38 and r6 call 58) left agents to infer what an empty section with unrelated identity warnings meant. A live public reproduction returned HTTP 200 with `Variant not found` and `data.variant: null` for `gnomad_r4` and GRCh38 allele `2-165994236-A-G`.

## Proof

- Proof: the replay for ClinVar 68552 asserts the exact qualified key and no-record message through the population command and variant card. A substituted returned allele cannot produce the no-record claim. Run the variant population cases.

## Starts from

- Starts from: the gnomAD reply check in `gnomad/variant/raw.rs`, population section and detail query handlers, and the empty section record added to the shared Markdown renderer in “Empty sections name what was asked.”
- PM decision (2026-11-17): use `population.state: empty` with a `record` whose namespace is `gnomad_r4` to express the scoped no-record outcome. As “Read the gnomAD release from the same response as its counts” establishes, the selector carries no exact release assertion.

## Keeps

- Keeps: source problems and provenance, response schemas, available native counts and any same-response release, and the distinction between a reported count of zero and missing data. Lookup failures and unresolved identity do not claim a gnomAD miss.

## Changes

- Changes: `crates/biomcp-query/src/variant/population.rs`, `crates/biomcp-query/src/variant/population_detail.rs`, `crates/biomcp/src/view/mod.rs`, `tests/cases/variant/population/`, `tests/replies/variant-population/`, `docs/cautions.md`, and the changelog.
