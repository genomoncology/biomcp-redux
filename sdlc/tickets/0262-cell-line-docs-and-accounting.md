# 0262 — Document the cell line commands and close the cell line accounting

Status: OPEN.

Milestone: 4

Depends on: 0261

## Outcome

Generated help and the MCP tool description cover `search cell-line`, `get cell-line` and `cell-line assay-count`, with their sections, source-ID input and paging options, and examples taken from case run steps. Help states that an assay count is not an activity or sensitivity count, that no treatment response is inferred and that no bioinformatics tool runs. The 0.9 parity ledger has a row for every cell line command, option and section in cell-line.md's accounting, naming the 1.0 spelling, the deferral or the retirement. Every cell line case asserts a measured `max_bytes` from its CLI and MCP output under the existing headroom rule, and cell-line.md's budget table replaces each offline projection with the measured envelope. Each question C01 to C24 keeps its answered, partial or unanswered status, with C18 to C21 recorded as deferred with PharmacoDB. The landing commit gives ROADMAP.md's cell line block the checklist that the protein block carries, moves PharmacoDB to a deferred line with its reason, and ticks the boxes this work completes.

- Why: The slice delivers docs, cases with ceilings and full 0.9 accounting, as `planning/slices.md` says.
- Starts from: 0244 and 0228 as the pattern; cell-line.md's case map, measured budget, question bank and legacy accounting, with the cell line rulings; the generated help and MCP description code; ROADMAP.md's cell line block.
- Rules: PharmacoDB, the drug-response routes, the HPA gene cell-lines route and Cellosaurus and ChEMBL bundles stay recorded as deferred with their reasons. Name normalization and reranking move to the dropped line with the ruling's reason. 0.9 timings and output sizes stay marked unmeasured, and the milestone 5 comparison measures them. A measured size over its starting ceiling is fixed or raised to the PM before landing. An entity help or MCP description cap increase needs a PM ruling.
- Proof: a help case asserts each cell line command lists its sections, options and two parsed examples; an MCP `tools/list` case asserts the cell line tree within the existing description ceiling. A reviewer checks that every accounting row has a ledger row, every design case ID has a case file or an outside-repo case, and every case has a measured `max_bytes`. The landing commit ticks Biodata types and schema, Search, Get and sections, Outside-in cases, Token budget, MCP parity, Docs and 0.9 accounting, and the Cellosaurus and ChEMBL source boxes, each only after its check passes. This ticket adds no filter or exact-ID get and makes no provider call. No licensed content enters the repo. Then make landing-gate runs through pm job.
- Defers: the contained 0.9 cell line comparison, which the milestone 5 comparison measures.
- Changes: generated help and MCP description sources, the parity ledger, cases, `cell-line.md` and `ROADMAP.md`.

- PM ruling 2026-10-26 (m7478): help caps at cdee7c67 measured size plus 15 percent: `search cell-line` 1,012 bytes CLI, 1,011 MCP and 984 Markdown; `cell-line assay-count` 727 CLI, 726 MCP and 785 Markdown. `get cell-line` caps stay 1,168, 1,167 and 1,081. Global caps stay unchanged, and a measurement over a global cap returns to the PM.
- PM ruling 2026-10-26 (m7478 ask 3): the full compact MCP `tools/list` reply cap, checked without the trailing newline, rises from 7,916 to 9,137 bytes, the 7,945 bytes measured at cdee7c67 plus 15 percent, keeping the required cell line guidance. The description cap stays 3,563 and the whole help cap stays 91,228.
