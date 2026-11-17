# 1021 — drug-xrefs-carry-atc-codes

Status: OPEN.

Milestone: 7

## Outcome

`get drug X xrefs` gives the drug's ATC codes.

## Why

pt-committee 9.3 (bench `personas/pt-committee/runs/2026-11-16/analysis.md`); MyChem's card fields omit `chembl.atc_classifications`.

## Proof

- Proof: a replayed case for apixaban asserting B01AF02; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

## Progress

- Checked live 2026-11-16: MyChem's ChEMBL component carries `atc_classifications` (apixaban: B01AF02, a single string; ChEMBL's molecule API gives `['B01AF02']`). The card call BioMCP already makes for `get drug X xrefs` simply did not ask for the field, so no new call or source is needed.
- Changed: the MyChem card fields ask for `chembl.atc_classifications`; each code becomes an xrefs row with namespace `atc`, relation `chembl.atc_classifications`, attributed to the ChEMBL record. The xrefs section help says MyChem rows include ChEMBL ATC codes. One release-notes line. `--source chembl` xrefs are unchanged.
- Recorded in pm job j42093: `get drug apixaban xrefs` resolves to QNZCBYKSOIHPEH-UHFFFAOYSA-N (72 rows, ATC third). Both MyChem replies kept unchanged in `tests/replies/drug/xrefs-atc/`.
- Proof: `tests/cases/drug/xrefs-atc.yaml` (CLI and MCP) expects B01AF02 with namespace `atc` and record CHEMBL231779.
