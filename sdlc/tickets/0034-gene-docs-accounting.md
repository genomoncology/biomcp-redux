# 0034 — Document the gene commands and close the gene slice

Status: OPEN.

Milestone: 1

## Outcome

`biomcp help`, `biomcp help get gene` and `biomcp help search gene` print JSON help generated from the schemas with sections and examples taken from cases, the MCP tool description carries the same generated command tree, gene.md records the 1.0 answer for every 0.9 gene command, option and section, and ROADMAP.md shows the gene slice complete.

Depends on: 0027
Depends on: 0028
Depends on: 0029
Depends on: 0030
Depends on: 0031
Depends on: 0032
Depends on: 0033

- Starts from: architecture 3.7 (doc comments become help), 8.7 (the tool description's compact tree, pinned under 1,500 bytes), 8.8 and 10.2; the embedded schemas in schema.rs; mcp/protocol.rs, whose tool description is a hand-written string today; 0032's `help key`; 0033's `retired.rs` table; gene.md sections 1, 5 and 6; ROADMAP.md gene block; decision `2026-10-24-the-roadmap-tracks-progress-as-checklists.md`.
- Changes: `crates/biomcp/src/help.rs`, `crates/biomcp/src/command.rs`, `crates/biomcp/src/grammar.rs`, `crates/biomcp/src/mcp/protocol.rs`, `crates/biomcp/src/schema.rs`, `crates/biodata/src/query/gene.rs`, `crates/biodata/schemas/**`, `crates/biomcp/tests/runner/mcp.rs`, `tests/cases/gene/help.yaml`, `sdlc/planning/slices/gene.md`, `ROADMAP.md`.
- PM decision: 2026-10-24. Gene docs are the generated help and the MCP tree, because architecture 8.8 makes help the reference and milestone 3 owns the generated reference site and the hand-written guide. Help reads the checked-in schemas: each command's description, its arguments with types, bounds and positional order, the section words with one line each, the `--section` rule for inputs that equal a section word, and two examples per command taken verbatim from case `run` steps. `biomcp help` lists entities, verbs and housekeeping commands. The MCP description is generated from the same data, one line per verb with its sections and main flags. Section and flag descriptions come from doc comments on the query types, so missing comments are added there.
- PM decision: 2026-10-24. The 0.9 accounting closes in gene.md: section 1's command table gains a "1.0" column naming, for every command, option and section, the 1.0 spelling, the slice it moved to, or the drop with its reason from section 5, and the retired message from 0033. Section 6's token table gains a "1.0 shipped" column measured once from the cases' outputs by the runner's bytes-divided-by-four rule, with no new tool. Any shipped size over its section 6 budget is fixed or raised to the PM before landing.
- PM decision: 2026-10-24. ROADMAP.md's gene sources list follows the rulings Ian approved in gene.md section 5: GTEx moves to the dropped line, GenCC (bundle) to a deferred line, and COSMIC Cancer Gene Census (keyed) stays unticked with "awaits a ruling", because gene.md section 7 only recommends it. The landing commit ticks Biodata types and schema, Get and sections, Outside-in cases, Token budget, MCP parity, Docs and 0.9 accounting, each only after its check below passes.
- Keeps: Command output, grammar and every existing case. No Markdown view, no `--all` surface check, no generated site.
- Proof: `tests/cases/gene/help.yaml` asserts `help get gene` lists all seven section words and `all`, the `--section` rule and two examples; that `help search gene` lists `--band` and `--locus-group`; that each printed example parses (follow it in a later step over the gene replies); and that `help key oncokb` still answers. An MCP `tools/list` case asserts the description names every gene section and stays under 1,500 bytes. A reviewer checks that every row of gene.md section 1 has a 1.0 entry and every section 8 case has a case file under `tests/cases/gene/` (case 20 excepted, outside the repo), and that every section 6 budget has a ceiling in a case. Then make landing-gate runs through pm job after current main.
- Defers: `help --all`, the surface check of 10.2 and the generated reference site go to milestone 3. COSMIC waits for Ian's ruling, sought in gene.md section 7.
