# 0409 — Find NCI trials at a chosen organization

Status: OPEN.

Milestone: 5

Depends on: 0408

## Outcome

`biomcp search trial --source nci --organization <name or ID>` returns NCI trials with a site at that organization, using NCI's own site filter, and each trial header says which listed site matched. An organization header names this search as its next step.

- Why: `decisions/2026-10-28-benchmark-gaps-join-1-0.md`. It lets an agent go from a named cancer center to its trials, which the Boston step of MCP-Bench needs.
- Starts from: 0408, the NCI trial search adapter and NCI's documented site filter fields. A guessed field name returned zero trials on 2026-10-28, so the reviewer confirms the exact field from NCI's documentation or one capture before building.
- Rules: keys are read only when the source is called and never printed or recorded. Bounded live captures within the source's published limits; no licensed content in fixtures. Follow the entity, header, card, section and paging rules in `sdlc/planning/architecture.md`, the shared bad-value helper and typed IDs. New schemas get IDs with hyphens and enter the 1.0 baseline. Help, examples, the generated command reference, the upgrade table and MCP tool text name the new surface. No text heuristics. The existing ClinicalTrials.gov `--facility` filter and its refusal with a radius stay unchanged.
- Proof: replay cases for a matching organization, an organization with no trials and the combined refusal rules; one bounded keyed capture.
- Changes: the NCI trial filter, the organization next step, help and cases.
- Defers: nothing.
