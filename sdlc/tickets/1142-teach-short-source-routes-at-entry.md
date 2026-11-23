# 1142 — teach-short-source-routes-at-entry

Status: COMPLETE.
Landed: 0905b709e27d5dbde22e221e1ec265179d8c78d3

Milestone: 7

## Outcome

Each entry point an agent actually reads (CLI help, the shipped skill, the MCP server instructions and tool descriptions) shows the short working call for the reads agents got wrong or went around, and says where the evidence stops.

## Why

In the 2026-11-22 comparison, agents with a shell made 5 to 9 direct publisher requests a run (DailyMed, Drugs@FDA, openFDA, ClinicalTrials.gov, GenCC, ClinGen), some after a fixable refusal. Connector-only agents never see the skill; they get only the server instructions and tool descriptions. The skill's worked examples suggest a high label page (`--limit 50`), describe `get` ID lists too broadly, place `article text` next to get sections, omit mechanism and regimen-partner paths, and tell a stale GenCC user to sync, which a hosted connector cannot do. The typed front map (`crates/biomcp/src/mcp/typed/catalog.rs:340-362`) gives no disease-first ClinGen example, and the full-mode instruction about `tools/list` metadata is setup advice the model cannot act on. 95 replies were cut by the client limit, mostly broad label searches, trial searches, flow and location reads, and Purple Book presentations.

## Changes

- Replace the misleading examples in `skills/SKILL.md` and command help in place, within the skill's size budget.
- Show a narrow page with its follow-up for broad label, trial flow and location, and presentation reads.
- Add a disease-first curation example and the mechanism route to the MCP front map; drop the startup-only instruction from what the model sees.
- Where a response cannot cover something (PMA summaries, holder labels), say so in that response, with a permitted read when one exists.

## Proof

- Replay neutral examples (mechanism query with a projected small page; CDx application with prefixed ID; disease-first curation; both article text forms) through CLI help, the rendered skill, typed MCP guidance and compact MCP guidance; each example runs; initial bytes measured before and after.

## Defers

- New sources or archive coverage (Ian's decision).
