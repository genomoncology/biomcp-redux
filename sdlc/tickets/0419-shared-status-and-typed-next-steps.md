# 0419 — Share status rules and build next steps from types

Status: OPEN.

Milestone: 5

## Outcome

`item_status` and `search_status` from architecture 6.11 exist once in biomcp-query and every entity uses them, so the same condition gives the same status everywhere. Next steps are built with typed constructors, never `serde_json::from_value(json!(..))` with failures dropped. Repeated `empty`, `run` and `page_info` helpers become one each.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`. Five `overall` and five `finish` copies disagree on whether a rejected field makes a reply partial.
- Starts from: `crates/biomcp-query/src` (pathway, phenotype, disease, variant, study, diagnostic, drug, trial, pgx, gwas, adverse_event).
- Rules: one status rule set, written down in architecture 6.11; status changes that this makes visible are listed in the ticket record.
- Proof: a case per entity family shows the shared rule; renaming a query field breaks the build instead of dropping a next step.
- Changes: `crates/biomcp-query/src/status.rs`, the entity modules and cases.
- Defers: nothing.
