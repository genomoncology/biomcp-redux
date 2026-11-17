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
- Proof: `tests/cases/cell-line/filters.yaml` step 1 (`--species "Homo sapiens" --disease Melanoma`) asserts the notice among Cellosaurus's notices on existing replies (replay also carries the count-failure notice, since the count reply is not recorded); the AsPC-1 step without `--disease` asserts its notices exclude it.
- Job j42073 on 3bbace219: schemas and docs regenerated (descriptions only, applied unchanged); `mcp/parity` and `mcp-parity` cases passed; with base crates `cell-line/filters` fails (no notice). It failed the new assertion (notice index 0 held the count-failure notice) and the structural ratchet (an `if let Some` source lookup); both fixed: the notice is now attached as the source row is built.
- Job j42080 on 6b33ab26c: `cargo fmt` and regeneration changed nothing, `make check-cheap` passed; `cell-line/filters` step 4 failed because that reply already carries another Cellosaurus notice, so the step now asserts the disease notice is absent instead of no notices.
