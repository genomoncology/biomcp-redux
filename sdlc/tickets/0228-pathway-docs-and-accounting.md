# 0228 — Document the pathway commands and close the pathway accounting

Status: OPEN.

Milestone: 4

Depends on: 0224
Depends on: 0226

## Outcome

Generated help and the MCP tool description cover `search pathway`, `get pathway`, `pathway genes`, `pathway events` and `sync wikipathways` with their sources, selectors, sections and paging options, with examples taken from case run steps. Help states that membership comes from a named source and release and makes no claim of therapy, response or actionability, and that no enrichment is calculated. The 0.9 parity ledger has a row for every pathway command, option and section in pathway.md's accounting, naming the 1.0 spelling, the deferral or the retirement. Every pathway case asserts a measured `max_bytes` from its CLI and MCP output under the existing headroom rule, and pathway.md's disclosure cost table replaces each proposed allocation with the measured envelope. Each question 1 to 26 keeps its answered status, updated only where a case now answers it. The landing commit gives ROADMAP.md's pathway block the checklist that the pharmacogenomics block carries and ticks the boxes this work completes.

- Why: the slice delivers docs, cases with ceilings and full 0.9 accounting, as `planning/slices.md` says.
- Starts from: 0169 and 0177 as the pattern; pathway.md's case matrix, measured disclosure costs, question bank and 0.9 accounting; the generated help and MCP description code; ROADMAP.md's pathway block.
- Rules: KEGG, g:Profiler, `enrich` and `enrichment` stay on the dropped line with the entity boundary ruling as the reason. `pathway drugs`, `pathway articles` and `pathway trials` move to the dropped line with the 2026-10-26 ruling's reason. The `search all` pathway section stays recorded as accounted with no new parent search. The full GPML archive and a bundle-wide gene index stay recorded as deferred. A measured size over its allocation is fixed or raised to the PM before landing. An entity help or MCP description cap increase needs a PM ruling, while the source name list and whole-tree help move to measured size plus 15 percent under the standing rule. The token budget box ticks when every row has its measured 1.0 size and the 0.9 figure stays marked unmeasured, as the 2026-10-26 rulings say, for the milestone 5 comparison.
- Proof: a help case asserts each pathway command lists its sources, selectors, sections, options and two parsed examples; an MCP `tools/list` case asserts the pathway tree within the existing description ceiling. A reviewer checks that every accounting row has a ledger row, every design case has a case file or an outside-repo case, and every case has a measured `max_bytes`. The landing commit ticks Slice complete, Biodata types and schema, Search, Get and sections, Outside-in cases, Token budget, MCP parity, Docs and 0.9 accounting, and the Reactome and WikiPathways source boxes, each only after its check passes. This ticket adds no filter or exact-ID get and makes no provider call; the pathway filters and exact-ID gets keep their known-positive captures from one bounded keyless pm job each. Then make landing-gate runs through pm job.
- Defers: the contained 0.9 pathway comparison, which the milestone 5 comparison measures.
