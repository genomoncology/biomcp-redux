# 1190 — gwas-empty-trait-hint

Status: COMPLETE.
Landed: 41f32c8683e4b85b721bf20da5ed4f83b3a0803b

Milestone: 7

## Outcome

`biomcp search gwas --trait "type 2 diabetes"`, a trait search that comes back empty, gives a next step. The step says `--trait` matches only the exact GWAS Catalog trait label, and it gives a disease search for finding the ontology ID to pass as `--trait-id`. A trait search with rows is unchanged.

## Why

A public report against 0.9.3 found the documented trait examples return nothing. On 1.0 main (23d82a4c3), `--trait "type 2 diabetes"` returns `complete` with 0 rows and no next step, while `--trait "type 2 diabetes mellitus"` and `--trait-id MONDO_0005148` each return 9,655 associations. The help already says `--trait` is an exact `efo_trait` label, but a reader who gets an empty page has no way forward.

## Starts from

- Starts from:
  - `crates/biomcp-query/src/gwas/mod.rs` `search`, and the next-step code in `crates/biomcp-query/src/gwas/get.rs` and `gwas/get/next.rs`;
  - `crate::next::Step`.

## Decisions

- PM decision (2026-11-27): no automatic free-text resolution. The search still sends the text as given; only the empty reply gains a next step.
- PM decision (2026-11-27): the step offers a real, runnable disease search on the same text that returns ontology IDs (choose the existing disease command and source that do this). Wording names `--trait-id`.

## Keeps

- Keeps: request plans, non-empty replies, `--trait-id` behavior, and the help text unless it is wrong.

## Proof

- Proof: an outside-in case under `tests/cases/` with a recorded empty GWAS Catalog reply for `--trait "type 2 diabetes"` asserts the next step and its command; an existing non-empty trait case is unchanged. `make landing-gate`.

## Defers

- Defers: resolving free text to an ontology ID inside the search (not planned).

## Changes

- Changes: `crates/biomcp-query/src/gwas/`, `tests/cases/`.
