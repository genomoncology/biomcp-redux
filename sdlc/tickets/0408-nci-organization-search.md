# 0408 — Search NCI organizations

Status: OPEN.

Milestone: 5

## Outcome

`biomcp search organization --source nci` finds NCI clinical trial organizations by name, city, state or province, country and type, and `biomcp get organization` returns one. Each header gives the native NCI organization identity, name, city, state, country and the source's trial count; the card adds address, coordinates, organization family and public contact fields as published.

- Why: `decisions/2026-10-28-benchmark-gaps-join-1-0.md`. MCP-Bench asks whether trial sites match Boston's large cancer centers, and 0.4.6 offered `nci_organization_searcher`.
- Starts from: the NCI adapter and its key handling, NCI CTS API v2 `/organizations` (a 2026-10-28 check for "Dana-Farber" returned name, address, `org_coordinates`, `org_family`, contact fields and `count`), and 0.4.6's tool arguments.
- Rules: keys are read only when the source is called and never printed or recorded. Bounded live captures within the source's published limits; no licensed content in fixtures. Follow the entity, header, card, section and paging rules in `sdlc/planning/architecture.md`, the shared bad-value helper and typed IDs. New schemas get IDs with hyphens and enter the 1.0 baseline. Help, examples, the generated command reference, the upgrade table and MCP tool text name the new surface. No text heuristics. Reviewer checks which native field is a stable organization ID before `get` is designed; if none exists, `get` is dropped and the ticket says so.
- Proof: replay cases for name and city searches, paging, a missing key refusal and a bad value with a problem; one bounded keyed capture.
- Changes: a new organization entity, the NCI adapter, schemas, help and cases.
- Defers: organizations from other registries.
