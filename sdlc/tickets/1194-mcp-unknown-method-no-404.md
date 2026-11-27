# 1194 — mcp-unknown-method-no-404

Status: COMPLETE.
Landed: 9d728ab0dfddf6f2285382149292b9d9a1ea13c2

Milestone: 7

## Outcome

Over HTTP, a request for an unknown method under a revision before 2026-07-28 gets a JSON-RPC error -32601 with HTTP 200, not HTTP 404. Under 2026-07-28, it keeps the status that revision requires.

## Why

Older clients, including the official Python SDK, read an HTTP 404 as the session ending, so one unknown method drops the connection.

## Starts from

- Starts from: `crates/biomcp/src/mcp/http.rs`, where the unknown-method reply returns 404; the test in `crates/biomcp/tests/runner/http.rs` that asserts "unsupported modern method needs HTTP 404 and RPC -32601".

## Keeps

- Keeps: the 404 for an unknown path, and the 2026-07-28 behavior.

## Proof

- Proof: HTTP cases asserting a 200 status and -32601 for an unknown method under 2025-11-25 and 2025-03-26, while the 2026-07-28 case keeps its status; `make landing-gate`.

## Changes

- Changes: `crates/biomcp/src/mcp/http.rs`, `crates/biomcp/tests/runner/http.rs`.
