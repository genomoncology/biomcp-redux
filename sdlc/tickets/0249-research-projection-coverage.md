# 0249 — Project phenotype results through search and fetch, and check every entity

Status: COMPLETE.
Landed: 72a3b5e0670233d2af76660dea86268661b2f6c4

Milestone: 4

Depends on: 0217

Depends on: 0250

## Outcome

The research `search` and `fetch` tools return phenotype results the way architecture 15.4 requires. Search rows carry the HPO label as their title, and fetch returns the complete phenotype card as research fetch text with its attribution URL, not an MCP tool error. The 0217 developer found that the generic projector has no title path for phenotype headers and no URL path for `result.term.attribution.url`, so search rows show bare IDs and fetch returns isError with the card.

- Starts from: the research projector in `crates/biomcp/src/mcp/research.rs` and its title and URL paths; 0217's retained witnesses initial__product-proof__00.json and 02.json.
- Changes: the research projector and its cases.
- Fix: add the phenotype title and URL paths. Then add one check that walks every registered search page and card schema and fails when the projector has no title or URL path for it, so the next entity cannot land with the same gap. Entities that 15.4 says to omit stay omitted by an explicit list.
- Proof: research search and fetch cases for an HPO term show the label title, the canonical URL and the full card text. The coverage check fails on a fake schema with no path and passes on the current registry. No source calls; replays only. Then make landing-gate runs through pm job.
- PM ruling 2026-10-26: this ticket adds every path over existing title and URL fields and the fail-closed registry check. The check passes once 0250 gives sources their canonical URL templates; until then 0249 does not land. Omission is allowed only for a shape whose source has no template, as architecture's research section allows. No known-gap list.
