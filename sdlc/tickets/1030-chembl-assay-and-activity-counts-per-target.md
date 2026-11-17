# 1030 — chembl-assay-and-activity-counts-per-target

Status: COMPLETE.
Landed: 6290ca5431125997c06def2f2373bb1e85a45351

Milestone: 8

## Outcome

A gene or protein section maps the UniProt accession to its ChEMBL target and gives that target's assay and activity totals.

## Why

comp-bio 7.1 (bench `personas/comp-bio/runs/2026-11-16/analysis.md`); only `cell-line assay-count` exists today.

## Proof

- Proof: a replayed case for one of comp-bio's targets asserting both totals.

## Starts from

- Starts from: an existing adapter of the same shape, and the source's own documentation and terms.

## Keeps

- Keeps: every existing command form.

## Defers

- Defers: nothing.

## Changes

- Changes: a new adapter or section, its help, schemas, cases, recordings and a release-notes line.

## Progress

- Source and terms: ChEMBL is already a BioMCP source (`chembl`, CC BY-SA 3.0, attribution, https://chembl.github.io/chembl-licensing/), so the section reuses its adapter, quota (one call per second) and terms. Live checks 2026-11-17 with curl: `target.json?target_components__accession=ACC&target_type=SINGLE PROTEIN` returns the protein's own target (POLQ O75417 CHEMBL6025, WRN Q14191 CHEMBL2146312, PARP1 P09874 CHEMBL3105); without the type filter ChEMBL also lists families and protein-protein interaction targets that contain the protein (PARP1 has 4 in all). `assay.json?target_chembl_id=ID&limit=1` and `activity.json?target_chembl_id=ID&limit=1` give the totals in `page_meta.total_count` (PARP1 705 assays, 10,162 activities; EGFR CHEMBL203 5,966 and 58,847). An isoform accession (P09874-2) and an accession with no target (Q8N726) return an empty page.
- Design: new `chembl` section on `get protein` (also in `all`), like the cell-line `chembl` section. The adapter (`crates/biomcp-adapters/src/chembl/target.rs`) reads the single-protein targets of the canonical accession (an isoform asks for its canonical accession), checks every row is a single-protein target with that component, then reads ChEMBL's own assay and activity totals per target with `limit=1`, checking the one row names the target. It reuses the cell-line paging check (`PageMeta`). The section gives `total` (targets) and per target `id`, `name`, `organism`, `assay_count`, `activity_count`; no target is an empty section. `docs/cautions.md` says the counts are ChEMBL's unfiltered totals and that complexes and families are not counted.
- j42227 (00d182f9): built and recorded; every fresh card with a target ran out of time (`deadline-exceeded`) after UniProt, because one protein takes three paced ChEMBL reads (a fresh target lookup took 2.7 s from the M5, each count about 1 s) inside the 20 s card budget. j42234 timed the calls. Like ClinVar's gene counts, the section now has its own ChEMBL policy `TARGET_COUNTS` (30 s per protein, 60 s command), and `get protein` takes that budget only when the chembl section is asked.
- j42236 (2b6a3ef9): live recording: `get protein O75417 P09874 chembl` complete (POLQ CHEMBL6025 46 assays / 693 activities; PARP1 CHEMBL3105 705 / 10,162), `O75417-2` reads CHEMBL6025, Q8N726 empty, `--disable-sources chembl` skipped; a fresh five-protein card (POLQ, PARP1, WRN, CHEK1, TTK) completed in 19 s; fresh `get protein P09874 all` 11.7 s, 20,718 bytes; `help get protein` 2,151 bytes. Protein, help, mcp, local and cell-line cases and schema-release-check passed; the structural ratchet wanted the declaration to name `super::target::TARGET_COUNTS`, fixed. Case `tests/cases/protein/chembl-targets.yaml` replays those replies (provenance in `tests/replies/protein/chembl-targets/provenance.json`); the EGFR `all` step in `native-sections.yaml` now replays EGFR's ChEMBL replies and asserts CHEMBL203's totals.
- Gate j42238 (e03baa79) failed only on clippy's 80-line limit for `protein::get::get`; the budget choice moved into a helper. Gate j42245 on ac9c9744 (`make --keep-going landing-gate docs-check schema-release-check`): passed. Since then main landed 1029 (STRING `interactions` protein section), which edits the same protein section list, `ProteinSections`, the protein-sections stub count, the `get protein` tool description and the EGFR `all` replay, so landing will need a merge.
- Merged origin/main after 1029 landed: both the STRING `interactions` and the `chembl` sections stay, in that order; the protein-sections stub now answers five sources. j42249 (bdabdf32): regenerated schemas and docs; check-cheap, types, schema-release and the help, mcp, local and cell-line cases passed; `tests/cases/protein/cards.yaml` step 1 measured 2,008 bytes against its 2,001 cap, because the card's `more` and `next` now name both new sections. Its cap is now 2,310, measured plus 15 percent (`sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md`).
- Gate j42252 on 6e51e79e (merged with origin/main at 1029; `make --keep-going landing-gate docs-check schema-release-check`): passed.
