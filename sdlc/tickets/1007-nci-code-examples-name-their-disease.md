# 1007 — nci-code-examples-name-their-disease

Status: OPEN.

Milestone: 7

## Outcome

Every help, schema and error text that gives an NCIt code as an example names its disease, such as `C3224 (melanoma)`.

## Why

The 2026-11-12 trial-ops run took C3224 from `query/trial.rs:51`'s example as pancreatic cancer; C3224 is melanoma. The bench's fact copied the error (bench `personas/trial-ops/runs/2026-11-16/analysis.md` 9.1).

## Proof

- Proof: a grep test that every `C[0-9]{4,}` example in help text has a disease name beside it; `make docs-check` passes.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

## Progress

- Codes and names, from the repo's own files: C3224 is melanoma (`sdlc/planning/slices/disease.md:30`, MONDO:0005105 carries NCIt C3224; `sdlc/planning/slices/trial.md:203`). C96022 is NCI's "CD274 Gene", found by the synonym PD-L1 (`tests/cases/biomarker/code.yaml`, `tests/cases/biomarker/synonym.yaml`). No other NCIt code appears as an example in help, schema, docs or error text; `MedGen:C…` codes are not NCIt.
- Changed text: `search trial --condition` ("such as C3224 (melanoma)"), `--condition-id` (now gives "NCIT:C3224 (melanoma)" before its generated example), `--biomarker-code` and `get biomarker` ("C96022 (CD274 Gene, PD-L1)"), `search cell-line --disease` ("Melanoma or C3224 (melanoma)"), the NCI disease refusal ("NCIt identifier such as C3224 (melanoma)"), `docs/cautions.md`, one release-notes line under Trials. Schemas and `docs/reference/commands.md` regenerated (job j41968, applied unchanged).
- Proof: `scripts/check_docs.py` `ncit_examples` (runs in `check-cheap` and `docs-check`) fails any line of docs, skills, README, release notes, checked-in schemas or crate source that holds an NCIt code without "CODE (name" beside it, and any code missing from its name table. `scripts/test_docs.py` shows a bare C3224 fails, NCIT:C3224 (melanoma) beside a MedGen code passes, and an unlisted code fails.
- Schema: the refusal's text is a value of the `biomcp.error/4` message enum, and C3224's old text is in the 1.0.0 baseline, so the reword removes a baseline value. Job j41966 failed generation and j41968 failed `schema-release-check` at `/error/message` until `release/schema-reviews.json` gained two reviewed enum transitions (checked-in to new, baseline to new), as 0588 and 0846 did for their messages. The baseline-to-new entry lists every current message, so a later ticket that adds a message must extend it. This needs the PM's yes under the release-candidate rule.
