# 1010 — section-counts-use-one-field-name

Status: OPEN.

Milestone: 7

## Outcome

Every section that counts its whole set carries the count under one field name, the one the skill tells agents to read.

## Why

comp-bio read `.data.total` on the complexes section, got nothing, and reported 5 shown rows for WRN's 14 complexes; complexes and domains carry `native_total`, features `total`, and the skill says read `total` (bench `personas/comp-bio/runs/2026-11-16/analysis.md` 10.1; `crates/biomcp-types/src/protein/response.rs` 72-79).

## Proof

- Proof: a schema ratchet that no section has both names; the protein cases updated; release-notes line for the renamed field.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

## Progress

- Survey (every checked-in reply schema): about 140 schemas name a count `total`, and every one is the size of the whole set the reader could page through (the envelope's `page.total`, a card section's rows before card bounding, a capped list's whole length), never the rows shown. `native_total` appears only in protein domains and complexes, alone and as card sections, and means something else: for domains InterPro's count before `--type` selection, for complexes Complex Portal's text-search count before exact participant admission (`complex_portal/mod.rs`: "never an admitted membership count"; `sdlc/planning/slices/protein.md` says the same). Other names (`has_total`, `distinct_id_total`, `qualifying_total`, `regions_total`) sit in their own sections beside an envelope `page.total` and are outside this ticket.
- Why not rename: renaming `native_total` to `total` would hand agents a search count as the number of complexes whenever admission drops one, and removes a required field, which the 1.0 baseline forbids (the checker lets a major-version bump through, but the brief allows only added optional fields). The simplest fix that makes the skill's "Read its `total`" true for every section is to add `total` where it was missing.
- Change: `ProteinDomains` and `ProteinComplexes` gain an optional `total` (serde default, so the schemas gain one optional field), the rows in the whole selected set before paging, placed before `native_total` and the rows. The standalone commands already had it as `page.total`; a card's domains and complexes sections now carry it too. `native_total` keeps its meaning. Release notes say both.
- Ratchet: `every_object_with_a_native_total_also_states_its_total` in `crates/biomcp-types/tests/schemas.rs` fails if any registered schema has `native_total` without `total`. This replaces "no section has both names", because the two names count different things.
- Proof: `tests/cases/protein/card-section-totals.yaml` reads `get protein Q14191 domains complexes --section-limit 5` and expects complexes `total` 14 and domains `total` 17 beside 5 rows each, and `protein complexes Q14191 --limit 5` with `/result/total` 14 equal to `/page/total`. UniProt, InterPro and Complex Portal replies were recorded live in pm job j41965 (2026-11-16) and kept whole with `provenance.json`; Complex Portal admitted all 14 of its 14 search hits for WRN. The synthetic runner checks now assert `total` where the two counts differ: P04637 `--type domain` gives `total` 1, `native_total` 2; P38398 complexes give `total` 2, `native_total` 3.
- Cheap job j41967 on 1d701f81b: schemas regenerated, `make check-cheap`, the biomcp-types schema and response tests, and all 29 protein cases (including the synthetic domain and complex runners and mcp-parity protein forms) pass; no size budget needed raising. With the base crates (f34d6b2bd) the new case fails at step 1 (`complexes/data/total` missing).
- Gate j41974 on dae59f827 (contains origin/main f34d6b2bd; main had not moved): `make --keep-going landing-gate docs-check schema-release-check` passed; the schema comparison against the 1.0.0 baseline accepts the added optional field.
