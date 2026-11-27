# 1192 — mcp-list-cache-fields

Status: OPEN.

Milestone: 7

## Outcome

Under MCP revision 2026-07-28, every `server/discover` and `tools/list` result carries `ttlMs` (a positive integer) and `cacheScope`. `cacheScope` is `"private"` in hosted mode and `"public"` otherwise. The newest official TypeScript client (2.4.0) and Python SDK (2.3.0) then list BioMCP's tools over stdio and HTTP.

## Why

Blocks 1.0. Those clients reject a 2026-07-28 `tools/list` result that lacks these fields, so apps built on them, including Cherry Studio, see no tools. Published 0.9.3 sent both fields.

## Starts from

- Starts from: `crates/biomcp/src/mcp/current.rs` `discover`, `crates/biomcp/src/mcp/protocol.rs` `discovery_result`, `crates/biomcp/src/mcp/discovery.rs`, `crates/biomcp/src/mcp/hosted.rs`; the 2026-07-28 specification's definitions of `ttlMs` and `cacheScope` (https://modelcontextprotocol.io/specification/2026-07-28).

## Decisions

- PM decision (2026-11-27): `ttlMs` is one named constant chosen from how long a catalog stays valid within one server process (the catalog changes only on restart or configuration change); name the reason beside it.
- PM decision (2026-11-27): older revisions get no new fields.
- PM decision (2026-11-27): the typed catalog ceiling of 196,608 bytes is never raised. After 1197 it has about 101 bytes to spare. If this change pushes any measured catalog over its ceiling, trim catalog text instead, for example tool titles that only repeat the tool name; if that can't be done without losing meaning, stop and report the sizes.

## Keeps

- Keeps: tool lists and replies under every earlier revision.

## Proof

- Proof: stdio and HTTP cases under 2026-07-28 assert both fields on `server/discover` and `tools/list`, with `"private"` under `--hosted` and `"public"` otherwise; a case under 2025-11-25 asserts neither appears; `make landing-gate`.

## Defers

- Defers: the standing client interop check is 1193.

## Changes

- Changes: `crates/biomcp/src/mcp/`, `tests/cases/`.
