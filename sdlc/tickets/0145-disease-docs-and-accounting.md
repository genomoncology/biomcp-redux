# 0145 — Document the disease commands and close the disease accounting

Status: OPEN.

Milestone: 3

Depends on: 0144

## Outcome

Generated help and the MCP tool description cover `search disease`, the SEER site catalog search, and `get disease` with its sections, sources, ontology selector, SEER selectors and paging options, with examples taken from case run steps. The 0.9 parity ledger has a row for every disease command, option and section in disease.md's accounting, naming the 1.0 spelling, the owning slice or the retirement. Every disease case asserts a measured `max_bytes` from its CLI and MCP output under the existing headroom rule, and disease.md's budget table replaces each unmeasured product row with the measured size. The landing commit ticks the disease roadmap boxes this work completes.

- Why: the slice delivers docs, cases with ceilings and full 0.9 accounting, as `planning/slices.md` says.
- Starts from: 0135's drug docs ticket as the pattern; disease.md's case map, budget table and 0.9 accounting; the generated help and MCP description code; ROADMAP.md's disease block.
- Rules: `search trial --condition-id` and the old `disease trials` options are recorded as owned by trial ticket 0137, with no disease case added for them. A measured size over its design allocation is fixed or raised to the PM before landing. The Open Targets disease-genes section stays deferred, and its roadmap box stays open with that reason. Direct HPO, CIViC disease context, Reactome, UMLS and MedlinePlus move to the dropped line with disease.md's reasons, beside DisGeNET and NIH RePORTER. The contained 0.9 comparison belongs to 0136, so the token budget box waits on 0136.
- Proof: a help case asserts each disease command lists its sections, options and two parsed examples; an MCP `tools/list` case asserts the disease tree within the existing description ceiling. A reviewer checks that every accounting row has a ledger row, every design case ID has a case file or an outside-repo case, and every case has a measured `max_bytes`. The landing commit ticks Biodata types and schema, Search, Get and sections, Outside-in cases, MCP parity, Docs and 0.9 accounting, and the MyDisease.info, Monarch, OLS and SEER source boxes, each only after its check passes. This ticket adds no filter or exact-ID get and makes no provider call; the disease filters and exact-ID gets keep their known-positive captures from one bounded keyless pm job each. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
- Defers: the contained 0.9 comparison and the token budget tick to 0136.
