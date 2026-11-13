# 0896 — faers-codes-labelled-and-narrative

Status: OPEN.

Milestone: 7

## Outcome

Narrowed by PM ruling, 2026-11-13 (option 1 after the live check in Progress). A FAERS card labels its age unit from openFDA's published list (800 to 805), as it already labels sex, qualification and reaction outcome. Each seriousness flag `1` carries openFDA's own wording; `2`, which openFDA does not document ("absent otherwise") but the data carries, stays unlabelled, and the docs and field description say so. One marker, `unmapped_codes`, names any present code that is off openFDA's published list for its field, distinct from a field the source omitted; it covers fields with a published list (sex, age unit, qualification, drug role, action, reaction outcome), not seriousness `2`. The card shows `patient.summary.narrativeincludeclinical` unchanged as `case_event_date`; there is no FAERS narrative section, and help says openFDA's public FAERS data carries no case narrative.

## Why

In the pv-scientist persona run of 2026-11-13 (biomed-agent-bench `personas/pv-scientist/runs/2026-11-13/log.md` and `report.md`, analysis beside them; code-mode run in `runs/2026-11-13-raw/`): seriousness is six bare 1/2 codes and age unit stays 801, while sex and outcome carry labels (report Q7); the agent needed a curl to tell a source omission from an off-list code; openFDA carries `patient.summary.narrativeincludeclinical` but `available_sections` offered no narrative (report Q2). The code-mode agent had raw codes only and rebuilt the code lists from FDA's schema.

## Proof

- Proof: a recorded fatal myocarditis card (27298481) carries `death_label` for `1`, no label for `2`, `age_unit_label` "Year" beside `801`, and `case_event_date` verbatim; a recorded report (22814945) with drug role `5`, off openFDA's 1 to 3 list, names it in `unmapped_codes`; a synthetic report shows an off-list age unit, reaction outcome and drug role named in `unmapped_codes`.

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
- Age unit: the published list holds (`800` Decade, `801` Year, `802` Month, `803` Week, `804` Day, `805` Hour); one report carries a unit outside it. `serious` also has a published list (`1`/`2`). Sex, qualification, reaction outcome and action codes seen live all sit on the published lists. Drug role also carries `4` (791 reports) and `5` (1, report 22814945); openFDA's published role list holds only 1 to 3 (j39377), so both are off-list. The one report with an off-list age unit (`806`) is 7166919-6, whose ID is not all digits, so BioMCP cannot fetch it; the off-list age unit proof is synthetic.
- PM ruling 2026-11-13: option 1, the narrowed Outcome above.
- Built: `age_unit_label` from openFDA's 800 to 805 list; `seriousness.*_label` for `1` from each flag's own description clause, `2` unlabelled; `unmapped_codes` (field and code) on the card, drug rows and reaction rows for sex, age unit, qualification, drug role, action and reaction outcome; `case_event_date` on the card, verbatim. An off-list age unit still drops the age with its existing problem, and now also shows its code in `unmapped_codes`. Help and `docs/cautions.md` say openFDA's public FAERS data carries no case narrative and openFDA documents only seriousness `1`.
- Cases: `tests/cases/adverse-event/faers-recorded-labels.yaml` replays live replies for 27298481 and 22814945 recorded in j39377 and j39380; `faers-code-words.yaml` adds a synthetic off-list age unit and markers for its off-list outcome and role. Schema change is optional fields and descriptions only.
- The synthetic FAERS card budget is remeasured at 1,647 full and 1,525 comparable bytes (was 1,442 and 1,320; j39435): the card now carries two seriousness labels and the age-unit label.
- Gate passed on bb0c4d074 (`make --keep-going landing-gate docs-check schema-release-check`), job j39465.
- Review fix: `age_unit_label` now comes from the age that survived, so a report with a unit but no age (484 live) shows neither the unit nor its label; synthetic report 101 in `faers-code-words.yaml` covers it. `docs/cautions.md` notes that ICH E2B(R3) defines role 4 as "drug not administered".
