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

## Progress

- 2026-11-13, measured live before design (jobs j39363, j39370; openFDA dataset 2026-10-28, 21,054,159 reports). Stopped before building: two of the three parts rest on premises the source does not support.
- Narrative: openFDA's published field reference (`https://open.fda.gov/fields/drugevent.yaml`) describes `patient.summary.narrativeincludeclinical` as "Populated with Case Event Date, when available; does `NOT` include Case Narrative." 7,834,564 reports carry the field; all 7,834,564 match the phrase "CASE EVENT DATE" and none fall outside it. Case 27298481 holds `CASE EVENT DATE: 20260810`. Offering it as a `narrative` section would present a date as a case narrative.
- Seriousness flags: openFDA publishes no code list for the six `seriousness*` fields (`possible_values` is empty). Each description says the value "is `1` if the adverse event resulted in [death / ...], and absent otherwise", yet the data carries `2` on most reports (death: `2` 6,303,253, `1` 1,906,622) and no value outside 1 and 2. FDA's GitHub schema gives only `{"type": "string", "format": "int32"}`. Under the ticket's rule, `1` could take the publisher's description clause and every `2` would be marked off-list on nearly every card.
- Age unit: the published list holds (`800` Decade, `801` Year, `802` Month, `803` Week, `804` Day, `805` Hour); one report carries a unit outside it. `serious` also has a published list (`1`/`2`). Sex, qualification, reaction outcome and action codes seen live all sit on BioMCP's lists; drug role also carries `4` (791 reports) and `5` (1), which BioMCP's role list lacks (published list not rechecked).
- Waiting on a PM ruling on scope before any code lands.
