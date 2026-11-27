# 1197 — mcp-tool-titles-hints

Milestone: 7

## Outcome

Every advertised tool and the server's `serverInfo` carry a human `title`, and every read-only tool's annotations carry `idempotentHint: true`, under the revisions that define those fields.

## Why

Clients show `title` in their tool menus, and they use `idempotentHint` to decide whether a call can be retried safely.

## Starts from

- Starts from: tool definitions in `crates/biomcp/src/mcp/protocol.rs`, `research.rs`, `discovery.rs`, `typed/grouping.rs`, `typed/help.rs` and `typed/saved.rs`; `protocol::info`.

## Decisions

- PM decision (2026-11-27): titles are short plain names ("Gene", "Search BioMCP"); the saved tool is not read-only and gets no `idempotentHint`.
- PM decision (2026-11-27): fields go only where the negotiated revision defines them; 1191 leaves annotations out under 2024-11-05.
- PM decision (2026-11-27): the typed catalog has a size ceiling. Never raise it; if titles push past it, shorten other text.

## Proof

- Proof: cases asserting `title` on every tool and on `serverInfo`, and `idempotentHint` on every read-only tool; the catalog stays within its ceiling; `make landing-gate`.

## Changes

- Changes: `crates/biomcp/src/mcp/`, `tests/cases/`.
