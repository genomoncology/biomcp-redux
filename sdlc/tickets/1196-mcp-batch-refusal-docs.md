# 1196 — mcp-batch-refusal-docs

Milestone: 7

## Outcome

The MCP docs say in one line that JSON-RPC batches, which 2025-03-26 allows, are refused, and how the refusal reads.

## Why

A 2025-03-26 client author has no other way to learn that batches are not supported. No code changes.

## Starts from

- Starts from: the batch refusal in `crates/biomcp/src/mcp/`, read to quote it exactly; the MCP section of `docs/configuration.md`.

## Proof

- Proof: `make docs-check`.

## Changes

- Changes: `docs/configuration.md`.
