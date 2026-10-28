# 0412 — Search FDA device recalls

Status: OPEN.

Milestone: 5

## Outcome

`biomcp search diagnostic --source fda-device-recall` finds FDA medical device recalls by firm, product code, product text and date range, and `get diagnostic` returns one recall by its native identity. Each header gives the recall identity, firm, product code, classification when the enforcement report supplies it, status and initiation date; the card adds the reason, root cause, product description, distribution pattern and linked 510(k) or PMA numbers as published.

- Why: `decisions/2026-10-28-benchmark-gaps-join-1-0.md`. MCP-Bench asks about problems with genomic test kits. The 0162 ruling that dropped FDA recalls is replaced.
- Starts from: openFDA `device/recall.json` and `device/enforcement.json` (a 2026-10-28 check for recalling firm Illumina returned 10 records from each), the existing openFDA device adapters (MAUDE, 510(k), PMA) and their key handling. The reviewer decides whether recall and enforcement are one source joined by a published identity or two sources, and whether recalls belong under `diagnostic` or another existing entity.
- Rules: keys are read only when the source is called and never printed or recorded. Bounded live captures within the source's published limits; no licensed content in fixtures. Follow the entity, header, card, section and paging rules in `sdlc/planning/architecture.md`, the shared bad-value helper and typed IDs. New schemas get IDs with hyphens and enter the 1.0 baseline. Help, examples, the generated command reference, the upgrade table and MCP tool text name the new surface. No text heuristics. Counts are records, never products harmed; no causal or safety-rate wording.
- Proof: replay cases for firm, product code and date searches, a get, paging and a bad value with a problem; bounded captures.
- Changes: the adapter, source registration, outbound host policy row if needed, schemas, help and cases.
- Defers: drug and food recalls.
