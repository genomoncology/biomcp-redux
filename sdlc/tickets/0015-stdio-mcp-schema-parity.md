# 0015 — Serve the existing commands through one stdio MCP tool

Status: OPEN.

Milestone: 1

## Outcome

BioMCP serves one stdio MCP tool that runs the existing schema/error command dispatcher, and real cases prove CLI and MCP return equal JSON results.

Depends on: 0013
Depends on: 0012

- Starts from: `sdlc/plan.md` stages 5 and 6; architecture sections 8.7 and 10.1–10.2; the official stable MCP specification when this ticket starts; current binary dispatcher/output boundary and YAML runner.
- Changes: `crates/biomcp/src/main.rs`, `crates/biomcp/src/config.rs`, `crates/biomcp/src/mcp.rs`, `crates/biomcp/src/command.rs`, `crates/biomcp/src/schema.rs`, `crates/biomcp/tests/runner/**`, `crates/biomcp/tests/cases.rs`, `tests/cases/schema/**`, `tests/cases/mcp/**`, `crates/biomcp/tests/ratchets/source.rs`.
- PM decision: 2026-10-24. Implement stdio MCP for existing schema commands and usage errors with the same dispatcher, pinned current stable spec revisions and parity cases. outputSchema covers only today's JSON object results; widen it with gene envelopes. No transport flags before a real transport command.
- Scope: Add biomcp serve with newline-delimited JSON-RPC. Confirm stable revisions and normative behavior from official MCP sources, keep supported revisions in one declared list and record the pin rationale in the commit. Implement initialize negotiation, silent valid notifications, ping, tools/list and tools/call; arrays are invalid requests and unknown methods are Method Not Found. The single biomcp tool takes the CLI args list and calls the existing dispatcher. Return the same parsed JSON as structuredContent and one text block, with isError true for the existing usage/local failures. Describe only supported commands and derive schema discovery from existing artifacts. Never route serve recursively through its own tool.
- Runner: Add the architecture's mcp key using the built server process. A step containing run and mcp compares their JSON results, and error cases distinguish CLI exit status from MCP isError. Keep one shared assertion/ceiling path. Cases exercise protocol revisions, notification silence, batching refusal, unknown method/tool, malformed params, success and usage parity. Run tools/call requests concurrently through bounded standard threads with one synchronized writer, preserving request IDs and no shared per-call mutable command state.
- Keeps: No MCP dependency or async runtime. Bound inbound message framing and worker admission in named code configuration, reject malformed messages with fixed diagnostics and avoid raw input in errors. Do not block input/output progress behind a slow tool call or mix interleaved responses. Keep current schema output and typed usage envelope; add a new public error variant only if an actual new path needs it. No replay/MCP-global flag promises, provider calls or unimplemented command grammar.
- Ownership: Record/replay lane 0014 owns root manifests/lock. This ticket adds no dependency and owns binary/runner files. Correct narrow Claims if a shared dispatcher helper is needed; do not edit transport files.
- Proof: Actual built-server cases prove initialization fallback, all supported revision negotiations, one tool, fixed input schema, exact schema/error parity, no reply to notifications and complete correlated concurrent responses. Exercise malformed/oversized input without echoing a distinctive payload. Use events/EOF/protocol replies rather than tight wall-clock success ceilings. Retain existing runner negative/filter/isolation and broken-output cases. Run make landing-gate through pm job after current main.
- Defers: Entity request grammar, normal envelopes, replay/server flags and live-data MCP cases arrive with their real consumers. The one-tool roadmap item can be ticked only when the accepted actual tool/CLI grammar contract is satisfied; no broad output/runner item is completed by this ticket alone.
