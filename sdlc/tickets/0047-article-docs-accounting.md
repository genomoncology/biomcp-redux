# 0047 — Document the article commands and close the article accounting

Status: OPEN.

Milestone: 1

## Outcome

`biomcp help` and `biomcp help` for `search article`, `get article`, `article text`, `article supplement`, `article citations` and `article references` print JSON help generated from the schemas with examples taken from cases, the MCP tool description carries the article commands within its ceiling, article.md records the 1.0 answer for every 0.9 article command, option and section, and ROADMAP.md ticks each article box whose check passes while the boxes that wait on Ian's open questions stay unticked.

Depends on: 0040
Depends on: 0041
Depends on: 0042
Depends on: 0044
Depends on: 0045
Depends on: 0046

- Starts from: 0034's generated help, `help.rs`, `build.rs` example embedding and the compact MCP tree under 1,500 bytes; architecture 3.7, 8.7 and 8.8; article.md sections 1, 5 (keep, change or drop, sources kept and dropped), 6 (token table and budget), 8 and 9; 0046's retired table; ROADMAP.md article block; decision `2026-10-24-the-roadmap-tracks-progress-as-checklists.md`.
- Changes: `crates/biomcp/src/help.rs`, `crates/biomcp/build.rs`, `crates/biomcp/src/mcp/protocol.rs`, `crates/biodata/src/query/article.rs`, `crates/biodata/schemas/**`, `crates/biomcp/tests/runner/protocol.rs`, `tests/cases/article/help.yaml`, `tests/cases/article/**`, `sdlc/planning/slices/article.md`, `ROADMAP.md`.
- PM decision: 2026-10-24. Article docs follow 0034: help reads the checked-in schemas, each command's description and arguments come from doc comments on the query types, `--source` and `--sort` carry article.md's short source table for the sources that landed, and two examples per command come verbatim from case `run` steps. The MCP description adds one line per article verb with its sections and main flags; if the 1,500-byte ceiling binds, the PM decides between shortening lines and raising the ceiling before landing.
- PM decision: 2026-10-24. article.md section 1's command table gains a "1.0" column naming, for every 0.9 command, option and section, the 1.0 spelling, the slice it moved to, the drop with its reason from section 5, or the retired message from 0046. The rows for `--pdf`, `tldr`, citation contexts, `article citation-evidence` and `article recommendations` read "awaits Ian's ruling, question C". Section 6's token table gains a "1.0 shipped" column measured once from the cases' outputs by the runner's bytes-divided-by-four rule; any shipped size over its section 6 budget is fixed or raised to the PM before landing.
- PM decision: 2026-10-24. ROADMAP.md's article sources follow article.md section 5: PubMed, Europe PMC, Semantic Scholar (keyed), PMC Open Access, LitVar and PubTator3 (export for annotations) ticked as each landed, a line for iCite ticked, NCBI EFetch for PMC, the NCBI ID converter, OpenCitations and Figshare moved to the dropped line, and LitSense2 and PubTator3 search left unticked with "awaits a ruling". The landing commit ticks Investigation, Biodata types and schema, Search with plain PubMed as the floor, Get with full text, supplementary material and citations, Token budget, MCP parity, Docs and 0.9 accounting, each only after its check below passes. "Questions answered by Ian" and "Outside-in cases, including AKT1 E17K" stay unticked until Ian rules on questions A, B and C.
- Keeps: Command output, grammar and every existing case.
- Proof: `tests/cases/article/help.yaml` asserts that help for `get article` lists `fulltext`, `supplements`, `annotations`, `citations` and `all`, that help for `search article` lists `--source` with each landed source and `--sort` with `rank`, `date` and `citations` and their costs, and that each printed example parses by following it over the article replies. An MCP `tools/list` case asserts the description names every article verb and stays under its ceiling. A reviewer checks that every row of article.md section 1 has a 1.0 entry, that every section 8 case has a case file under `tests/cases/article/` or is listed as outside the repo or waiting on question A or B, and that every section 6 budget has a ceiling in a case. Then make landing-gate runs through pm job after current main.
- Defers: Question A (whether milestone 1 requires both AKT1 E17K papers, 17611497 and 33123537, on the default page one, or each on page one of a strategy one step away) is a placeholder: case 28 and the standing AKT1 box wait for Ian's answer. Question B (LitSense2 and PubTator3 search as named sources) and question C (`--pdf`, `tldr`, citation contexts, recommendations) wait for Ian, and any tickets they need are filed after his answer. `help --all`, the surface check of 10.2 and the generated reference site go to milestone 3.
