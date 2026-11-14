# 0928 — faers-serious-labelled-on-rows-and-sections

Status: COMPLETE.
Landed: 36dfb6598241ae5fb1b556cf0093cd6c8efcf166

Milestone: 7

## Outcome

FAERS search rows and the `drugs` and `reactions` section replies of `get adverse-event <id>` carry a short label for the report's `serious` flag beside the code (`serious` for `1`, `not serious` for `2`, the meaning of 0916's `SERIOUS` list). openFDA's full definition appears once per reply, in the field's description, not on every row. MAUDE and VAERS rows are unchanged.

## Why

0916's review (2026-11-14): the card now labels `serious`, but search rows and the section replies share a header that still shows the bare `1` or `2`. The drug-safety persona's round 2 scan (bench `personas/pv-scientist/runs/2026-11-13-r2/log.md:79-81`) read the reactions section of `get adverse-event <id>`, so an importer still needs openFDA's 1/2 rule there.

## Proof

- Proof: recorded FAERS cases for a search page and a reactions section assert `serious_label` as `serious` or `not serious`; the field description quotes openFDA's two sentences; a MAUDE search case shows no such field.

## Starts from

- Starts from: 0916's `SERIOUS` list in `crates/biomcp-adapters/src/faers/words.rs` and its card label at `crates/biomcp-query/src/adverse_event/get/render.rs:178`; the shared header in `crates/biomcp-types/src/adverse_event/response.rs:28-30`. 0916's first attempt put the label on the shared header and the search help then offered it as a `--fields` name that MAUDE rows would never fill (j40179), so label FAERS rows and sections without making it a field other sources list.

## Keeps

- Keeps: the 1.0.0 schema rules (optional fields only); search `--fields` lists true for each source.

## Defers

- Defers: nothing.

## Changes

- Changes: FAERS search and section rendering, their schemas (optional field), cases.

## Progress

- PM direction 2026-11-14: a short label on each FAERS row and section, openFDA's full definition once per reply rather than per row, and caps unchanged unless the short label alone exceeds them. An earlier build of this branch put openFDA's full sentence on every row; measured in j40394, a five-row synthetic search page grew from the 2,392-byte cap to 3,136 bytes, so it was dropped.
- `serious_label` now sits in the shared report header (`AdverseEventHeader`) beside `serious`, with the words of a new `SERIOUS_SHORT` list (`1` serious, `2` not serious) drawn from 0916's `SERIOUS`. FAERS fills it on search rows, the card and every section (drugs, reactions, identifiers); MAUDE builds its header with no `serious` and no label. The header field's description quotes openFDA's two sentences for `1` and `2`, so the definition appears once in each reply's schema and costs no reply bytes. 0916's card-level `serious_label` (the full sentence) is gone, because the header now labels the card too. Neither field is in the 1.0.0 baseline, which gains only an optional header field; `schema-release-check` passed in j40331.
- Search `--fields` help now lists fields by source, as trial search does: `faers: first_received, id, serious, serious_label, source, version; maude: event_type, first_received, id, source`. The old single list also offered `serious` and `version` to MAUDE and `event_type` to FAERS, which those rows never fill. The per-source names live in `AdverseEventHeader::ROW_FIELDS`; a contract test in `crates/biomcp-types/tests/adverse_event_contracts.rs` fails if a header field is missing from every source's list or a listed name is not a header field. `--source maude --fields serious_label` is still accepted and returns rows without it, as `serious` did before.
- Measured sizes with the short label (j40396): every `get adverse-event` card and section stays within its existing measured +15% ceiling (card 1,673/1,551 bytes against measured 1,647/1,525; drugs 1,858/1,736 against 1,832/1,710; identifier sections about 26 bytes over their measured value, inside 15%), so those caps are unchanged. Three search caps were exceeded by the short label alone, by 26 bytes a row: the synthetic five-row search envelope measured 2,462/2,340 bytes against caps 2,392/2,260 (70/80 over), now measured +15% = 2,832/2,691; `adverse-event/examples.yaml` step 1 measured 1,881 against 1,751 (130 over), cap now 1,881, and step 2 measured 2,036 against 1,906 (130 over, j40423), cap now 2,036 with its token cap following the same size, 477 to 509; `serious-any.yaml` measured 1,264 against 1,238 (26 over), cap now 1,264. These case caps keep their files' exact-measure convention.
- Proof from recorded real openFDA replies: `serious-any` (pembrolizumab row 26940567, `serious` `1` → `serious`) and `faers-widened-names` (apixaban row, `serious` `2` → `not serious`) on search rows; `faers-recorded-labels` in the header of the card, of the drugs section of 22814945 (`not serious`) and of the reactions section of 27298481 (`serious`); `maude-event-type` asserts its MAUDE row has neither `serious` nor `serious_label`.
- Gate j40450 (`make --keep-going landing-gate docs-check schema-release-check` on 9c5a6990b, branched from origin/main at 0c0f6547f) passed, exit 0, 2026-11-14. Later commits change only this ticket file.
- Merged origin/main after 0923 landed (its say-once help lines kept, with this ticket's per-source `--fields` line). `scripts/refresh_help_cases.py` in job j40499 re-measured `help search adverse-event` at about 6,977 bytes (0923 measured 6,880; the per-source list adds 96): its cap goes 7,914 → 8,024 (measured + 15%, 0905). No other help cap changed.
- Gate j40500 (`make --keep-going landing-gate docs-check schema-release-check` on 97001fa3c, merged with origin/main) passed, exit 0, 2026-11-14. Later commits change only this ticket file.
