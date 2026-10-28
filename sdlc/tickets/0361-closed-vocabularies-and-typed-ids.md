# 0361 — Map closed source vocabularies to enums and keep IDs and dates typed

Status: OPEN.

Milestone: 5

Depends on: 0341, 0360

## Outcome

Fields whose source publishes an authoritative, complete value list are enums that keep the native text: cell-line `sex` from the Cellosaurus XSD and CIViC `status`, `kind` and `significance` from the CIViC GraphQL schema. NCI trial `phase` and drug regulatory `status` and `kind` become enums only if the reviewer finds such a list in official documentation; otherwise they stay native source text. Cell-line `age` stays native source text, because the Cellosaurus XSD defines it as an optional attribute with no enumeration. A value outside a set keeps its native text and records a problem. `cosmic_id`, `mondo_id` and `nci_thesaurus_concept_id` use typed IDs, `ComponentUpload.date` uses a date type, and the plain `String` fields in `research.rs` and `variant_normalize.rs` use the crate's text types.

- Why: the docs lead's product findings report (2026-10-27), items D3 and D4. 1.0 freezes the schemas, so type changes are cheapest now. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: `response/nci_trial.rs`, `biodata/src/cell_line/records.rs`, `response/variant_civic.rs`, `drug/regulatory.rs`, `predictor.rs`, `gene/clingen.rs`, `research.rs` and `variant_normalize.rs`.
- Rules: each enum's members come from the source's authoritative, complete published value list, cited in the ticket; observed or example values never define an enum (PM ruling 2026-10-28, m12048). `TrialPhase` takes NCI values only if NCI publishes them as the same set; exact matching only, as `2026-10-24-no-text-heuristics.md` allows. An unknown value never fails the item.
- Proof: cases for each field show a known value as its enum member and an unknown value kept with a problem; the baseline is regenerated.
- Changes: the named types, adapters, schemas and the baseline.
- Defers: nothing.
