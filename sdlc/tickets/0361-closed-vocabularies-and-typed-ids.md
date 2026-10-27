# 0361 — Map closed source vocabularies to enums and keep IDs and dates typed

Status: OPEN.

Milestone: 5

Depends on: 0341, 0360

## Outcome

Fields whose source publishes a closed value set are enums that keep the native text: NCI trial `phase` uses `TrialPhase`, and cell-line `sex` and `age`, CIViC `status`, `kind` and `significance`, and drug regulatory `status` and `kind` get enums. A value outside the set keeps its native text and records a problem. `cosmic_id`, `mondo_id` and `nci_thesaurus_concept_id` use typed IDs, `ComponentUpload.date` uses a date type, and the plain `String` fields in `research.rs` and `variant_normalize.rs` use the crate's text types.

- Why: the docs lead's product findings report (2026-10-27), items D3 and D4. 1.0 freezes the schemas, so type changes are cheapest now. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: `response/nci_trial.rs`, `biodata/src/cell_line/records.rs`, `response/variant_civic.rs`, `drug/regulatory.rs`, `predictor.rs`, `gene/clingen.rs`, `research.rs` and `variant_normalize.rs`.
- Rules: each enum's members come from the source's published value list, cited in the ticket; exact matching only, as `2026-10-24-no-text-heuristics.md` allows. An unknown value never fails the item.
- Proof: cases for each field show a known value as its enum member and an unknown value kept with a problem; the baseline is regenerated.
- Changes: the named types, adapters, schemas and the baseline.
- Defers: nothing.
