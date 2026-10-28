# 0358 — Give each MCP tool a compact outputSchema that names its reply schemas

Status: COMPLETE.
Landed: 7d3b5f4b031071640b44d0d070f4afef8c3bb9f2

Milestone: 5

Depends on: 0341

## Outcome

Each MCP tool's `outputSchema` describes the reply envelope compactly: its `schema` property lists, as `const` or `enum`, the reply schema IDs the tool can return, and its other envelope fields are typed, instead of the bare `{"type":"object"}`. Full canonical reply schemas stay available on demand through `biomcp schema <id>`, over the command line and MCP.

- Why: the docs lead's product findings report (2026-10-27), item C2. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: `crates/biomcp/src/mcp/protocol.rs`, the schema registry and 0341's per-command schema IDs.
- Rules: PM ruling 2026-10-27 (m10775): fully inlined schemas measured 2,253,055 bytes for tools/list, which would load into client model context every session, so they are refused. tools/list stays within its 9,137-byte cap, or at most 16,000 bytes by a measured caps amendment. Update the architecture's MCP tool list line to say the tool list names schemas and `biomcp schema` serves them.
- Proof: an MCP case validates real tools/call structured results against their tool's `outputSchema`; every schema ID the tool returns in recorded cases appears in its `outputSchema`; each listed ID resolves with `biomcp schema`; a check fails if tools/list exceeds its cap.
- Changes: MCP tool listing, schema references and cases.
- Defers: nothing.
