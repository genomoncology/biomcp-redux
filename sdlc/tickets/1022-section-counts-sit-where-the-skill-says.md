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
