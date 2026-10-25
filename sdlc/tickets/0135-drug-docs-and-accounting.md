# 0135 — Document the drug commands and close the drug accounting

Status: COMPLETE.
Landed: 69ad2092692ec4356caa2d7aeec3497d43899f6d

Milestone: 3

Depends on: 0133

Depends on: 0134

## Outcome

Generated help and the MCP tool description cover `search drug`, `get drug` with its sections and options, and `biomcp sync ema` and `biomcp sync ddinter`, with examples taken from case run steps. The 0.9 parity ledger has a row for every drug command, option and section in drug.md's accounting, naming the 1.0 spelling, the owning slice or the retirement. Every drug case asserts a measured `max_bytes` from its CLI and MCP output under the existing headroom rule, and drug.md's budget table gains the measured sizes. The landing commit ticks the drug roadmap boxes this work completes.

- Why: the slice delivers docs, cases with ceilings and full 0.9 accounting, as `planning/slices.md` says.
- Starts from: 0034's gene docs ticket as the pattern; drug.md section 4's case list, budget tables and 0.9 accounting; the generated help and MCP description code; ROADMAP.md's drug block.
- Rules: O01 to O04 are recorded as owned by the trial, adverse event, disease, pathway and shared batch owners, with no drug case added for them. A measured size over its design allocation is fixed or raised to the PM before landing. The token budget box ticks when every row in the design's token table has the measured 1.0 size and either 0136's qualified 0.9 figure or a stated reason that 0.9 could not be measured for that question. WHO, FDA orphan, DGIdb, PharmacoDB and CVX move to the dropped line with drug.md's reasons. `identity_level` comes from the endpoint or record type, never from a field's shape.
- Proof: a help case asserts each drug command lists its sections, options and two parsed examples; an MCP `tools/list` case asserts the drug tree within the existing description ceiling. A reviewer checks that every accounting row has a ledger row, every design case ID has a case file or an outside-repo case, and every case has a measured `max_bytes`. The landing commit ticks Biodata types and schema, Search, Get and sections, Outside-in cases, MCP parity, Docs and 0.9 accounting, and the kept source boxes, each only after its check passes. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
- Defers: full 1.0 against 0.9 parity, which 0098 measures after the adapters land.
- PM ruling 2026-10-25: the ChEMBL search row needs the default page, because the default is what agents see. One bounded keyless ChEMBL request captures the default 10-row search, becomes a replay case with a ceiling, and fills the row. The captured 3-row page stays as a labeled extra row.
- PM ruling 2026-10-25: the DDInter interactions section defaults to 5 rows, keeping every field, the order and the page step, with explicit limits unchanged, because 10 rows measured 8,763 bytes against the 6,144 allocation. Both known stored IDs are measured at the new default.
