# 0169 — Document the pharmacogenomics commands and close the pharmacogenomics accounting

Status: COMPLETE.
Landed: 07a09ab2185a9ac10a2798ec6d47d33d2fc7c836

Milestone: 4

Depends on: 0168

## Outcome

Generated help and the MCP tool description cover `search pgx` and `get pgx` with their required sources, sections, `--diplotype`, `--kind`, `--cpic-level` and paging options, with examples taken from case run steps. Help states that no phenotype, phase or dose is calculated. The 0.9 parity ledger has a row for every pharmacogenomics command, option and section in pharmacogenomics.md's accounting, naming the 1.0 spelling, the deferral or the retirement. Every pharmacogenomics case asserts a measured `max_bytes` from its CLI and MCP output under the existing headroom rule, and pharmacogenomics.md's budget table replaces each offline projection with the measured envelope. Each question Q01 to Q33 keeps its answered, partial or unanswered status, updated only where a capture now answers it. The landing commit gives ROADMAP.md's pharmacogenomics block the checklist that the drug and disease blocks carry, renames the PharmGKB box to ClinPGx, and ticks the boxes this work completes.

- Why: the slice delivers docs, cases with ceilings and full 0.9 accounting, as `planning/slices.md` says.
- Starts from: 0135 and 0145 as the pattern; pharmacogenomics.md's case map, offline projections, question coverage table and 0.9 accounting; the generated help and MCP description code; ROADMAP.md's pharmacogenomics block.
- Rules: frequency pages, the exact pair card, `--pgx-testing`, `--population`, `--lookup-key` and any unproven `--drug-id` stay recorded as deferred with their reason. PharmCAT, PharmVar downloads and DPWG scraping move to the dropped line with the design's reason. A measured size over its projection ceiling is fixed or raised to the PM before landing. The token budget box ticks when every row has its measured 1.0 size and either a qualified 0.9 figure from 0098 or a stated reason that 0.9 could not be measured.
- Proof: a help case asserts each pharmacogenomics command lists its sources, sections, options and two parsed examples; an MCP `tools/list` case asserts the pharmacogenomics tree within the existing description ceiling. A reviewer checks that every accounting row has a ledger row, every design case ID has a case file or an outside-repo case, and every case has a measured `max_bytes`. The landing commit ticks Biodata types and schema, Search, Get and sections, Outside-in cases, MCP parity, Docs and 0.9 accounting, and the CPIC and ClinPGx source boxes, each only after its check passes. This ticket adds no filter or exact-ID get and makes no provider call; the pharmacogenomics filters and exact-ID gets keep their known-positive captures from one bounded keyless pm job each. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
- Defers: the contained 0.9 pharmacogenomics comparison, which 0098 measures after the adapters land with verified containment.
- PM ruling 2026-10-25: the search pgx request schema narrows its source to CPIC inside this ticket, with the shared request and query version changes the compatibility check requires. The schema must not advertise ClinPGx for a search the runtime refuses. No help-only choices override and no second source list. If a parallel lane lands a version change first, merge onto it and take the next version. Help keeps the explicit CPIC-only wording.
