# 1071 proposal: `--fields` chooses columns in sections

2026-11-19. Measured on a debug build of main 991b0ab93 (pm jobs j43300, j43302), live sources, with `sdlc/planning/1071-measure.py`. Bytes are the whole JSON reply. "After" keeps the named columns plus the always-kept ones below; it is computed from the measured reply, and the two searches that already take `--fields` match their measured runs within 0.5%. Diagnostic rows came from the be96d8d40 build because GTR failed from the build machine; GenCC is from j43300 because it was partial in j43302.

## How `--fields` works on search rows today

`fields.rs` reads the reply schema; a row is `result.items[]` and its names are its top-level keys. Always kept: required and identifier (`x-identifier`) keys, so a narrowed reply validates. A nested object (a diagnostic `header`) is chosen whole; an inner name is refused with `next` naming the holding field. Narrowing runs on the finished reply, so `next` steps are built from full rows; search page steps carry `--fields`. The reply does not echo the fields. Every MCP entity tool has a search action and so already has a `fields` array. Any non-search refuses ("chooses columns of search rows only").

## Measured rows

| List (call) | B/row | reply | columns asked (always kept added) | B/row after | reply after |
|---|---|---|---|---|---|
| Trial locations (NCT04613596, 100) | 274 | 30,551 | facility, city, state, status | 114 | 14,554 (48%) |
| same, `--lat 39.96 --lon -83.00` | 293 | 32,502 | + distance_mi | 135 | 16,725 (51%) |
| Trial arms (10) | 205 | 4,902 | data.label, data.kind (+record) | 71 | 3,559 |
| Drug approvals (NDA018936, 50) | 645 | 35,368 | event.status, event.dates (+submission_number, event.record, event.attribution) | 224 | 14,313 (40%) |
| Drug label search (50), already works | 300 | 21,769 | name, effective_date (+id, source, identity_level) | 157 | 14,640 measured |
| Pathway genes (R-HSA-162582, 100) | 222 | 28,889 | name (+reference, physical_context_count, gene) | 128 | 19,541 (68%) |
| Pathway events (100) | 250 | 26,445 | id, name (+kind, db_id) | 133 | 14,685 (56%) |
| Protein features (P15056, 50) | 418 | 21,834 | feature.kind, feature.span (+feature.protein, attribution, source_coordinates) | 324 | 17,147 (79%) |
| Adverse-event search (50), already works | 117 | 11,192 | serious_label (+id, source) | 44 | 7,542 |
| Adverse-event drugs (one FAERS report, 50) | 435 | 23,243 | medicinalproduct, role_label, indication (+index, identity_counts) | 265 | 14,743 (63%) |
| Diagnostic search (GTR, BRAF, 50) | 683 | 43,707 | header.name, header.laboratory.name (+identity_status, header.id, header.url) | ~220 | ~20,600 (47%) |
| Diagnostic genes (GTR panel, 50) | 210 | 13,771 | gene (+test_id, mim_number, object_name) | 154 | 10,971 (80%) |
| Gene ClinGen validity (BRAF, 4) | 366 | 4,190 | disease, classification (+mondo_id, mode_of_inheritance) | 122 | 3,214 |
| Gene GenCC (BRAF, 23) | 376 | 10,904 | disease, classification, submitter (+id, version) | 132 | 5,292 (49%) |

Three findings. Required columns set the floor: protein features (attribution, 150 B/row, is required) and GTR genes and conditions (test_id repeats the parent on every row) save little. Text outside the rows is untouched: a trial locations page spends about 3 KB outside its rows (card, sources, `next`), and `get drug imatinib indications --source opentargets` spends 43,894 of 66,900 bytes on its regulatory context, not on its 50 indication rows. Nested rows (trial arms, approvals, protein features, diagnostic headers) need paths, or top-level names cut nothing.

## Proposal

1. **Naming.** A field is a row's key as the reply spells it, or a dotted path to a value inside an object of the row (`event.status`, `data.label`, `header.name`); a path never crosses a list. The same rule serves search rows, so `header.name` starts working there too. Help lists each section's top-level names and, for an object field, its inner names in parentheses.
2. **Rows.** Every list of objects inside a named section's `data` (`Section_*` in the schema) is a row list, and so are `result.items` of the subcommands (`pathway genes`, `protein features`). The section's own frame (`state`, `source`, `page`, `total`, `continuation`) and the card are never cut.
3. **Always kept.** The row's required and identifier keys, as on search rows; for a path, the required keys of each object it passes through. `next` steps are built before narrowing, so a pivot never loses its identity; page steps of a section carry `--fields` as search page steps do.
4. **The reply says which fields were asked for.** The envelope gains an optional `fields` list after `page`, the names as accepted; absent without `--fields`, so a reply without it is byte-for-byte unchanged. Searches get it too.
5. **Several sections.** One list per call. Each requested section keeps the names its rows have; a section with rows but none of the names stays whole, and text sections (eligibility, summary) are unaffected. A name no requested section has is refused. (Alternative: cut such a section to its always-kept keys; smaller, but it drops data the caller did not name.)
6. **Refusal.** `--fields` on a call with no row list keeps today's refusal, reworded to "rows of a search or a section". An unknown name uses the existing `usage.arguments` with `error.option` `--fields`, the fixed message `FieldsUnknown` reworded from "top-level row fields" to "row fields", `valid_values` the requested sections' top-level names (9 for locations), and `next` the call with each name moved to its nearest, as today.
7. **Schemas.** Only optional keys are dropped, so every narrowed reply validates against today's row schemas; no row field becomes optional. The one schema change is the envelope's optional `fields`, additive in every reply schema. Making a heavy required field optional (protein attribution, GTR test_id) is a separate decision; I do not propose it here.
8. **MCP.** No new property and no tool description text: `fields` is already on all 13 entity tools and now applies to `get` and subcommand actions. Its input schema gains a short description ("Row columns to keep; help lists each list's names"), about 50 characters per tool; tool description text does not change, so each stays where it is under 3,500.
9. **Order.** One schema-driven mechanism, enabled per command by a list. Batch 1: trial sections, drug sections (approvals, indications, targets, interactions, label references), pathway genes and events (section and subcommand), with help and cases. Batch 2: protein, gene, adverse-event and diagnostic sections. Batch 3: disease, variant, study, cell line, article lists.

## Cases (replay fixtures)

- `get trial NCT04613596 locations --fields facility,city,state,status,distance_mi --limit 100 --lat 39.96 --lon -83.00`: only those columns, under half the bytes, `fields` echoed, the page step carries `--fields`.
- `get drug NDA018936 approvals --source drugsfda --fields event.status,event.dates`: rows keep `submission_number`, `event.record`, `event.attribution`; the reply validates.
- `get pathway R-HSA-162582 genes --fields name` and `pathway genes R-HSA-162582 --fields name`: `reference`, `gene` and the per-gene `next` pivots stay.
- `get trial NCT04613596 --section locations --section arms --fields facility`: locations narrowed, arms whole.
- `--fields city,zipcode` on locations: refused with `FieldsUnknown`, `valid_values` the 9 location names, `next` the call with `zip`.
- `get trial NCT04613596 --fields title` (no section): refused, reworded message.
- A call without `--fields` is unchanged (existing cases hold); `search diagnostic --fields header.name` works on a search.
