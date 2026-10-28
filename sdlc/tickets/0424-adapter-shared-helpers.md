# 0424 — Share repeated adapter and query helpers

Status: OPEN.

Milestone: 5

Depends on: 0418

## Outcome

Shared helpers replace copied code: `SourceError::bad_reply`, one openFDA envelope, one GraphQL alias batch type with a runtime size, `Problems::field`, one `received_date`, one XML root search, one component-disable check, and a bundle partition key kind in place of `KeyKind::Band` misuse. Variant get runs its sections through `fan_out::run`. The trial and evidence snapshot tables become one table with one expiry constant and one disabled-source rule.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`.
- Starts from: the openFDA adapters (fda_510k, fda_pma, maude, drugsfda, openfda_label, faers), civic, gnomad, `received.rs`, `drug_sources/date.rs`, clinvar, pubmed, the bundle text-search adapters, `crates/biomcp-query/src/variant/sections.rs`, `store/trial.rs`, `store/evidence.rs`.
- Rules: no behavior change except the snapshot rule and variant timing, which get cases; schedule after 0412, 0413 and 0416 so openFDA work does not collide.
- Proof: existing cases pass; a variant get case shows sections run in parallel within the deadline; a snapshot case covers a disabled source.
- Changes: the files above and cases.
- Defers: nothing.
