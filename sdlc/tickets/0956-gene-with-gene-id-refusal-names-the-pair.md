# 0956 — gene-with-gene-id-refusal-names-the-pair

Status: COMPLETE.
Landed: e2b48cae5407c143684471da4f5d9d180ef34f7a

Milestone: 7

## Outcome

`search diagnostic --gene X --gene-id N` is refused with a message that names both options and says to use one, not with generic guidance on `--gene` alone that reads as "the gene looks invalid".

## Why

0949's review (2026-11-14): `crates/biomcp-types/src/query/diagnostic/mod.rs:206-207` returns the fields `["gene","gene_id"]` with no message, and the CLI renderer (`crates/biomcp/src/grammar/validation.rs:15-25`) shows only the first field with `ErrorMessage::ArgumentGuidance`. That is the misleading refusal 0949 was opened about (dx-lab-director round 2 call 14), now for the one GTR combination still refused.

## Proof

- Proof: a case runs `search diagnostic --source gtr --gene EGFR --gene-id 1956` and shows a refusal naming both `--gene` and `--gene-id`; it fails on main.

## Starts from

- Starts from: the lines above; other validation checks that use `.with_message(...)`.

## Keeps

- Keeps: 0949's gene-with-condition join; the refusal itself (no new combination accepted).

## Defers

- Defers: nothing.

## Changes

- Changes: diagnostic search validation, a case under `tests/cases/diagnostic/`.

## Progress

- The refusal of `--gene` with `--gene-id` now carries its own message: "--gene and --gene-id each name one gene, by symbol or by NCBI GeneID; give one of them, not both". The option shown stays `--gene` and the refusal itself is unchanged; no new combination is accepted.
- The new message widens the error message enum in `crates/biomcp-types/schemas/error.json` (regenerated in j41048; rustfmt made no change there).
- Case `diagnostic/gtr-gene-condition.yaml` gains an offline step for `search diagnostic --source gtr --gene EGFR --gene-id 1956` that expects the new message; on main it gets the generic argument guidance. No source is called, so no reply was recorded.
