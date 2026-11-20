# 1115 — name-the-source-and-section-limits-in-disease-get-refusals

Status: OPEN.

Milestone: 7

## Outcome

A disease get refusal gives the valid sources for the requested section, names a missing source or section directly, and suggests a command with a valid combination.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get disease ORPHA:115 xrefs --source orphanet` returned `"valid_values":["ols","mydisease","monarch","seer","orphadata"]`. `biomcp get disease ORPHA:115 xrefs --source orphadata` rejects the offered `orphadata` and lists only `mydisease`. `biomcp get disease ORPHA:115 --source orphadata` says `required option missing: add error.option with a value valid_values accepts`. `biomcp get disease MONDO:0019391 genes` says `value not accepted: give error.option a value valid_values accepts` despite accepting an explicit `--source monarch`. `biomcp get disease MONDO:0019391 gene-validity --source clingen` returns `unknown section: error.option is not a section of this record`.

## Proof

- Proof: Replay these calls and confirm that `xrefs` never offers a source it rejects, that required `prevalence` and `--source monarch` are named where applicable, and that the unknown section is identified explicitly.

## Starts from

- Starts from: Disease get source and section checks, with ticket 1062’s valid follow-up rule.

## Keeps

- Keeps: ORPHA identity lookup and each valid explicit source continue to work.

## Defers

- Defers: Orphanet mapping relations and disease-first gene curations have separate tickets.

## Changes

- Changes: Disease get validation, guidance and cases.
