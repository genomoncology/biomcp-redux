# 0177 — Document the diagnostic commands and close the diagnostic accounting

Status: COMPLETE.
Landed: 946ef9d04012feac5e8b619edd42d3fe602635b0

Milestone: 4

Depends on: 0176
Depends on: 0181
Depends on: 0179

## Outcome

Generated help and the MCP tool description cover `search diagnostic` for `gtr` and `fda-cdx` with their selectors, `get diagnostic` for `gtr`, `who-ivd` and `fda-pma` with their sections and `--supplement`, `get diagnostic` for `fda-510k` by exact K number, and `biomcp sync gtr`, `biomcp sync who-ivd` and `biomcp sync fda-cdx`, with examples taken from case run steps. Help states that a listing is not authorization and a companion row is not a treatment recommendation. The 0.9 parity ledger has a row for every diagnostic command, option and section in diagnostic.md's accounting, naming the 1.0 spelling, the deferral or the retirement. Every diagnostic case asserts a measured `max_bytes` from its CLI and MCP output under the existing headroom rule, and diagnostic.md's budget table replaces each provisional source ceiling with the measured envelope. The landing commit gives ROADMAP.md's diagnostic block the checklist that the drug and disease blocks carry, replaces the openFDA devices box with FDA companion diagnostic tables, openFDA PMA and openFDA 510(k) boxes, and ticks the boxes this work completes.

- Why: the slice delivers docs, cases with ceilings and full 0.9 accounting, as `planning/slices.md` says.
- Starts from: 0135 and 0145 as the pattern; diagnostic.md's case map, progressive disclosure table and legacy surface accounting, with the 0154 recheck amendment 4; the generated help and MCP description code; ROADMAP.md's diagnostic block.
- Rules: free-text 510(k) search, WHO discovery and selectors, the GTR laboratory selector and free-text PMA search stay recorded as deferred with their reason. The name-derived FDA overlay, disease synonym expansion, the generic source merge and read-triggered refresh move to the dropped line with the design's reasons. A measured size over its provisional ceiling is fixed or raised to the PM before landing. The token budget box ticks when every row has its measured 1.0 size and either a qualified 0.9 figure from 0098 or a stated reason that 0.9 could not be measured.
- Proof: a help case asserts each diagnostic command lists its sources, sections, options and two parsed examples; an MCP `tools/list` case asserts the diagnostic tree within the existing description ceiling. A reviewer checks that every accounting row has a ledger row, every design case ID has a case file or an outside-repo case, and every case has a measured `max_bytes`. The landing commit ticks Biodata types and schema, Search, Get and sections, Outside-in cases, MCP parity, Docs and 0.9 accounting, and the GTR, WHO, FDA companion table and PMA source boxes, each only after its check passes. This ticket adds no filter or exact-ID get and makes no provider call; the diagnostic filters and exact-ID gets keep their known-positive captures from one bounded keyless pm job each. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
- Defers: the contained 0.9 diagnostic comparison, which 0098 measures after the adapters land with verified containment.
- PM ruling 2026-10-25: get diagnostic help ceilings approved at measured +15%: 2558 stdout and 2557 compact bytes (measured 2224/2223). Generated help now shows five section descriptions. Search help, whole help and the MCP description keep their existing caps.
