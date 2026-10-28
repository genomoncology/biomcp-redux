# 0414 — Join several trial conditions or interventions with OR

Status: OPEN.

Milestone: 5

## Outcome

`search trial` accepts `--condition` and `--intervention` more than once, and joins repeated values of one option with OR in each source's own query syntax, while different options still combine with AND. The reply's query echo lists every value.

- Why: `decisions/2026-10-28-benchmark-gaps-join-1-0.md`. 0.4.6 accepted lists for these arguments, and agents trained on it pass several values.
- Starts from: the ClinicalTrials.gov and NCI trial adapters, ClinicalTrials.gov's documented search expression syntax and NCI's documented OR behavior for repeated values.
- Rules: keys are read only when the source is called and never printed or recorded. Bounded live captures within the source's published limits; no licensed content in fixtures. Follow the entity, header, card, section and paging rules in `sdlc/planning/architecture.md`, the shared bad-value helper and typed IDs. New schemas get IDs with hyphens and enter the 1.0 baseline. Help, examples, the generated command reference, the upgrade table and MCP tool text name the new surface. No text heuristics. Each value is escaped as a literal inside the OR expression, so no value can inject query syntax. A cap on repeated values is set from the source's limits and stated in help.
- Proof: replay cases for two conditions, two interventions, a value containing OR or quotes, and the cap; bounded captures.
- Changes: the trial grammar, both adapters, help and cases.
- Defers: OR across different options.
