# 0413 — Filter MAUDE device reports by manufacturer and product code

Status: OPEN.

Milestone: 5

## Outcome

`biomcp search adverse-event --source maude` accepts `--manufacturer <name>` and `--product-code <code>`, mapped exactly to openFDA's device manufacturer and product code fields, alongside the existing device phrase. A recall from 0412 names this search for its product code as a next step.

- Why: `decisions/2026-10-28-benchmark-gaps-join-1-0.md`. 0.4.6 filtered device events by manufacturer, and a product code joins recalls, clearances and problem reports for one device type.
- Starts from: the MAUDE adapter, openFDA's device event searchable fields and 0412.
- Rules: keys are read only when the source is called and never printed or recorded. Bounded live captures within the source's published limits; no licensed content in fixtures. Follow the entity, header, card, section and paging rules in `sdlc/planning/architecture.md`, the shared bad-value helper and typed IDs. New schemas get IDs with hyphens and enter the 1.0 baseline. Help, examples, the generated command reference, the upgrade table and MCP tool text name the new surface. No text heuristics. Counts are reports, not people or device failures.
- Proof: replay cases for each filter, both together and an unknown code; bounded captures.
- Changes: the MAUDE filter grammar, adapter, help and cases.
- Defers: nothing.
