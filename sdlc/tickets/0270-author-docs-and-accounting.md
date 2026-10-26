# 0270 — Document the author commands and close the author accounting

Status: COMPLETE.
Landed: 588f4daa023c2501bc0e9bf6fbf19e751cb5c503

Milestone: 4

Depends on: 0269

## Outcome

Generated help and the MCP tool description cover `search author`, `get author` and `author papers`, with both sources, the qualified ID forms and their aliases, the `details` section and `--full`, and paging options, with examples taken from case run steps. Help states that an author ID identifies one provider record and not a person, that no ranking or merge is computed, that both sources are `noncommercial` and how commercial users switch them off, and how to set each optional key with `biomcp key set` or its 0.9 variable. The 0.9 parity ledger has a row for every author command, option and section in author.md's accounting, naming the 1.0 spelling, the deferral or the retirement. Every author case asserts a measured `max_bytes` from its CLI and MCP output under the existing headroom rule, and author.md's measurement table replaces each unmeasured row with the measured envelope. Each question A01 to A24 keeps its answered, partial or unanswered status. The landing commit gives ROADMAP.md's author block the checklist that the cell line block carries and ticks the boxes this work completes.

- Why: The slice delivers docs, cases with ceilings and full 0.9 accounting, as `planning/slices.md` says.
- Starts from: 0262 and 0244 as the pattern; author.md's case list, measurement table, question bank and legacy accounting, with the author rulings and the new-sources decision; the generated help and MCP description code; ROADMAP.md's author block.
- Rules: topic-wide author ranking and cross-provider merging stay recorded as deferred with the ruling's reason. The ORCID public data file is not a planned route, because the new-sources decision replaces it. 0.9 timings and output sizes stay marked unmeasured, and the milestone 5 comparison measures them. A measured size over its starting ceiling is fixed or raised to the PM before landing. An entity help or MCP description cap increase needs a PM ruling.
- Proof: a help case asserts each author command lists its sources, sections, options and two parsed examples; an MCP `tools/list` case asserts the author tree within the existing description ceiling. A reviewer checks that every accounting row has a ledger row, every design case ID has a case file or an outside-repo case, and every case has a measured `max_bytes`. The landing commit ticks Biodata types and schema, Search, Get and sections, Outside-in cases, Token budget, MCP parity, Docs and 0.9 accounting, and the Semantic Scholar and ORCID source boxes, each only after its check passes. This ticket adds no filter or exact-ID get and makes no provider call. No licensed content enters the repo. Then make landing-gate runs through pm job.
- Defers: the contained 0.9 author comparison, which the milestone 5 comparison measures.
- Changes: generated help and MCP description sources, the parity ledger, cases, author.md and ROADMAP.md.
- PM ruling 2026-10-26 (m8063): the `biomcp` MCP description cap rises from 3,563 to 4,416 bytes, the 3,840 measured in j15878 plus 15 percent, with no content cut. Author help caps from j15876, measured plus 15 percent, CLI, MCP and Markdown: `get author` 1,405, 1,405 and 1,486; `author papers` 2,322, 2,322 and 2,165; the author aggregate 5,446, 5,445 and 5,305; `search author` 1,954, 1,954 and 1,839 under the standing rule. The `tools/list` and whole help caps stay unchanged.
