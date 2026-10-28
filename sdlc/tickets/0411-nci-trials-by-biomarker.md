# 0411 — Find NCI trials that include or exclude a biomarker

Status: OPEN.

Milestone: 5

Depends on: 0410

## Outcome

`biomcp search trial --source nci --biomarker-code <NCIt code>` returns NCI trials that list that biomarker, and `--biomarker-use inclusion` or `exclusion` narrows to that native eligibility use. Each trial header says which use matched. A biomarker header names this search as its next step.

- Why: `decisions/2026-10-28-benchmark-gaps-join-1-0.md`. It answers "which trials require PD-L1?" from NCI's structured data instead of eligibility text.
- Starts from: 0410, the NCI trial search adapter and NCI's trial biomarker fields.
- Rules: keys are read only when the source is called and never printed or recorded. Bounded live captures within the source's published limits; no licensed content in fixtures. Follow the entity, header, card, section and paging rules in `sdlc/planning/architecture.md`, the shared bad-value helper and typed IDs. New schemas get IDs with hyphens and enter the 1.0 baseline. Help, examples, the generated command reference, the upgrade table and MCP tool text name the new surface. No text heuristics. A clinical reviewer approves the help wording for inclusion and exclusion, which must say these are NCI's curated listings, not a patient eligibility decision. The existing literal `--variant` text search stays separate.
- Proof: replay cases for inclusion, exclusion and an unknown code; one bounded keyed capture per use.
- Changes: the NCI trial filter, the biomarker next step, help and cases.
- Defers: nothing.
