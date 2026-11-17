# 1020 — cell-line-disease-filter-says-it-is-exact

Status: OPEN.

Milestone: 7

## Outcome

`search cell-line --disease` says in a notice that Cellosaurus matches the exact disease name, so narrower terms are separate searches.

## Why

life-sciences-ci 10.3 (bench `personas/life-sciences-ci/runs/2026-11-16/analysis.md`).

## Proof

- Proof: a replayed case asserting the notice; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

## Progress

- Measured live 2026-11-16 (Cellosaurus `/search/cell-line`, rows up to 20000): `din` is not an exact-name match. A name matches those words in that order anywhere in an NCIt disease name, ignoring case: `din:"Melanoma"` gave 3,362 lines (2,235 Melanoma, 390 Cutaneous melanoma, 247 Mouse melanoma, 100 Uveal melanoma and more); `din:"Pancreatic adenocarcinoma"` gave 130, all labelled Pancreatic adenocarcinoma; `din:"Pancreatic ductal adenocarcinoma"` gave 333. An NCIt ID matches that concept only: C3224 (melanoma) gave the 2,235 Melanoma lines and no subtypes. So the notice says what is true rather than "exact": a name is a phrase match, no narrower NCIt disease is added, and a subtype whose name breaks the phrase, such as Pancreatic ductal adenocarcinoma, is its own search. This keeps the ticket's outcome (narrower terms are separate searches).
- Changed: `search cell-line` adds a Cellosaurus `request-guidance` notice whenever `--disease` is given (`crates/biomcp-query/src/cell_line/mod.rs`); the `--disease` help adds "a name matches as a phrase, without narrower NCIt diseases"; one release-notes line. No schema shape change.
- Proof: `tests/cases/cell-line/filters.yaml` step 1 (`--species "Homo sapiens" --disease Melanoma`) asserts the notice on existing replies; the AsPC-1 step without `--disease` asserts no notices.
