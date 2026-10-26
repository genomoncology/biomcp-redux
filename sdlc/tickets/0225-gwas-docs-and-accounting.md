# 0225 — Document the GWAS commands and close the GWAS accounting

Status: OPEN.

Milestone: 4

Depends on: 0237

## Outcome

Generated help and the MCP tool description cover `search gwas` and `get gwas` with every selector, `--include-child-traits`, `--extended-geneset`, `--p-value`, the default order, paging options and the `loci`, `associations` and `ancestries` sections, with examples taken from case run steps. Help states that the commands report source associations and never calculate personal risk, infer a causal variant or gene, or interpret a statistic clinically, and that `--p-value` requires the default p-value-ascending order. The 0.9 parity ledger has a row for every GWAS command, option and section in gwas.md's legacy accounting, naming the 1.0 spelling, the deferral or the retirement, including the retired local prefix intersection and the kept positional, `-g` and `-l` aliases. Every GWAS case asserts a measured `max_bytes` from its CLI and MCP output under the existing headroom rule, and gwas.md's budget table replaces each projection with the measured envelope. Each question Q01 to Q25 keeps its answered, partial or unanswered status, updated only where a case now answers it. The landing commit gives ROADMAP.md's GWAS block the checklist that the pharmacogenomics block carries and ticks the boxes this work completes.

- Why: the slice delivers docs, cases with ceilings and full 0.9 accounting, as `planning/slices.md` says.
- Starts from: 0169 as the pattern; gwas.md's case map, measured disclosure costs, question bank and legacy surface; the generated help and MCP description code; ROADMAP.md's GWAS block.
- Rules: the four release bundles, `sync gwas`, summary-statistics files, interval search, a GWAS batch command and other GWAS sources stay recorded as deferred or dropped with the design's reason. Issue 283 keeps its shared clean JSON-RPC stream case, which gains a GWAS call that writes cache; issue 78's enrichment analysis stays outside scope. A measured size over its projection ceiling is fixed or raised to the PM before landing. The token budget box ticks when every row has its measured 1.0 size and either a qualified 0.9 figure from 0098 or a stated reason that 0.9 could not be measured.
- Proof: a help case asserts each GWAS command lists its selectors, sections, options and two parsed examples; an MCP `tools/list` case asserts the GWAS tree within the existing description ceiling, and the command-tree help and schema source list ceilings move to measured size plus 15 percent under the slice rule. A reviewer checks that every accounting row has a ledger row, every design case ID has a case file or an outside-repo case, and every case has a measured `max_bytes`. The landing commit ticks Biodata types and schema, Search, Get and sections, Outside-in cases, Token budget, MCP parity, Docs and 0.9 accounting, and the GWAS Catalog source box, each only after its check passes, and ticks Slice complete only when every other box is ticked. This ticket adds no filter or exact-ID get and makes no provider call; the GWAS filters and exact-ID gets keep their known-positive captures from one bounded keyless pm job each. Source replies stay outside the repo. Then make landing-gate runs through pm job.
- Defers: the contained 0.9 GWAS comparison, which 0098 measures after the adapters land with verified containment.
