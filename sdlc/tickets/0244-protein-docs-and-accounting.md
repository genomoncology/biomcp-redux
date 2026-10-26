# 0244 — Document the protein commands and close the protein accounting

Status: OPEN.

Milestone: 4

Depends on: 0243

## Outcome

Generated help and the MCP tool description cover `search protein`, `get protein` and the relation commands `features`, `domains`, `isoforms`, `complexes`, `complex-members`, `structures` and `sequence`, with their sections, `--position`, `--type`, `--complex`, scope and paging options, and examples taken from case run steps. Help states that no variant effect, structure, interaction or clinical meaning is inferred and no bioinformatics tool runs. The 0.9 parity ledger has a row for every protein command, option and section in protein.md's accounting, naming the 1.0 spelling, the deferral or the retirement, and each closed public issue in that accounting keeps its stated disposition. Every protein case asserts a measured `max_bytes` from its CLI and MCP output under the existing headroom rule, and protein.md's budget table replaces each offline projection with the measured envelope. Each question P01 to P28 keeps its answered, partial or unanswered status, updated only where a capture now answers it. The landing commit gives ROADMAP.md's protein block the checklist that the drug and disease blocks carry, moves STRING to a deferred line with its reason, and ticks the boxes this work completes.

- Why: the slice delivers docs, cases with ceilings and full 0.9 accounting, as `planning/slices.md` says.
- Starts from: 0169 and 0177 as the pattern; protein.md's case table, measured budget section, question table, 0.9 accounting and closed public issue accounting, with the 2026-10-26 protein rulings; the generated help and MCP description code; ROADMAP.md's protein block.
- Rules: STRING, `protein associations`, `--disease`, `--existence` and any unproven `--include-unreviewed` stay recorded as deferred with their reason. `--all-species` moves to the dropped line. Bulk human proteome, InterPro match, ComplexTab and whole-human STRING bundles stay recorded as deferred because no harvested question needs them. 0.9 timings and output sizes stay marked unmeasured, and the milestone 5 comparison measures them. A measured size over its starting ceiling is fixed or raised to the PM before landing. The token budget box ticks when every row has its measured 1.0 size and a stated reason that 0.9 is unmeasured.
- Proof: a help case asserts each protein command lists its sections, options and two parsed examples; an MCP `tools/list` case asserts the protein tree within the existing description ceiling. A reviewer checks that every accounting row has a ledger row, every design case ID has a case file or an outside-repo case, and every case has a measured `max_bytes`. The landing commit ticks Biodata types and schema, Search, Get and sections, Outside-in cases, Token budget, MCP parity, Docs and 0.9 accounting, and the UniProt, InterPro and Complex Portal source boxes, each only after its check passes. This ticket adds no filter or exact-ID get and makes no provider call; the protein filters and exact-ID gets keep their known-positive captures from one bounded keyless pm job each. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
- Defers: the contained 0.9 protein comparison, which the milestone 5 comparison measures.
