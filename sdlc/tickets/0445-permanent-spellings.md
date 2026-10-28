# 0445 — Fix permanent spellings before the tag

Status: OPEN.

Milestone: 5

## Outcome

PGx uses `annotation_kind`, not `annotationkind`. Semantic Scholar has one spelling across `ArticleKey`, `Source` and the `AuthorId` pattern. BioMCP-owned values use one case style; source-published values keep their source spelling.

- Why: the BioData review of BioMCP 1.0's data model (2026-10-28), ask 12. Ian ruled that incomplete or incorrect 1.0 data model types are fixed before the tag, because after it the release gate freezes every pattern, type set and schema ID.
- Starts from: `crates/biomcp-types/src/pgx/response.rs:118`, `ArticleKey`, `Source`, `AuthorId` and `Reason`.
- Rules: the case style follows the existing `Source` kebab-case unless the schema ID convention says otherwise.
- Proof: a lint over serde tag names and BioMCP-owned enum values.
- Changes: the named types, schemas, cases and the baseline.
- Defers: nothing.
