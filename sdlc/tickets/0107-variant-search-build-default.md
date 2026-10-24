# 0107 — Stop inferring a genome build in variant search

Status: OPEN.

Milestone: 2

## Outcome

`VariantSearch::build()` no longer defaults to GRCh37. A region search without `--build` is a usage error. A gene and protein search sends no assembly unless the caller gives `--build`, and labels returned coordinates with the assembly MyVariant states for them.

- Why: the slice rule says coordinates always carry their build and no code infers one; the comparison of both biodata crates found this default at `crates/biodata/src/query/variant_search.rs` line 186.
- Proof: a region case without a build asserts usage; a gene and protein case asserts the echoed query has no inferred build and each returned coordinate carries the source-stated assembly; existing variant search cases pass or update their echo. Then make landing-gate runs through pm job after current main.
