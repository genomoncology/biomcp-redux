# 0427 — Parse and encode once per call

Status: OPEN.

Milestone: 5

## Outcome

Each CLI call builds the grammar once, reads the config once and serializes the reply once; the JSON value is built only for Markdown or MCP, and output writes from a borrowed buffer. MCP serializes each reply once.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`. Cost grows with the schema set, not the request.
- Starts from: `crates/biomcp/src/main.rs`, `command.rs`, `output.rs`, `grammar.rs`, `grammar/errors.rs`, `config.rs`, `mcp/protocol.rs`.
- Rules: no output change; measure before and after.
- Proof: existing cases pass; a measured offline replay query on the release build is faster, with both numbers in the ticket record.
- Changes: the files above.
- Defers: nothing.
