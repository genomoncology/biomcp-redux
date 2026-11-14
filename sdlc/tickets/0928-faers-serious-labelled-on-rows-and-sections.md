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
