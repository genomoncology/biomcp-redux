# 0047 — Document the article commands and close the article accounting

Status: OPEN.

Milestone: 1

## Outcome

`biomcp help` and `biomcp help` for `search article`, `get article`, `article text`, `article supplement`, `article citations`, `article references`, `article citation-evidence` and `article recommendations` print JSON help generated from the schemas with examples taken from cases, the MCP tool description carries the article commands within its ceiling, article.md records the 1.0 answer for every 0.9 article command, option, section and source, and ROADMAP.md ticks each article box whose check passes.

Depends on: 0040
Depends on: 0041
Depends on: 0042
Depends on: 0044
Depends on: 0045
Depends on: 0046
Depends on: 0048
Depends on: 0049
Depends on: 0050
Depends on: 0051
Depends on: 0052
Depends on: 0053

- Starts from: 0034's generated help, `help.rs`, `build.rs` example embedding and the compact MCP tree under 1,500 bytes; architecture 3.7, 8.7 and 8.8; article.md sections 1, 5 (keep, change or drop, sources kept and dropped), 6 (token table and budget), 8 and 9; 0046's retired table; ROADMAP.md article block; decision `2026-10-24-the-roadmap-tracks-progress-as-checklists.md`.
- Changes: `crates/biomcp/src/help.rs`, `crates/biomcp/build.rs`, `crates/biomcp/src/mcp/protocol.rs`, `crates/biodata/src/query/article.rs`, `crates/biodata/schemas/**`, `crates/biomcp/tests/runner/protocol.rs`, `tests/cases/article/help.yaml`, `tests/cases/article/**`, `sdlc/planning/slices/article.md`, `ROADMAP.md`.
- PM decision: 2026-10-24. Article docs follow 0034: help reads the checked-in schemas, each command's description and arguments come from doc comments on the query types, `--source` and `--sort` carry article.md's short source table for the sources that landed, and two examples per command come verbatim from case `run` steps. The MCP description adds one line per article verb with its sections and main flags; if the 1,500-byte ceiling binds, the PM decides between shortening lines and raising the ceiling before landing.
- PM decision: 2026-10-24. article.md section 1's command table gains a "1.0" column naming, for every 0.9 command, option and section, the 1.0 spelling, the slice it moved to, the drop with its reason from section 5, or the retired message from 0046. The rows for `--pdf`, `tldr`, `indexing`, citation contexts, `article citation-evidence`, `article recommendations`, `--open-access`, `--no-preprints`, `--exclude-retracted` and `--include-retracted` name the 1.0 spelling that keeps each, as `decisions/2026-10-24-article-keeps-every-source-one-option-away.md` rules; `--ranking-mode` and the `--weight-*` flags read "retired; maps to no blend", as `decisions/2026-10-24-no-text-heuristics.md` rules. Section 6's token table gains a "1.0 shipped" column measured once from the cases' outputs by the runner's bytes-divided-by-four rule; any shipped size over its section 6 budget is fixed or raised to the PM before landing.
- PM decision: 2026-10-24, revised after Ian's ruling. ROADMAP.md's article sources follow article.md section 5: PubMed, Europe PMC, PubTator3, LitVar, LitSense2, Semantic Scholar (optional key), PMC Open Access and iCite ticked as each landed, and NCBI EFetch for PMC, the NCBI ID converter, OpenCitations and Figshare on a line that says another source keeps each capability. The landing commit ticks the remaining article boxes, including the AKT1 E17K cases, each only after its check below passes.
- Keeps: Command output, grammar and every existing case.
- Proof: `tests/cases/article/help.yaml` asserts that help for `get article` lists `fulltext`, `supplements`, `annotations`, `citations`, `indexing`, `tldr`, `pdf` and `all`, that help for `search article` lists `--source` with all six sources, each described by its job and failure modes without ranking, `--sort` with `rank`, `date` and `citations` and their costs, and the four access, preprint and retraction filters, that help for `article citations` and `article references` lists `--source icite|semantic-scholar`, and that each printed example parses by following it over the article replies. An MCP `tools/list` case asserts the description names every article verb and stays under its ceiling. A reviewer checks that every row of article.md section 1 has a 1.0 entry, that every section 8 case has a case file under `tests/cases/article/` or is listed as outside the repo, and that every section 6 budget has a ceiling in a case. Then make landing-gate runs through pm job after current main.
- Defers: Ian answered questions A, B and C in `decisions/2026-10-24-article-keeps-every-source-one-option-away.md`; 0040 holds the AKT1 E17K bar, and 0049 to 0053 build the kept sources and capabilities. `help --all`, the surface check of 10.2 and the generated reference site go to milestone 3.

- Reviewed close-out: Update stale article source/filter tables, optional-header row behavior, preprint-unavailable reason, retired short flags, clinical citation fields, per-exchange binary sidecars and reader grammar to the landed rulings. Every kept source and capability from Ian's ruling appears in the docs and the accounting. Installed examples use the existing build-time case embedding; no runtime repository reads or second handwritten command list.
- PM page-ceiling decision, 2026-10-24, m4341: Each actual recorded article page sets its case ceiling at measured output plus 15 percent, never above recorded 0.9 JSON tokens for the same question. The 1,300-token number is a target; provisional 7,000/8,000-byte page ceilings are too loose. Capture the resulting exact case values, and explain any case that cannot meet them on its ticket as slices.md requires. Retain honest unknowns for missing baseline/real-key evidence; synthetic outputs do not prove real licensed content budgets. Other non-page command bounds stay as approved unless separately ruled.
- Reviewed integration: After 0035, card retrieval 0036 and filters 0037 may proceed in separate worktrees with narrow private owners and additive shared declarations. One batch lander combines accepted query/schema registrations and regenerates schemas. Later section/next/query enum collisions use the same ownership and batching practice; do not pretend a broad shared claim is independent. No new general review stage, gate or report.
