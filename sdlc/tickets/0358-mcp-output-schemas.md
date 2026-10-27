# 0358 — Publish real reply schemas as MCP outputSchema

Status: OPEN.

Milestone: 5

Depends on: 0341

## Outcome

Each MCP tool's `outputSchema` is the generated reply schema for that command, by value or by `$ref`, instead of `{"type":"object"}`, as `sdlc/planning/architecture.md` says.

- Why: the docs lead's product findings report (2026-10-27), item C2. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: `crates/biomcp/src/mcp/protocol.rs`, the schema registry and 0341's per-command schema IDs.
- Rules: the tools list stays within its measured size cap, or a caps amendment records the new measure.
- Proof: an MCP case validates a real tools/call structured result against its tool's `outputSchema`; a check fails if any tool's `outputSchema` is the bare object schema.
- Changes: MCP tool listing, schema references and cases.
- Defers: nothing.
