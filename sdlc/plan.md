# Plan: foundations, then the gene and article slices

This plan covers milestone 1. It builds the foundations every entity rests on, then the gene slice, then the article slice. It names stages, not tickets. Each stage files its own tickets when it starts, and each slice files its tickets from its investigation, as `decisions/2026-10-23-slices-not-preplanned-tickets.md` says. `planning/architecture.md` holds the design, `planning/rust-standards.md` holds the rules and ratchets, and `planning/slices.md` holds the slice method.

## Why this order

Foundations come first because every slice uses all of them. The crate layout fixes the dependency direction before any code can cross it. The lint ratchets go on while the code is empty, so no file or function ever exceeds its limit and no ratchet starts with a debt. The shared transport comes before any adapter, so no adapter ever builds its own client, retry or cache. The biodata conventions and schema generation come before the first entity type, so every type follows one pattern. The runner and the output contract come before the first command, so the first command is written outside-in against a schema.

Gene comes first among the slices because it is the simplest entity that still runs the whole stack. A gene has stable identifiers, few sources and small replies. The slice proves the types, the schema, the transport, the adapter shape, the commands, the MCP tool and the harvested cases end to end, with little domain risk. Gene symbols and identifiers also feed every later slice, including article queries.

Articles come second because article search is where BioMCP 0.9 fails a clinician most visibly: a plain PubMed search finds papers that BioMCP drops. The article slice proves what gene cannot. It has many sources, paged and multi-step adapters, a merged and ranked pool, a stored snapshot for stable paging, and a get that returns supplementary material. It also tests the adapter size estimate in `planning/architecture.md` on the hardest sources first.

## Stages

### 1. Workspace and crate layout

- One Cargo workspace with the crates in `planning/architecture.md`. Crates that no stage needs yet do not exist.
- Cargo enforces the dependency direction. `biodata` carries the no-I/O `clippy.toml`.
- `make landing-gate` runs formatting, clippy with warnings denied, the ratchet check and the case suite.

### 2. Rust standards and lint ratchets

- The workspace lint table from `planning/rust-standards.md`.
- The file length limits, `clippy::too_many_lines` at 80, and the ratchet check that fails when a crate names a forbidden dependency or type.
- Each ratchet starts at its 1.0 target, because the code starts empty.

### 3. The shared transport with record and replay

- `biomcp-fetch` with endpoint declarations, deadline, rate limit, the raw reply cache in SQLite, retry with `Retry-After`, body limits and one `SourceError` enum.
- Record mode writes each reply to a recording directory. Replay mode serves only recordings and fails on a miss.
- Recordings can hold a status code, a delay and a broken body, so failures are ordinary cases.
- A bundle endpoint kind for local data bundles, built when the first bundle source needs it.

### 4. biodata core conventions and schema generation

- The ID newtype pattern, closed-set enums, coordinate types, the error pattern and the serde rules from `planning/rust-standards.md`.
- JSON Schema generated from the types behind a `schema` feature, checked in, versioned, and failed on drift.
- The shared envelope types: status, section states, problems and typed next steps.

### 5. The outside-in runner

- One integration test target that turns each YAML case into a named test.
- It runs the built binary over recordings, checks JSON pointer assertions and prints a readable diff.
- The assertion set: equality, subset, includes, count range, rank range, uniqueness and regular expression.
- An `mcp:` key sends the same request through the MCP tool, so a case proves CLI and MCP agree.

### 6. JSON output and the schema contract

- Every command prints compact JSON in the envelope by default.
- The runner validates every output against its schema.
- `biomcp schema <name>` prints a checked-in schema.

### 7. The gene slice

Runs the method in `planning/slices.md`:

1. Investigation: real gene questions harvested from published papers and real questions, how each source answers them, and a critical look at every 0.9 gene command, section and option.
2. The five to ten delight questions, answered in `planning/gene.md`.
3. Types and schema: the gene identifiers, the gene header and the gene card.
4. Adapters: only the sources the investigation shows the answers need.
5. Commands: `search gene`, `get gene` and the drill-downs the investigation names, through the CLI and the one MCP tool.
6. Cases: harvested stories with answers verified against the live sources, recorded for replay.

The PM files the gene tickets from step 2's findings.

### 8. The article slice

Runs the same method for articles. Its known bars, from 0.9's failures:

- Every PMID that a plain PubMed relevance search returns appears in the full walk, or the response says which source was partial and why. AKT1 E17K is the standing case, with its recorded floor and its two key papers, 17611497 and 33123537, on page one.
- Two full walks give the same order, with no duplicates or gaps.
- A warm page makes no network request.
- Article get returns supplementary material where the source offers it.
- A kernel of about 20 cancer variant questions, harvested from published papers and real questions, sets the pass mark for ranking.

The entity store tables arrive in this slice, because the article pool is the first thing that must be stored.

## Running beside each other

- Stages 1 and 2 land together, because the gate needs both.
- Stages 3, 4 and 5 run side by side after stage 2. Stage 6 follows 4 and 5.
- The gene investigation may start any time, because it writes no product code. Gene code waits for stage 6.
- The article investigation may start during the gene slice. Article code waits for the gene slice to land, so it reuses what gene proved.
- Every ticket lands green with its ratchets on.
