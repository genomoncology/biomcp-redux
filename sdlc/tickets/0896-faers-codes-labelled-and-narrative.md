# 0896 — faers-codes-labelled-and-narrative

Status: OPEN.

Milestone: 7

## Outcome

A FAERS case card labels its seriousness flags and age unit as it already labels sex, qualification and reaction outcome; a code not on openFDA's list is marked as such, distinct from a field the source left out; and the case narrative openFDA carries is offered as a section.

## Why

In the pv-scientist persona run of 2026-11-13 (biomed-agent-bench `personas/pv-scientist/runs/2026-11-13/log.md` and `report.md`, analysis beside them; code-mode run in `runs/2026-11-13-raw/`): seriousness is six bare 1/2 codes and age unit stays 801, while sex and outcome carry labels (report Q7); the agent needed a curl to tell a source omission from an off-list code; openFDA carries `patient.summary.narrativeincludeclinical` but `available_sections` offered no narrative (report Q2). The code-mode agent had raw codes only and rebuilt the code lists from FDA's schema.

## Proof

- Proof: recorded cases where a fatal myocarditis card carries seriousness and age-unit labels from openFDA's published code lists, a recorded or synthetic off-list code is marked unmapped, and a case with a narrative returns it verbatim.

## Starts from

- Starts from: `crates/biomcp-adapters/src/faers/`, `crates/biomcp-types/src/adverse_event/`.

## Keeps

- Keeps: the raw code beside every label.

## Defers

- Defers: MedDRA hierarchy.

## Changes

- Changes: `crates/biomcp-adapters/src/faers/`, `crates/biomcp-types/`, `tests/cases/adverse_event/`, `docs/`
