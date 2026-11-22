# 1139 — search-curations-by-source-title

Status: OPEN.

Milestone: 7

## Outcome

A caller can find gene-disease curations by a literal prefix of the publisher's own disease title, across ClinGen and GenCC, in one paged query with field projection.

## Why

Task dx-1b (2026-11-22) asks for GenCC submissions under GenCC's own disease titles. BioMCP's curation lookup matches an exact ID or exact title only, so an umbrella term returns only its own curation. Connector-only agents enumerated 20 to 23 subtype IDs and fetched each one, up to 105 calls in one run, and still missed the GenCC titles. Lists of IDs and projections onto the validity rows were refused (25 field errors: the `gene-validity` rows are not in the field registry). The held source data for that task has 17 ClinGen rows and 123 GenCC submissions under 22 titles beginning "Fanconi anemia". Code: `crates/biomcp-adapters/src/curation.rs`, `crates/biomcp-adapters/src/gencc/by_disease.rs`, `crates/biomcp-types/src/response/disease_validity.rs:10-70`, `crates/biomcp/src/fields/section_rows.rs`.

## Changes

- An explicit source-title filter (literal prefix, case-insensitive) on the disease curation path, paged, per source.
- The validity rows join the field registry, so `--fields gene_symbol,assertion` works.
- Exact ID and exact title behaviour is unchanged. A prefix selects text; it does not claim disease equivalence.

## Proof

- With held fixtures, the exact lookup misses the family; the new filter returns every row a direct literal column filter returns, including the comma variant, a misleading title in another column, and unrelated titles sharing a word. Check classifications, submitters, IDs, dates, source coverage, partial availability and the projection.

## Defers

- MONDO descendant expansion: it answers a different question and is not shown to reproduce GenCC's titles.
- General disease-ID batching, until a case still needs it after this.
