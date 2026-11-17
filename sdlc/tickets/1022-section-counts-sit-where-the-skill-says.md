# 1022 — section-counts-sit-where-the-skill-says

Status: OPEN.

Milestone: 7

## Outcome

Every card section that counts a set carries its whole-set count as a top-level `total`, as the skill tells agents to read it: study card sections (mutations, samples, files, profiles, sample lists), pathway card sections (genes, events) and cell-line card sections, which today carry it only at `data.page.total` or `data.items.total`. A schema check fails a section that holds rows and no top-level `total`.

## Why

1010's review (2026-11-16) found these sections nest their count, so an agent reading `.data.total` there makes the comp-bio mistake 1010 fixed for protein (bench `personas/comp-bio/runs/2026-11-16/analysis.md` fact 10.1). 1010's ratchet only checks `native_total` beside `total`.

## Proof

- Proof: the schema check above, tested on a planted bad schema; the study, pathway and cell-line cases assert top-level `total` equal to the nested one; `schema-release-check` accepts the added optional fields.

## Starts from

- Starts from: 1010's change in `crates/biomcp-types/src/protein/response.rs` and `crates/biomcp-types/tests/schemas.rs:552-592`.

## Keeps

- Keeps: the nested `page.total` and `items.total`, unchanged.

## Defers

- Defers: nothing.

## Changes

- Changes: those sections' reply types and schemas, the schema check, cases, one release-notes line.

## Progress

- Survey: a check over every checked-in reply schema for card sections (a section's `data` variant) whose data holds `items` and no top-level `total` found exactly the ticket's list plus one more: study profiles, sample lists, files, samples and mutations; pathway genes and events; cell-line names, xrefs, variants and ChEMBL; and protein function. Cell-line identity holds three separately counted sets (species, diseases, warnings), each already a `{total, items}` pair, so it has no single set to count and is left as is.
- Change: each of those 12 section types gains an optional `total` (serde default, so each schema gains one optional field), placed before the rows, equal to the nested `page.total` or `items.total`, which stay. Protein function's `total` is its paragraph count; all paragraphs are shown. The same types are also the standalone section replies, so `get study ID samples` and the like carry `result.total` too. Pathway relations and cell-line cards copy the nested count into `total` when they page a saved pool or cached card, so a pool saved before this change still answers with the right count.
- Check: `every_section_with_rows_states_its_total_at_the_top` in `crates/biomcp-types/tests/schemas/section_totals.rs` fails any registered schema with a section whose data has `items` and no `total`; it first proves itself on a planted bad schema. 1010's `native_total` check moved into the same file, because `schemas.rs` hit its file line limit.
- Proof: new case `tests/cases/study/card-section-totals.yaml` reads `get study skcm_tcga_pan_can_atlas_2018 all --gene BRAF --section-limit 2` and expects `total` 13 profiles, 11 sample lists, 49 files, 245 mutations and 448 samples, each equal to `page.total`, beside 2 rows. The cBioPortal and GitHub replies were recorded live in pm job j42123 (2026-11-16); the 552-study catalog keeps only the SKCM row, and `provenance.json` gives the original hashes. Existing cases now also assert the top-level count: `pathway-sections/all` (genes 332, events 81), `pathway/empty-relations` (0, alone and on the card), `cell-line/native-all` (names 7, xrefs 1398, variants 2, ChEMBL 1) and `cell-line/k562-xrefs` (1398). Values for the replayed cell-line and pathway cards were read in pm job j42120.
- Cheap job j42125 on 24badf647: schemas and docs regenerated (committed), the touched cases (study, pathway, pathway-sections, cell-line, protein, cbioportal cache-admission) and the biomcp-types schema and response tests pass, and `schema-release-check` accepts the added optional fields. `check-cheap` failed on two limits: the cellosaurus card function went to 85 lines (limit 80) and `schemas.rs` passed its file line limit; both fixed by moving code, not by raising limits. Job j42130 then failed the ratchet that keeps `#[test]` functions in listed files, so the two tests stay in `schemas.rs` and call the module.
- Gate j42133 on dd29a745c failed at the secret scan: the pretty-printed samples and mutations recordings read as generic API keys (cBioPortal's base64 `uniqueSampleKey` values). Those two bodies are now stored as compact JSON text, as `tests/replies/study/rows` stores the same replies; their hashes in `provenance.json` are unchanged.
- Gate j42136 on 88ea02891 failed the same scan, because it reads the branch history and an earlier commit still held the pretty-printed bodies. The branch is squashed to one commit (3eec6335b, same tree) and merged with origin/main 152cd54a9.
