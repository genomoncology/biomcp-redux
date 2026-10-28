# 0410 — Search NCI biomarkers

Status: OPEN.

Milestone: 5

## Outcome

`biomcp search biomarker --source nci` finds NCI trial biomarkers by name or synonym, and `biomcp get biomarker <NCIt code>` returns one. Each header gives the NCIt code, name, native type and the source's trial count; the card adds synonyms, parent and ancestor codes and whether trials use the biomarker for inclusion, exclusion or both, exactly as NCI's `eligibility_criterion` states.

- Why: `decisions/2026-10-28-benchmark-gaps-join-1-0.md`. MCP-Bench asks about biomarker criteria such as PD-L1, and 0.4.6 offered `nci_biomarker_searcher`. A 2026-10-28 check for "PD-L1" returned CD274 Gene, C96022, with synonyms including PD-L1.
- Starts from: the NCI adapter, NCI CTS API v2 `/biomarkers` and its fields (`codes`, `name`, `synonyms`, `type`, `parent_ids`, `ancestor_ids`, `eligibility_criterion`, `count`).
- Rules: keys are read only when the source is called and never printed or recorded. Bounded live captures within the source's published limits; no licensed content in fixtures. Follow the entity, header, card, section and paging rules in `sdlc/planning/architecture.md`, the shared bad-value helper and typed IDs. New schemas get IDs with hyphens and enter the 1.0 baseline. Help, examples, the generated command reference, the upgrade table and MCP tool text name the new surface. No text heuristics. NCI's overview says public information may exclude some biomarker data, so the first capture measures completeness for three biomarkers a clinical reviewer chooses, and help states what the source covers. The name `biomarker` must not collide with the existing `search diagnostic --biomarker` option's meaning; help separates them.
- Proof: replay cases for a synonym search, a get by code, paging, a missing key refusal and a bad value with a problem; bounded keyed captures.
- Changes: a new biomarker entity, the NCI adapter, schemas, help and cases.
- Defers: biomarker interpretation, which BioMCP never does.
