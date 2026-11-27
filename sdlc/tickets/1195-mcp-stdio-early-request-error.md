# 1195 — mcp-stdio-early-request-error

Milestone: 7

## Outcome

Over stdio, a `ping` or any other request sent before `initialize` gets a JSON-RPC error whose message says the server is not initialized yet and the client must send `initialize` first, instead of a bare "Invalid params".

## Why

A bare "Invalid params" hides the cause, so a user or client author can't tell what went wrong.

## Starts from

- Starts from: `crates/biomcp/src/mcp/rpc.rs`, `crates/biomcp/src/mcp/legacy.rs`, `crates/biomcp/src/mcp/current.rs`, wherever a pre-initialize request is answered.

## Decisions

- PM decision (2026-11-27): keep the existing JSON-RPC error code unless the revision's lifecycle rules name another; only the message and its `data` change.

## Proof

- Proof: stdio cases sending `ping` and `tools/list` before `initialize` assert the new message; `make landing-gate`.

## Changes

- Changes: `crates/biomcp/src/mcp/rpc.rs`, `tests/cases/`.
