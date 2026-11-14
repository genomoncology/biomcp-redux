# 0928 — faers-serious-labelled-on-rows-and-sections

Status: OPEN.

Milestone: 7

## Outcome

FAERS search rows and the `drugs` and `reactions` section replies of `get adverse-event <id>` carry openFDA's label for the report's `serious` flag beside the code, as the FAERS card does since 0916. MAUDE and VAERS rows are unchanged.

## Why

0916's review (2026-11-14): the card now labels `serious`, but search rows and the section replies share a header that still shows the bare `1` or `2`. The drug-safety persona's round 2 scan (bench `personas/pv-scientist/runs/2026-11-13-r2/log.md:79-81`) read the reactions section of `get adverse-event <id>`, so an importer still needs openFDA's 1/2 rule there.

## Proof

- Proof: recorded FAERS cases for a search page and a reactions section assert `serious_label` with openFDA's two sentences (from 0916's `SERIOUS` list); a MAUDE search case shows no such field.

## Starts from

- Starts from: 0916's `SERIOUS` list in `crates/biomcp-adapters/src/faers/words.rs` and its card label at `crates/biomcp-query/src/adverse_event/get/render.rs:178`; the shared header in `crates/biomcp-types/src/adverse_event/response.rs:28-30`. 0916's first attempt put the label on the shared header and the search help then offered it as a `--fields` name that MAUDE rows would never fill (j40179), so label FAERS rows and sections without making it a field other sources list.

## Keeps

- Keeps: the 1.0.0 schema rules (optional fields only); search `--fields` lists true for each source.

## Defers

- Defers: nothing.

## Changes

- Changes: FAERS search and section rendering, their schemas (optional field), cases.

## Progress

- `serious_label` moved into the shared report header (`AdverseEventHeader`), beside `serious`, with 0916's `SERIOUS` words. FAERS fills it on search rows, the card and every section (drugs, reactions, identifiers); MAUDE builds its header with no `serious` and no label. The card's own top-level `serious_label` from 0916 is gone, because the header now carries it; neither field is in the 1.0.0 baseline, which gains only an optional header field.
- Search `--fields` help now lists fields by source, the way trial search does: `faers: first_received, id, serious, serious_label, source, version; maude: event_type, first_received, id, source`. The old single list also offered `serious` and `version` to MAUDE and `event_type` to FAERS, none of which those rows ever fill. The per-source names live in `AdverseEventHeader::ROW_FIELDS`, and a unit test fails if a header field is missing from every source's list or a listed name is not a header field. Asking `--source maude --fields serious_label` is still accepted and returns rows without it, as `serious` did before.
- Proof from recorded real openFDA replies: `serious-any` (pembrolizumab row 26940567, `serious` `1`) and `faers-widened-names` (apixaban row, `serious` `2`) assert each sentence on a search row; `faers-recorded-labels` asserts it in the header of the card, of the drugs section of 22814945 (`2`) and of the reactions section of 27298481 (`1`); `maude-event-type` asserts its MAUDE row has neither `serious` nor `serious_label`.
