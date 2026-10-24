# biodata is the base data layer

Decided 2026-10-23 by Ian.

- This repo has a `biodata` crate. It holds the typed values for every entity BioMCP serves: genes, variants, articles, trials, drugs, diseases and the rest, with their identifiers, closed sets, query types, response types and schemas.
- `biodata` has no command of its own. The `biomcp` binary is its only command.
- `biodata` does no I/O, and its `clippy.toml` enforces that. Adapters map source replies into its values. It never fetches, caches or ranks.
- It names no other project and depends on no other crate in this workspace, because it may later merge with another biodata crate into one open-source crate.
- Its rules are in `planning/rust-standards.md`.
