# 0421 — Describe each command once

Status: OPEN.

Milestone: 5

Depends on: 0420

## Outcome

One table keyed by request variant holds each command's path words and query schema ID, and `query_schema`, `QUERIES`, the parse match, the spelling and the body pairing derive from it with no `_` fallback. Usage errors become a typed error, not response bodies. Exit codes come from `Status::code` everywhere.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`. Adding a command edits about eight hand-kept tables, and architecture 3.7 and 8.3 describe a mechanical mapping that does not exist.
- Starts from: `crates/biomcp-types/src/query/request.rs`, `crates/biomcp-query/src/run.rs`, `crates/biomcp/src/grammar*`, `spell.rs`, `command.rs`, the entity `encode` modules, `main.rs`, `exit.rs`, `build.rs`.
- Rules: keep the deliberate pgx and drug splits explicit; schemas stay compatible or take a major bump.
- Proof: a test fails when a command is missing from the table; existing cases pass.
- Changes: the files above and cases.
- Defers: nothing.
